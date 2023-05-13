use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};
use mdbook::MDBook;

use toml::value::{Table, Value};


pub struct ReadmeSummary;

impl ReadmeSummary {
    pub fn new() -> ReadmeSummary {
        ReadmeSummary
    }
}

impl Preprocessor for ReadmeSummary {
    fn name(&self) -> &str {
        "readme-summary"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book, Error> {
        // In testing we want to tell the preprocessor to blow up by setting a
        // particular config value
        if let Some(nop_cfg) = ctx.config.get_preprocessor(self.name()) {
            if nop_cfg.contains_key("blow-up") {
                anyhow::bail!("Boom!!1!");
            }
        }

        // generate read me link at src directory, not root
        generate_readme_links(&ctx.config.book.src)?;

        match MDBook::load(&ctx.root) {
            Ok(mdbook) => {
                return Ok(mdbook.book);
            }
            Err(e) => {
                panic!("{}",e);
            }
        };
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}

// fn generate_readme_links(directory: &Path) -> std::io::Result<String> {
//     let mut link_tree = String::new();

//     for child in directory.read_dir()? {
//         if let Ok(child_entry) = child {
//             if child_entry.file_type()?.is_dir() {
//                 let child_path = child_entry.path();
//                 let child_readme_path = child_path.join("README.md");

//                 if child_readme_path.exists() {
//                     let child_name = child_path.file_name().unwrap().to_str().unwrap();

//                     link_tree.push_str("- [");
//                     link_tree.push_str(child_name);
//                     link_tree.push_str("](");
//                     link_tree.push_str(child_readme_path.to_str().unwrap());
//                     link_tree.push_str(")\n");
//                 }
//             }
//         }
//     }

//     Ok(link_tree)
// }

fn is_readme(entry: &DirEntry) -> bool {
    let file_name = entry.file_name().to_string_lossy();
    let lowercase_name = file_name.to_lowercase();
    lowercase_name == "readme.md" || lowercase_name == "readme.html"
}

fn is_directory(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}

fn has_table_of_contents_marker(readme_path: &Path) -> std::io::Result<bool> {
    let mut file = fs::File::open(readme_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content.contains("{{TOC}}"))
}

fn generate_readme_links(src: &Path) -> std::io::Result<()> {
    for entry in WalkDir::new(src)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| is_directory(e))
    {
        let path = entry.path();
        let relative_path = path.strip_prefix(src).unwrap();

        // Skip the root directory
        if relative_path.components().count() == 0 {
            continue;
        }

        let readme_path = path.join("README.md");
        if !readme_path.exists() || !has_table_of_contents_marker(&readme_path)? {
            continue;
        }

        let mut link_tree = String::new();

        for child in path.read_dir()? {
            if let Ok(child_entry) = child {
                if child_entry.file_type()?.is_dir() {
                    let child_path = child_entry.path();
                    let child_readme_path = child_path.join("README.md");

                    if child_readme_path.exists() {
                        let child_relative_path = child_path.strip_prefix(src).unwrap();
                        let child_name = child_path.file_name().unwrap().to_str().unwrap();

                        link_tree.push_str("- [");  
                        link_tree.push_str(child_name);
                        link_tree.push_str("](./");
                        link_tree.push_str(child_relative_path.to_str().unwrap());
                        link_tree.push_str("/)\n");
                    }
                }
            }
        }

        if !link_tree.is_empty() {
            let mut content = fs::read_to_string(&readme_path)?;
            content = content.replace("{{TOC}}", &link_tree);
            fs::write(readme_path, content)?;
        }
    }

    Ok(())
}

