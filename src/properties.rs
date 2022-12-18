use std::collections::HashMap;
use std::fs;

pub struct Properties {
    pub map: HashMap<String, String>
}

impl <'a> Properties {
    pub fn new(filepath: &'a str) -> Properties {
        let map = load_properties(filepath);
        Self {map}
    }
}

impl Clone for Properties {
    fn clone(&self) -> Self {
        let map = self.map.clone();

        Self {map}
    }
}

fn load_properties(filepath: &str) -> HashMap<String, String> {
    let content = match fs::read_to_string(filepath) {
        Ok(content) => content,
        Err(err) => panic!("Language Properties File doesn't exist Error -> {}", err)
    };

    let mut map: HashMap<String, String> = HashMap::new();

    let lines: Vec<&str> = content.lines().collect();

    for line in lines {
        let index = match line.find("=") {
            Some(index) => index,
            None => panic!("Line {} no separator", line)
        };

        let replaced = line.replace("=", "");
        let (key, value): (&str, &str) = replaced.split_at(index);

        map.insert(String::from(key), String::from(value));
    }

    map
}

