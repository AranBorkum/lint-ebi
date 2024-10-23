use crate::enums::import_type::ImportType;
use crate::language::python::is_python_file;
use rayon::prelude::*;
use regex::bytes::Regex;
use std::fs::read_to_string;
use walkdir::DirEntry;

fn run_parallel(dir_entry: &DirEntry) {
    read_to_string(dir_entry.path().to_str().unwrap())
        .par_iter()
        .for_each(|line| {
            if let Some(_captures) = ImportType::Entity.import_regex().captures(&line) {
                println!("{} has entity import", dir_entry.path().to_str().unwrap());
            }
        });
}

fn run_consecutive(dir_entry: &DirEntry) {
    for line in read_to_string(dir_entry.path().to_str().unwrap())
        .unwrap()
        .lines()
    {
        if let Some(_captures) = ImportType::Entity.import_regex().captures(&line) {
            println!("{} has entity import", dir_entry.path().to_str().unwrap());
        }
    }
}

pub fn search_for_entity_imports(dir_entry: &DirEntry, parallel: bool) {
    let has_import = false;
    if is_python_file(dir_entry) {
        match parallel {
            true => run_parallel(dir_entry),
            false => run_consecutive(dir_entry),
        }
    }

    if has_import {
        println!("{}", entity_import_in_right_path(dir_entry));
    }
}

fn entity_import_in_right_path(dir_entry: &DirEntry) -> bool {
    let mut correct_dir = false;
    let entity_re = Regex::new(r"entities").unwrap();
    let use_cases_re = Regex::new(r"use_cases").unwrap();
    let path = dir_entry.path().to_str().unwrap();
    if let Some(_capture) = entity_re.captures(path.as_ref()) {
        correct_dir = true
    }
    if !correct_dir {
        if let Some(_capture) = use_cases_re.captures(path.as_ref()) {
            correct_dir = true;
        }
    }
    correct_dir
}
