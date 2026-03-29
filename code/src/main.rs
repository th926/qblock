use std::env;
use std::path::{PathBuf, Path};
use std::fs;

mod qblock;

use crate::qblock::{
    template::{Template, TemplateConfig},
    block::Block, wpinstall::WPinstall,
    replacer::{Replacer, ReplacerPlaceholders}};

fn main() {
    let mut has_acf = false;
    let bname = match env::args().nth(1) {
        Some(i) => i,
        None => {help(); panic!("You need to provide a name for the block!");},
    };
    let acf_options: Vec<String> = env::args().skip(2).collect();
    if !acf_options.is_empty() {
        has_acf = true;
    }
    let home_var = match env::var("HOME") {
        Ok(var) => var,
        Err(e) => panic!("$HOME variable might not be set, Error: {}", e),
    };
    let home_path = PathBuf::from(&home_var);
    let block = Block::new(&bname, "assets");
    let templates: Vec<Template> = vec![ // Can I make this prettie?
        Template::new((false, true, false), "template.php", None),
        Template::new((true, true, false), "block.json", None),
        Template::new((true, false, true), "assets/template.scss", Some(&block.name))
    ];

    let template_conf = TemplateConfig::new(templates, &home_path.join(".config/qblock/template"));
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => panic!("{}", e),
    };
    let wp = match WPinstall::find_wp_root(current_dir) {
        Some(dir) => WPinstall::new(Path::new(&dir)),
        None => panic!("You are not in a wordpress installation!!!!!!!!"),
    };
    match fs::create_dir_all(&wp.full_path.join(&block.to_full_path())) {
        Ok(_) => (),
        Err(e) => panic!("Error block already exists: {}", e),
    }
    let placeholders = ReplacerPlaceholders::new("CAP_PLACEHOLDER", "LOW_PLACEHOLDER");
    let replacer = Replacer::new(wp, placeholders);
    for template in template_conf.templates {
        let template_path = &template_conf.location.join(&template.location);
        replacer.template_to_block(template, template_path, &block);
    }
        if has_acf {
            println!("This feature is under development lol");
        }
}

fn help() {

}
