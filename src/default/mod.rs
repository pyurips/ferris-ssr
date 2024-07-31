use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

pub fn get_component(
    component_name: &str,
    components: Option<&HashMap<String, String>>,
    data: Option<&HashMap<String, String>>,
) -> io::Result<String> {
    let file_name = format!("{}.html", component_name);
    let file_path = Path::new("./components").join(file_name);

    if !file_path.exists() || !file_path.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Component not found",
        ));
    }

    let mut file = fs::File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    if let Some(components) = components {
        content = components.iter().fold(content, |acc, (key, value)| {
            acc.replace(&format!("__{}__", key), value)
        });
    }

    if let Some(data) = data {
        content = data.iter().fold(content, |acc, (key, value)| {
            acc.replace(&format!("__{}__", key), value)
        });
    }

    Ok(content)
}
