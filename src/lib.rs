use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

pub struct Component {
    pub directory: String,
}

impl Component {
    pub fn new(directory: &str) -> Self {
        Self {
            directory: directory.to_string(),
        }
    }

    pub fn spawn(
        &self,
        name: &str,
        fragments: Option<&HashMap<String, String>>,
    ) -> io::Result<String> {
        let file_name = format!("{}.html", name);
        let file_path = Path::new(&self.directory).join(file_name);

        if !file_path.exists() || !file_path.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Fragment not found",
            ));
        }

        let mut file = fs::File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        if let Some(fragments) = fragments {
            content = fragments.iter().fold(content, |acc, (key, value)| {
                acc.replace(&format!("__{}__", key), value)
            });
        }

        Ok(content)
    }
}
