use crate::enums::import_type::ImportType;
use crate::language::python::is_python_file;
use rayon::prelude::*;
use std::fs::read_to_string;
use walkdir::DirEntry;

pub fn search_for_repository_interface_imports(dir_entry: &DirEntry, parallel: bool) {
    if is_python_file(dir_entry) {
        match parallel {
            true => run_parallel(dir_entry),
            false => run_consecutive(dir_entry),
        }
    }
}

fn run_parallel(dir_entry: &DirEntry) {
    read_to_string(dir_entry.path().to_str().unwrap())
        .par_iter()
        .for_each(|line| {
            if let Some(_captures) = ImportType::RepositoryInterface
                .import_regex()
                .captures(&line)
            {
                println!(
                    "{} has repository import",
                    dir_entry.path().to_str().unwrap()
                );
            }
        });
}

fn run_consecutive(dir_entry: &DirEntry) {
    for line in read_to_string(dir_entry.path().to_str().unwrap())
        .unwrap()
        .lines()
    {
        if let Some(_captures) = ImportType::RepositoryInterface
            .import_regex()
            .captures(&line)
        {
            println!(
                "{} has repository import",
                dir_entry.path().to_str().unwrap()
            );
        }
    }
}
