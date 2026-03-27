use std::env;
use std::path::{PathBuf, Path};
use std::fs;

fn main() {
    let mut has_acf = false;
    let asset_path: PathBuf = Path::new("assets").to_path_buf();
    let bname = match env::args().nth(1) {
        Some(i) => Block::new(&i),
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
    let templates: Vec<Template> = vec![
        Template::new(false, true, false, Path::new("template.php")),
        Template::new(true, true, false, Path::new("block.json")),
        Template::new(true, false, true, Path::new("assets/template.scss"))];
        let template_location: PathBuf = home_path.join(".config/qblock/template");
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => panic!("{}", e),
    };
    let wp = match WPinstall::find_wp_root(current_dir) {
        Some(dir) => WPinstall::new(Path::new(&dir)),
        None => panic!("You are not in a wordpress installation!!!!!!!!"),
    };
    match fs::create_dir_all(&wp.full_path.join(&bname.to_path()).join(&asset_path)) {
        Ok(_) => (),
        Err(e) => panic!("Error block already exists: {}", e),
    }
    let low_placeholder = "LOW_PLACEHOLDER";
    let cap_placeholder = "CAP_PLACEHOLDER";
    for template in templates {
        let template_file = &template_location.join(&template.location);
        let mut template_content = match fs::read_to_string(&template_file) {
            Ok(res) => res,
            Err(e) => panic!("Failed reading file: {}, Error: {}", &template_file.display(), e),
        };
        if template.low {
            template_content = template_content.replace(&low_placeholder, &bname.name);
        }
        if template.cap {
            template_content = template_content.replace(&cap_placeholder, &bname.to_upper());
        }
        let mut write_location: PathBuf = PathBuf::from(&wp.full_path.join(&bname.to_path().join(&template.location)));
        if template.is_assets {
            write_location = PathBuf::from(write_location.into_os_string().into_string().unwrap().replace("template", &bname.name));
        }
        match fs::write(&write_location, template_content) {
            Ok(_) => (),
            Err(e) => {
                match fs::remove_dir_all(&bname.to_path()){
                    Ok(_) => (),
                    Err(e_i) => panic!("Failed failing cleanup had errors: {}", e_i),
                }
                panic!("Failed writing to file: {}, Error: {}", &write_location.display(), e);
            },
        };
        if has_acf {
            println!("This feature is under development lol");
        }
    }
}

fn help() {

}

struct Block {
    name: String,
    block_files: Vec<Template>,
}

impl Block {
    fn new(in_name: &str, ) -> Self {
        Self {
            name: String::from(in_name)
        }
    }

    fn to_path(&self) -> &Path {
        Path::new(&self.name)
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

enum TemplateInfo {
    Low,
    Cap,
    IsAssets,
}

type TI = TemplateInfo;

// Change the booleans to use enums instead
struct Template {
    low: bool,
    cap: bool,
    is_assets: bool,
    location: PathBuf,
}

impl Template {
    pub fn new(triple: (bool, bool, bool), i_location: &str) -> Template {
        Template {
            low: triple.0,
            cap: triple.1,
            is_assets: triple.2,
            location: PathBuf::from(Path::new(i_location)),
        }
    }
}

struct TemplateMan {
    templates: Vec<Template>,
}

struct Replacer {
    block: Block,
    template: TemplateMan,
}

impl Replacer {
    fn template_to_block(template: Template, block: Block) {

    }
}
