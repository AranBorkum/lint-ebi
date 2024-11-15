use serde_derive::Deserialize;
use std::fs::read_to_string;
use std::process::exit;

#[derive(Deserialize)]
struct Data {
    lint_ebi: LintEbiConfig,
}

#[derive(Deserialize)]
pub struct LintEbiConfig {
    example: Option<String>,
    something: Option<String>,
}

impl Default for LintEbiConfig {
    fn default() -> LintEbiConfig {
        LintEbiConfig {
            example: Option::from(String::from("example")),
            something: Option::from(String::from("something")),
        }
    }
}

// impl LintEbiConfig {
// fn default() -> LintEbiConfig {
//     LintEbiConfig {
//         ..Default::default()
//     }
// }
// pub fn get_example(&self) -> String {
//     self.example
//         .clone()
//         .unwrap_or(Self::default().example.unwrap())
// }
// pub fn get_something(&self) -> String {
//     self.something
//         .clone()
//         .unwrap_or(Self::default().something.unwrap())
// }
// }

pub fn read_config(config: &str) -> LintEbiConfig {
    let filename = config;

    let contents = read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Couldn't read pyproject.toml");
        exit(1)
    });

    let data: Data = toml::from_str(&contents).unwrap_or_else(|_| {
        println!("Couldn't read data from pyproject.toml, using default values");
        Data {
            lint_ebi: LintEbiConfig {
                ..Default::default()
            },
        }
    });

    data.lint_ebi
}
