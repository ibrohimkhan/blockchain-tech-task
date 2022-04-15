use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use walkdir::{DirEntry, WalkDir};

pub fn count_lines(dir: String, ext: String) -> u32 {
    let mut counter = 0;

    for entry in get_entries(dir, ext) {
        let file = BufReader::new(File::open(entry.path()).expect("Unable to open file"));
        file.lines().for_each(|_| counter += 1);
    }
    
    counter
}

fn get_entries(dir: String, ext: String) -> impl Iterator<Item = DirEntry> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir())
        .filter(move |f| {
            f.file_name()
                .to_str()
                .map(|name| name.ends_with(&ext))
                .expect("some issue with file name")
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_lines_in_single_file() {
        let dir = "root_dir/sub_dir1".to_string();
        let ext = "txt".to_string();
        let result = count_lines(dir, ext);
        assert_eq!(result, 2);
    }

    #[test]
    fn count_lines_in_multi_files() {
        let dir = "root_dir/sub_dir2".to_string();
        let ext = "txt".to_string();
        let result = count_lines(dir, ext);
        assert_eq!(result, 5);
    }

    #[test]
    fn count_lines_in_multi_files_and_dirs() {
        let dir = "root_dir".to_string();
        let ext = "txt".to_string();
        let result = count_lines(dir, ext);
        assert_eq!(result, 12);
    }

    #[test]
    fn count_lines_in_doc_file() {
        let dir = "root_dir".to_string();
        let ext = "doc".to_string();
        let result = count_lines(dir, ext);
        assert_eq!(result, 2);
    }
}
