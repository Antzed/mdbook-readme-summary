use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

use std::path::{Path};

const ENABLE_DRAFT: &str = "enable-draft";
const ENABLE_LOG: &str = "enable-log";

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

        let mut use_enable_draft = false;
        let mut use_enable_log = false;

        if let Some(nop_cfg) = ctx.config.get_preprocessor(self.name()) {
            if nop_cfg.contains_key("blow-up") {
                anyhow::bail!("Boom!!1!");
            }
            if nop_cfg.contains_key(ENABLE_DRAFT) {
                let v = nop_cfg.get(ENABLE_DRAFT).unwrap();
                use_enable_draft = v.as_bool().unwrap_or(false);
            }
            if nop_cfg.contains_key(ENABLE_LOG) {
                let v = nop_cfg.get(ENABLE_LOG).unwrap();
                use_enable_log = v.as_bool().unwrap_or(false);
            }
        }

        fn process_item(item: &mut BookItem, use_enable_draft: bool, use_enable_log: bool) {
            
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
                                match generate_readme_links(&parent_path, use_enable_draft, use_enable_log) {
                                    Ok(links) => {
                                        chapter.content = chapter.content.replace("{{TOC}}", &links);                                    }
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
        book.for_each_mut(|item| process_item(item, use_enable_draft, use_enable_log));


        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}


fn has_toc(chapter_content: &str) -> bool {
    chapter_content.contains("{{TOC}}")
}

fn generate_readme_links(directory: &Path, use_enable_draft: bool, use_enable_log: bool) -> std::io::Result<String> {
    let mut link_tree = String::new();
    if use_enable_log {
        eprintln!("detect {{TOC}} in {}", directory.display());
    }
    for child in directory.read_dir()? {
        if let Ok(child_entry) = child {
            let should_ignore_draft = !use_enable_draft && child_entry.file_name().to_str().unwrap().contains("draft");
            if child_entry.file_name().to_str().unwrap() != "README.md" && !should_ignore_draft {
                if child_entry.file_type()?.is_dir() {
                    let child_path = child_entry.path();
                    let child_readme_path = child_path.join("README.md");
                    if child_readme_path.exists() {
                        let child_name = child_path.file_name().unwrap().to_str().unwrap();

                        link_tree.push_str("- [");
                        link_tree.push_str(child_name);
                        link_tree.push_str("/](./");
                        link_tree.push_str(child_name);
                        link_tree.push_str("/");
                        link_tree.push_str(")\n");
                    }
                }else if child_entry.file_type()?.is_file() {
                    let child_path = child_entry.path();
                    let child_name = child_path.file_name().unwrap().to_str().unwrap();
                    // get rid of .md extension
                    let child_name_name = &child_name[..child_name.len()-3];

                    link_tree.push_str("- [");
                    link_tree.push_str(child_name_name);
                    link_tree.push_str("](./");
                    link_tree.push_str(child_name);
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
