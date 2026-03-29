use std::path::{Path, PathBuf};

pub struct Block {
    pub name: String,
    pub full_path: PathBuf,
}

impl Block {
    pub fn new(in_name: &String, asset: &str) -> Self {
        let tmp_name = String::from(in_name);
        let tmp_path = PathBuf::from(&tmp_name).join(asset);
        Self {
            name: tmp_name,
            full_path: tmp_path,
        }
    }

    pub fn to_path(&self) -> &Path {
        Path::new(&self.name)
    }

    pub fn to_full_path(&self) -> &Path {
        &self.full_path
    }

    pub fn to_upper(&self)-> String {
        let mut c = self.name.chars();
        match c.next() {
            None => self.name.to_uppercase(),
            Some(f) => (f.to_uppercase().collect::<String>() + c.as_str()).replace("-", " "),
        }
    }
}
