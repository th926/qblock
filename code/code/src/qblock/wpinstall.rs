use std::path::{PathBuf, Path};

pub struct WPinstall {
    pub site_name: PathBuf,
    pub block_path: PathBuf,
    pub theme_path: PathBuf,
    pub location: PathBuf,
    pub full_path: PathBuf,
}

impl WPinstall {
    pub fn new(in_location: &Path) -> Self {
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
    pub fn find_wp_root(start_path: PathBuf) -> Option<PathBuf> {
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
