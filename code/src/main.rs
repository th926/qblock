use std::env;
use std::path::{PathBuf, Path};
use std::fs;

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
    let block = Block::new("assets", &bname);
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
        replacer.template_to_block(&template, &block);
    }
        if has_acf {
            println!("This feature is under development lol");
        }
}

fn help() {

}

struct Block {
    name: String,
    full_path: PathBuf,
}

impl Block {
    fn new(in_name: &str, asset: &String) -> Self {
        let tmp_name = String::from(in_name);
        let tmp_path = PathBuf::from(&tmp_name).join(asset);
        Self {
            name: tmp_name,
            full_path: tmp_path,
        }
    }

    fn to_path(&self) -> &Path {
        Path::new(&self.name)
    }

    fn to_full_path(&self) -> &Path {
        &self.full_path
    }

    fn to_upper(&self)-> String {
        let mut c = self.name.chars();
        match c.next() {
            None => self.name.to_uppercase(),
            Some(f) => (f.to_uppercase().collect::<String>() + c.as_str()).replace("-", " "),
        }
    }
}

struct WPinstall {
    site_name: PathBuf,
    block_path: PathBuf,
    theme_path: PathBuf,
    location: PathBuf,
    full_path: PathBuf,
}

impl WPinstall {
    fn new(in_location: &Path) -> Self {
        let bp = String::from("src/blocks");
        let tp = String::from("wp-content/themes");
        let sn = PathBuf::from(in_location.file_stem().unwrap());
        let full = in_location.join(&tp).join(&sn).join(&bp);
        Self {
            block_path: PathBuf::from(bp),
            theme_path: PathBuf::from(tp),
            site_name: sn,
            location: PathBuf::from(in_location),
            full_path: full,
        }
    }
    fn find_wp_root(start_path: PathBuf) -> Option<PathBuf> {
        let target_files = ["wp-content", "wp-admin", "wp-config.php"];
        let mut current_path = start_path;

        loop {
            let mut found_count = 0;

            for target_file in target_files {
                let file_path = current_path.join(target_file);
                if file_path.exists() {
                    found_count += 1;
                }
            }
            if found_count == target_files.len() {
                return Some(current_path);
            }

            match current_path.parent() {
                Some(parent) if parent != current_path => {
                    current_path = parent.to_path_buf();
                }
                _ => {
                    return None;
                }
            }
        }
    }
}

#[derive(Clone)]
struct Template {
    low: bool,
    cap: bool,
    is_assets: bool,
    location: PathBuf,
    namechange: Option<PathBuf>
}

impl Template {
    pub fn new(triple: (bool, bool, bool), i_location: &str, bname: Option<&str>) -> Template {
        let changed_name: Option<PathBuf>;
        if triple.2 {
            changed_name =  Some(PathBuf::from(i_location.replace("template", &bname.unwrap())));
        } else {
            changed_name = None;
        }
        Template {
            low: triple.0,
            cap: triple.1,
            is_assets: triple.2,
            namechange: changed_name,
            location: PathBuf::from(Path::new(i_location)),
        }
    }
}

struct TemplateConfig {
    location: PathBuf,
    templates: Vec<Template>,
}

impl TemplateConfig {
    fn new(temps: Vec<Template>, loc: &Path) -> Self {
        Self {
            location: PathBuf::from(loc),
            templates: temps,
        }
    }
}

struct ReplacerPlaceholders {
    cap_pattern: String,
    low_pattern: String,
}

impl ReplacerPlaceholders {
    fn new(cap: &str, low: &str) -> Self {
        Self {
            cap_pattern: String::from(cap),
            low_pattern: String::from(low),
        }
    }
}

struct Replacer {
    placeholders: ReplacerPlaceholders,
    wp: WPinstall,
}

impl Replacer {
    fn new(in_wp: WPinstall, in_placeholders: ReplacerPlaceholders) -> Self {
        Self {
            placeholders: in_placeholders,
            wp: in_wp,
        }
    }
    fn template_to_block(&self, template: &Template, block: &Block) {
        let template_full_path = self.wp.full_path.join(&template.location);
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
                match fs::remove_dir_all(&block.to_path()){
                    Ok(_) => (),
                    Err(e_i) => panic!("Failed failing cleanup had errors: {}", e_i),
                }
                panic!("Failed writing to file: {}, Error: {}", &block_full_path.display(), e);
            },
        };
    }
}
