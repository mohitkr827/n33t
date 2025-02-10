use std::fs;
use std::path::PathBuf;
use crate::rule_manager::RuleManager;
use crate::file_mover::FileMover;

pub struct FileOrganizer {
    directory: PathBuf,
    rules: std::collections::HashMap<&'static str, &'static str>,
}

impl FileOrganizer {
    pub fn new(directory: &str) -> Self {
        Self {
            directory: PathBuf::from(directory),
            rules: RuleManager::get_default_rules(),
        }
    }

    pub fn organize_files(&self) -> std::io::Result<()> {
        for entry in fs::read_dir(&self.directory)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if let Some(folder) = self.rules.get(ext) {
                        let new_dir = self.directory.join(folder);
                        FileMover::move_file(&path, &new_dir)?;
                    }
                }
            }
        }
        Ok(())
    }
}
