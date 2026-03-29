use std::path::PathBuf;
use std::fs;

use crate::qblock::{wpinstall::WPinstall, block::Block, template::Template};


pub struct ReplacerPlaceholders {
    cap_pattern: String,
    low_pattern: String,
}

impl ReplacerPlaceholders {
    pub fn new(cap: &str, low: &str) -> Self {
        Self {
            cap_pattern: String::from(cap),
            low_pattern: String::from(low),
        }
    }
}

pub struct Replacer {
    placeholders: ReplacerPlaceholders,
    wp: WPinstall,
}

impl Replacer {
    pub fn new(in_wp: WPinstall, in_placeholders: ReplacerPlaceholders) -> Self {
        Self {
            placeholders: in_placeholders,
            wp: in_wp,
        }
    }
    pub fn template_to_block(&self, template: Template, template_path: &PathBuf, block: &Block) {
        let template_full_path = template_path.join(&template.location);
        let mut template_content = match fs::read_to_string(&template_full_path) {
            Ok(res) => res,
            Err(e) => panic!("Failed reading file: {}, Error: {}", &template_full_path.display(), e),
        };
        if template.low {
            template_content = template_content.replace(&self.placeholders.low_pattern, &block.name);
        }
        if template.cap {
            template_content = template_content.replace(&self.placeholders.cap_pattern, &block.to_upper());
        }
        let block_full_path: PathBuf;
        if template.is_assets {
            block_full_path = self.wp.full_path.join(&block.to_path()).join(template.namechange.unwrap());
        } else {
            block_full_path = self.wp.full_path.join(&block.to_path()).join(template.location);
        }
        match fs::write(&block_full_path, template_content) {
            Ok(_) => (),
            Err(e) => {
                match fs::remove_dir_all(block.to_path()){
                    Ok(_) => (),
                    Err(e_i) => panic!("Failed failing cleanup had errors: {}, directory: {}", e_i, &block.to_path().display()),
                }
                panic!("Failed writing to file: {}, Error: {}", &block_full_path.display(), e);
            },
        };
    }
}
