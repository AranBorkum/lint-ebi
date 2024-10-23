use walkdir::DirEntry;

pub fn is_python_file(dir_entry: &DirEntry) -> bool {
    dir_entry
        .file_name()
        .to_str()
        .map(|file| file.ends_with(".py"))
        .unwrap_or(false)
}
