use crate::enums::import_type::ImportType;
use crate::language::python::is_python_file;
use rayon::prelude::*;
use regex::bytes::Regex;
use std::fs::read_to_string;
use walkdir::DirEntry;

pub fn search_for_entity_imports(dir_entry: &DirEntry, parallel: bool, directory: &String) {
    if is_python_file(dir_entry) {
        match parallel {
            true => run_parallel(dir_entry, directory),
            false => run_consecutive(dir_entry, directory),
        }
    }
}

fn run_parallel(dir_entry: &DirEntry, directory: &String) {
    read_to_string(dir_entry.path().to_str().unwrap())
        .par_iter()
        .for_each(|line| analyse_line(dir_entry, line, directory));
}

fn run_consecutive(dir_entry: &DirEntry, directory: &String) {
    for line in read_to_string(dir_entry.path().to_str().unwrap())
        .unwrap()
        .lines()
    {
        analyse_line(dir_entry, line, directory);
    }
}

fn analyse_line(dir_entry: &DirEntry, line: &str, directory: &String) {
    let has_entity_import = has_entity_imports(&line);

    if has_entity_import {
        let dir = dir_entry.path().to_str().unwrap();
        get_file_root(dir, directory);
        get_import_root(line);
    }

    let is_in_correct_directory: bool =
        has_entity_or_use_case_imports(dir_entry.path().to_str().unwrap());
    if !is_in_correct_directory && has_entity_import {
        println!("{} invalid import", dir_entry.path().to_str().unwrap());
    }
}

fn has_entity_imports(line: &str) -> bool {
    let mut has_entity_import = false;
    if let Some(_captures) = ImportType::Entity.import_regex().captures(&line) {
        has_entity_import = true;
    }
    has_entity_import
}

fn has_entity_or_use_case_imports(root: &str) -> bool {
    let mut is_in_correct_directory: bool = false;
    let capture_string = Regex::new("entities|use_cases").unwrap();

    if let Some(_captures) = capture_string.captures(root.as_ref()) {
        is_in_correct_directory = true;
    }
    is_in_correct_directory
}

fn get_import_root(line: &str) -> String {
    let space_split: Vec<&str> = line.split(" ").collect();
    let import_path: Vec<&str> = space_split[1].split(".").collect();
    import_path[0].to_string()
}

fn get_file_root(dir: &str, directory: &str) -> String {
    let path = dir.replace(directory, "");
    let split_path: Vec<&str> = path.split("/").collect();
    split_path[0].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_has_entity_imports() {
        let result = has_entity_imports("from something.entities.this import this");
        assert_eq!(result, true);
    }

    #[test]
    fn test_does_not_have_entity_imports() {
        let result = has_entity_imports("from something.use_cases.something import this");
        assert_eq!(result, false);
    }

    #[test]
    fn test_entity_has_entity_or_use_case_imports_in_root() {
        let result = has_entity_or_use_case_imports(
            "../PromotionContext/src/entities/something/something_else",
        );
        assert_eq!(result, true);
    }

    #[test]
    fn test_use_case_has_entity_or_use_case_imports_in_root() {
        let result = has_entity_or_use_case_imports(
            "../PromotionContext/src/use_cases/something/something_else",
        );
        assert_eq!(result, true);
    }

    #[test]
    fn test_get_import_root() {
        let result = get_import_root("from promotion_context.use_cases.something import this");
        assert_eq!(result, "promotion_context");
    }

    #[test]
    fn test_get_file_root() {
        let result = get_file_root(
            "../PromotionService/src/target/this/that",
            "../PromotionService/src/",
        );
        assert_eq!(result, "target");
    }
}
