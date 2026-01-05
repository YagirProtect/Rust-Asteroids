use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use serde::Serialize;

pub trait FileWritable{
    fn write_file(&self, path: PathBuf) where Self: Serialize{


        let file = File::create(path).unwrap();

        let w = BufWriter::new(file);

        let data = match serde_json::to_writer(w, self){
            Ok(x) => x,
            Err(e) => {
                panic!("Error serializing file: {}", e);
            }
        };
    }
}