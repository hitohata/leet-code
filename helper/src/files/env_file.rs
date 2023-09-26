use dotenv::dotenv;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::BufReader;

pub struct EnvFile {
    pub input_dir: String,
    pub output_dir: String,
}

impl EnvFile {
    pub fn file_dir() -> Result<Self, String> {
        todo!()
    }

    fn read_file() -> String {
        dotenv();
        todo!()
    }
}

/// languages and its extensions
#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub struct LanguageAndExtension {
    pub languageName: String,
    pub languageExtension: String,
}

impl LanguageAndExtension {
    /// read a languages.json on the top dir of this project.
    /// the structure of that file will be following:
    /// ```json
    /// [
    ///     {
    ///         languageName: "rust",
    ///         languageExtension: "rs"
    ///     },
    /// ]
    /// ```
    pub fn new() -> Vec<Self> {
        let file_name = if cfg!(test) {
            "./test_data/test_language.json"
        } else {
            "language.json"
        };

        let file = Self::open_language_file(file_name);
        let buf_reader = BufReader::new(file);

        let language_and_extension: Vec<LanguageAndExtension> =
            match serde_json::from_reader(buf_reader) {
                Ok(data) => data,
                Err(_) => panic!("reading the language.json error."),
            };

        language_and_extension
    }

    //
    fn open_language_file(file_path: &str) -> File {
        match File::open(file_path) {
            Ok(file) => file,
            Err(_) => panic!("Cannot find 'language.json' file. make sure you deploy it to project root directory.")

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_extention_file_open() {
        let lang_extention = LanguageAndExtension::new();
        assert_eq!(lang_extention.len(), 1);

        let first_one = &lang_extention[0];
        assert_eq!(first_one.languageName, "rust".to_string());
        assert_eq!(first_one.languageExtension, "rs".to_string());
    }

    #[test]
    #[should_panic(
        expected = "Cannot find 'language.json' file. make sure you deploy it to project root directory."
    )]
    fn language_file_not_found() {
        LanguageAndExtension::open_language_file("nofile");
    }
}
