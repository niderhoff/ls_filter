use std::{fs::OpenOptions, path::PathBuf};

use crate::model::Task;

pub fn ser_json(data: Vec<Task>, outfile: PathBuf) -> Result<(), std::io::Error> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(outfile)?;
    serde_json::to_writer(file, &data)?;
    Ok(())
}
