use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub enum ReadJsonError {
    Io(std::io::Error),
    Json(serde_json::Error),
}
impl From<std::io::Error> for ReadJsonError {
    fn from(e: std::io::Error) -> Self { Self::Io(e) }
}
impl From<serde_json::Error> for ReadJsonError {
    fn from(e: serde_json::Error) -> Self { Self::Json(e) }
}


pub fn read_json_file<T: DeserializeOwned>(path: &impl AsRef<Path>) -> Result<T, ReadJsonError> {
    let file = File::open(path.as_ref())?;
    let reader = BufReader::new(file);
    let val = serde_json::from_reader(reader)?;
    Ok(val)
}