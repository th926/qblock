use std::path::{PathBuf, Path};
pub struct Template {
    pub low: bool,
    pub cap: bool,
    pub is_assets: bool,
    pub location: PathBuf,
    pub namechange: Option<PathBuf>
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

pub struct TemplateConfig {
    pub location: PathBuf,
    pub templates: Vec<Template>,
}

impl TemplateConfig {
    pub fn new(temps: Vec<Template>, loc: &Path) -> Self {
        Self {
            location: PathBuf::from(loc),
            templates: temps,
        }
    }
}
