use std::error::Error;
use std::fs::File;
use std::io::Read;

pub fn save(bytes: Vec<u8>,path: &str) -> Result<(), Box<dyn Error>> {
        use std::fs::File;
        use std::io::Write;
        let mut file = File::create(path).expect("failed to make path");
        file.write_all(&bytes).map_err(|e| e.into())
    }


pub fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}
