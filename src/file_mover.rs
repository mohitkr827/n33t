use std::fs;
use std::path::Path;
use std::fs::create_dir_all;
use log::info;

pub struct FileMover;

impl FileMover {
    pub fn move_file(file_path: &Path, destination_folder: &Path) -> std::io::Result<()> {
        create_dir_all(destination_folder)?;
        let new_path = destination_folder.join(file_path.file_name().unwrap());
        fs::rename(file_path, &new_path)?;
        info!("Moved {:?} -> {:?}", file_path, new_path);
        Ok(())
    }
}
