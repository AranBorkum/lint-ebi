extern crate walkdir;

use walkdir::WalkDir;

use pyo3::prelude::*;

mod config;
mod ebi;
mod enums;
mod language;

use crate::config::toml::{read_config, LintEbiConfig};
use crate::ebi::entity::search_for_entity_imports;
use crate::ebi::repository_interface::search_for_repository_interface_imports;

#[pyfunction]
fn lint_ebi(mut directory: String, parallel: bool, config: &str) {
    if !directory.ends_with('/') {
        directory.push('/');
    }
    let _lint_ebi_config: LintEbiConfig = read_config(config);
    let walker = WalkDir::new(directory.clone()).into_iter();
    for dir_entry in walker {
        match dir_entry {
            Ok(dir_entry) => {
                search_for_entity_imports(&dir_entry, parallel, &directory);
                search_for_repository_interface_imports(&dir_entry, parallel);
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

#[pymodule]
fn rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(lint_ebi, m)?)?;
    Ok(())
}
