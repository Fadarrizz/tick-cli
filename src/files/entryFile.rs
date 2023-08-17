use std::path::PathBuf;

struct EntryFile {
    path: PathBuf,
}

impl EntryFile {
    fn from_string(file_name: &String) -> Self {


        let path;

        Self { path }
    }

    pub fn get_path(&self) -> PathBuf {
        self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_file_from_string() {
        let file_name = "2023-08-01".to_string();

        let entry_file = EntryFile::from_string(&file_name);

        let expected_path: PathBuf = ["2023", "08-august", "01-tue"].iter().collect();
        assert_eq!(expected_path, entry_file.get_path());
    }
}
