use std::env;
use std::path::{PathBuf, Path};
use std::fs;

fn main() {
    let wp_block_src_placeholder = "{sname}";
    let wp_block_src = String::from("wp-content/themes/{sname}/src/blocks");
    let wp_block_src_path: PathBuf;
    let mut has_acf = false;
    let asset_path: PathBuf = Path::new("assets").to_path_buf();
    let bname = match env::args().nth(1) {
        Some(i) => i.to_lowercase(),
        None => String::new(),
    };
    if bname.is_empty() {
        help();
        panic!("You need to provide a block name");
    }
    let bname_cap = match f_upped(&bname) {
        Some(i) => i.replace("-", " "),
        None => String::from(&bname),
    };
    let bname_path = PathBuf::from(Path::new(&bname));
    let acf_options: Vec<String> = env::args().skip(2).collect();
    if !acf_options.is_empty() {
        has_acf = true;
    }
    let templates: Vec<Template> = vec![
        Template::new(false, true, false, Path::new("template.php")),
        Template::new(true, true, false, Path::new("block.json")),
        Template::new(true, false, true, Path::new("assets/template.scss"))];
        let template_location: PathBuf = PathBuf::from(Path::new("/Users/trygve/.config/fish/functions/qblock_template"));
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => panic!("{}", e),
    };
    let wp_root = match find_wp_root(current_dir) {
        Some(dir) => dir,
        None => panic!("You are not in a wordpress installation!!!!!!!!"),
    };
    match wp_root.file_stem() {
        Some(name) => {
            wp_block_src_path = PathBuf::from(wp_block_src.replace(wp_block_src_placeholder, name.to_str().unwrap()));
        },
        None => panic!("Something strange happened sorry lol"),
    };
    match fs::create_dir_all(&wp_block_src_path.join(&bname_path.join(&asset_path))) {
        Ok(_) => (),
        Err(e) => panic!("Error block already exists: {}", e),
    }
    let low_placeholder = "LOW_PLACEHOLDER";
    let cap_placeholder = "CAP_PLACEHOLDER";
    let block_base = PathBuf::from(&bname);
    for template in templates {
        let template_file = &template_location.join(&template.location);
        let mut template_content = match fs::read_to_string(&template_file) {
            Ok(res) => res,
            Err(e) => panic!("Failed reading file: {}, Error: {}", &template_file.display(), e),
        };
        if template.low {
            template_content = template_content.replace(&low_placeholder, &bname);
        }
        if template.cap {
            template_content = template_content.replace(&cap_placeholder, &bname_cap);
        }
        let mut write_location: PathBuf = PathBuf::from(&wp_block_src_path.join(block_base.join(&template.location)));
        if template.is_assets {
            write_location = PathBuf::from(write_location.into_os_string().into_string().unwrap().replace("template", &bname));
        }
        match fs::write(&write_location, template_content) {
            Ok(_) => (),
            Err(e) => {
                match fs::remove_dir_all(&block_base){
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

fn f_upped(s: &String)-> Option<String> {
    let mut c = s.chars();
    match c.next() {
        None => None,
        Some(f) => Some(f.to_uppercase().collect::<String>() + c.as_str()),
    }
}

fn help() {

}

struct Template {
    low: bool,
    cap: bool,
    is_assets: bool,
    location: PathBuf,
}

impl Template {
    pub fn new(i_low: bool, i_cap: bool, i_assets: bool, i_location: &Path) -> Template {
        Template {low: i_low, cap: i_cap, is_assets: i_assets, location: PathBuf::from(i_location)}
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

//TemplateFolder
//
// Template
//  lower bool
//  caper bool
//  location
//
