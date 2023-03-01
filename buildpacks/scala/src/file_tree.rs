use crate::file_tree::FileTreeError::{
    CannotConvertToPathWithNoPrefix, CouldNotDeleteFile, CouldNotRetrieveFileList,
    InvalidExcludePattern, InvalidIncludePattern,
};
use std::fs;
use std::path::PathBuf;

pub(crate) struct FileTree {
    root: PathBuf,
    includes: Vec<String>,
    excludes: Vec<String>,
}

#[derive(Debug)]
pub(crate) enum FileTreeError {
    CouldNotRetrieveFileList(glob::PatternError),
    InvalidIncludePattern(glob::PatternError),
    InvalidExcludePattern(glob::PatternError),
    CannotConvertToPathWithNoPrefix(std::path::StripPrefixError),
    CouldNotDeleteFile(std::io::Error),
}

impl FileTree {
    pub(crate) fn include<Pattern: Into<String>>(&mut self, pattern: Pattern) -> &mut FileTree {
        self.includes.push(pattern.into());
        self
    }

    pub(crate) fn exclude<Pattern: Into<String>>(&mut self, pattern: Pattern) -> &mut FileTree {
        self.excludes.push(pattern.into());
        self
    }

    pub(crate) fn get_files(&self) -> Result<Vec<PathBuf>, FileTreeError> {
        let entries = glob::glob(&self.root.join("**").join("*").to_string_lossy())
            .map_err(CouldNotRetrieveFileList)?;

        let mut include_patterns: Vec<glob::Pattern> = vec![];
        for include in &self.includes {
            let include_pattern = glob::Pattern::new(include).map_err(InvalidIncludePattern)?;
            include_patterns.push(include_pattern);
        }

        let mut exclude_patterns: Vec<glob::Pattern> = vec![];
        for exclude in &self.excludes {
            let exclude_pattern = glob::Pattern::new(exclude).map_err(InvalidExcludePattern)?;
            exclude_patterns.push(exclude_pattern);
        }

        let mut all_files: Vec<PathBuf> = vec![];
        for file in entries
            .filter_map(Result::ok)
            .filter(|entry| entry.is_file())
        {
            let path_without_prefix = file
                .strip_prefix(&self.root)
                .map_err(CannotConvertToPathWithNoPrefix)?;
            all_files.push(path_without_prefix.to_path_buf());
        }

        let filter_files = all_files
            .iter()
            .filter(|file| {
                if include_patterns.is_empty() {
                    return true;
                }
                include_patterns
                    .iter()
                    .any(|pattern| pattern.matches(&file.to_string_lossy()))
            })
            .filter(|file| {
                if exclude_patterns.is_empty() {
                    return true;
                }
                !exclude_patterns
                    .iter()
                    .any(|pattern| pattern.matches(&file.to_string_lossy()))
            })
            .cloned()
            .collect();

        Ok(filter_files)
    }

    pub(crate) fn delete(&self) -> Result<(), FileTreeError> {
        let files = self.get_files()?;
        for file in files {
            fs::remove_file(self.root.join(file)).map_err(CouldNotDeleteFile)?;
        }
        Ok(())
    }
}

pub(crate) fn create_file_tree(root: PathBuf) -> FileTree {
    FileTree {
        root,
        includes: vec![],
        excludes: vec![],
    }
}

#[cfg(test)]
mod file_tree_tests {
    use crate::file_tree::create_file_tree;
    use std::fs::{create_dir_all, write};
    use std::path::{Path, PathBuf};
    use tempfile::tempdir;

    fn create_file(root: &Path, file_path: &str) {
        let path = root.join(file_path);
        let dir = path.parent().unwrap();
        create_dir_all(dir).unwrap();
        write(path, "").unwrap();
    }

    #[test]
    pub fn check_includes_excludes() {
        let temp_dir = tempdir().unwrap();
        let root = temp_dir.path();

        create_file(root, "scala-include0.java");
        create_file(root, "scala-dir/include1.txt");
        create_file(root, "streams/include2.yml");
        create_file(root, "resolution-cache/include3-test.xml");
        create_file(root, "resolution-cache/nested/include4.txt");
        create_file(root, "exclude0.java");
        create_file(root, "resolution-cache/exclude1-compile.xml");
        create_file(root, "resolution-cache/reports/exclude2.txt");

        let files = create_file_tree(root.to_path_buf())
            .include("scala-*")
            .include("streams/*")
            .include("resolution-cache/*")
            .exclude("resolution-cache/reports/*")
            .exclude("resolution-cache/*-compile.xml")
            .get_files()
            .unwrap();

        let mut expected_files = vec![
            "scala-include0.java",
            "scala-dir/include1.txt",
            "streams/include2.yml",
            "resolution-cache/include3-test.xml",
            "resolution-cache/nested/include4.txt",
        ];
        expected_files.sort_unstable();
        let expected_files: Vec<PathBuf> = expected_files.iter().map(PathBuf::from).collect();

        assert_eq!(files, expected_files);
    }

    #[test]
    pub fn check_can_delete_file_tree() {
        let temp_dir = tempdir().unwrap();
        let root = temp_dir.path();

        create_file(root, "scala-include0.java");
        create_file(root, "scala-dir/include1.txt");
        create_file(root, "streams/include2.yml");
        create_file(root, "resolution-cache/include3-test.xml");
        create_file(root, "resolution-cache/nested/include4.txt");
        create_file(root, "exclude0.java");
        create_file(root, "resolution-cache/exclude1-compile.xml");
        create_file(root, "resolution-cache/reports/exclude2.txt");

        create_file_tree(root.to_path_buf())
            .include("scala-*")
            .include("streams/*")
            .include("resolution-cache/*")
            .exclude("resolution-cache/reports/*")
            .exclude("resolution-cache/*-compile.xml")
            .delete()
            .unwrap();

        let remaining_files = create_file_tree(root.to_path_buf()).get_files().unwrap();

        let mut expected_files = vec![
            "exclude0.java",
            "resolution-cache/exclude1-compile.xml",
            "resolution-cache/reports/exclude2.txt",
        ];
        expected_files.sort_unstable();
        let expected_files: Vec<PathBuf> = expected_files.iter().map(PathBuf::from).collect();

        assert_eq!(remaining_files, expected_files);
    }
}
