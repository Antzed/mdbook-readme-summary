use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

use std::path::{Path};

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

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        if let Some(nop_cfg) = ctx.config.get_preprocessor(self.name()) {
            if nop_cfg.contains_key("blow-up") {
                anyhow::bail!("Boom!!1!");
            }
        }

        fn process_item(item: &mut BookItem) {
            
            match item {
                BookItem::Chapter(chapter) => {
                    // Get the path of the chapter
                    if let Some(path) = chapter.path.clone() {
                        let path_str = path.to_string_lossy();
                        if path_str.ends_with("index.md") {
                            if has_toc(&chapter.content) {
                                let mut parent_path_str = "./src/".to_owned() + path.parent().unwrap().to_str().unwrap();
                                parent_path_str = parent_path_str + "/";
                                let parent_path = Path::new(&parent_path_str);
                                match generate_readme_links(&parent_path) {
                                    Ok(links) => {
                                        chapter.content = chapter.content.replace("{{TOC}}", &links);
                                    }
                                    Err(e) => {
                                        eprintln!("Error generating readme links for path {}: {}", parent_path.display(), e);
                                    }
                                }
                            }       
                        }                 
                    }
                }
                _ => {}
            }
        }
        book.for_each_mut(&mut process_item);


        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}


fn has_toc(chapter_content: &str) -> bool {
    chapter_content.contains("{{TOC}}")
}

fn generate_readme_links(directory: &Path) -> std::io::Result<String> {
    let mut link_tree = String::new();
    eprintln!("detect {{TOC}} in {}", directory.display());

    for child in directory.read_dir()? {
        if let Ok(child_entry) = child {
            if child_entry.file_name().to_str().unwrap() != "README.md" && !child_entry.file_name().to_str().unwrap().contains("draft") {
                if child_entry.file_type()?.is_dir() {
                    let child_path = child_entry.path();
                    let child_readme_path = child_path.join("README.md");
                    if child_readme_path.exists() {
                        let child_name = child_path.file_name().unwrap().to_str().unwrap();

                        link_tree.push_str("- [");
                        link_tree.push_str(child_name);
                        link_tree.push_str("](./");
                        link_tree.push_str(child_name);
                        link_tree.push_str("/");
                        link_tree.push_str(")\n");
                    }
                }else if child_entry.file_type()?.is_file() {
                    let child_path = child_entry.path();
                    let child_name = child_path.file_name().unwrap().to_str().unwrap();

                    link_tree.push_str("- [");
                    link_tree.push_str(child_name);
                    link_tree.push_str("](");
                    link_tree.push_str(child_path.to_str().unwrap());
                    link_tree.push_str(")\n");
                }
            } else {
                // eprintln!("child is not ok");
                continue;  
            }
        }
    }

    Ok(link_tree)
}