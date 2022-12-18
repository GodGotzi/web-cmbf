use actix_web::http::header::HeaderValue;
use std::path::Path;
use crate::properties::{Properties};

pub struct LanguageProperties<'a>{
    filepath: &'a str,
    languages: Vec<&'a str>
}

impl <'a> LanguageProperties<'a> {
    pub fn new(filepath: &'a str, properties: &'static Properties) -> LanguageProperties<'a> {
        let language_str = match properties.map.get("languages") {
            Some(value) => {
                &value[1..value.len()-1]
            },
            None => panic!("languages Value doesn't exist")
        };

        let languages: Vec<&str> = language_str.split(";").collect();

        Self {
            filepath,
            languages
        }
    }
}

impl <'a> Clone for LanguageProperties<'a> {
    fn clone(&self) -> Self {
        let filepath = self.filepath.clone();
        let languages= self.languages.clone();

        Self {filepath, languages}
    }
}

pub fn starts_with_language(path: &str, languages: Vec<&str>) -> bool {
    for language in languages {
        if path.starts_with(language) {
            return true;
        }
    }

    return false;
}

pub fn get_language(language_value: &HeaderValue) -> &str {
    return match language_value.to_str() {
        Ok(str) => {
            let split_vec: Vec<&str> = str.split(",").collect();
            let ret: &str = split_vec.get(1).unwrap();
            ret
        },
        Err(_) => "de"
    };
}

pub fn get_path(path: &str, language: &str, language_properties: &LanguageProperties) -> String {
    let languages = language_properties.clone().languages;

    if path.contains("..") {
        return format!("frontend/{}/err.html", language);
    }

    return match path {
        "/" => {
            format!("frontend/{}/home.html", language)
        },
        _ => {
            if Path::new(format!("frontend{}", path).as_str()).exists() {
                return format!("frontend{}", path);
            } else if !starts_with_language(path, languages) {
                if Path::new(format!("frontend/{}{}", language, path)
                    .as_str())
                    .exists() {
                    return format!("frontend/{}{}", language, path);
                }
            }

            format!("frontend/{}/err.html", language)
        }
    }
}