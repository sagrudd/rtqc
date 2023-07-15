use file_lock::{FileLock, FileOptions};
use std::io::prelude::*;
use std::path::Path;
// use extendr_api::{rprintln, print_r_output};
use std::collections::HashMap;
use std::path::PathBuf;


pub fn register_fq_index_pair(dir: &str, src: &str, idx: &str) {
    let fq_index_log = Path::new(dir).join(crate::fastq::FASTQ_PARQUET_LOG);
    let str = format!("{},{}\n", src, idx);
    file_append(fq_index_log, str.as_bytes());
}

pub fn load_fq_index_pairs(dir: &str) -> HashMap<String, String> {
    let fq_index_log = Path::new(dir).join(crate::fastq::FASTQ_PARQUET_LOG);
    let mut fastq_pairs = HashMap::new();

    let buffer = read_file_content(fq_index_log);
    for line in buffer.lines() {
        let splitter: Vec<&str> = line.splitn(2, ',').collect();
        fastq_pairs.insert(splitter.get(0).unwrap().to_string(), 
                            splitter.get(1).unwrap().to_string()
        );
    }
    return fastq_pairs;
}

fn file_append(file_path: PathBuf, payload: &[u8]) {
    let should_we_block  = true;
    let options = FileOptions::new()
                        .write(true)
                        .create(true)
                        .append(true);

    let mut filelock = match FileLock::lock(file_path, should_we_block, options) {
        Ok(lock) => lock,
        Err(err) => panic!("Error getting write lock: {}", err),
    };

    let _ = filelock.file.write_all(payload).is_ok();
    let _ = filelock.unlock();
}


fn read_file_content(file_path: PathBuf) -> String {
    let should_we_block  = true;
    let options = FileOptions::new()
                        .read(true)
                        .create(true)
                        .append(true);
    // rprintln!("loading file {:?}", file_path);

    let mut filelock = match FileLock::lock(file_path, should_we_block, options) {
        Ok(lock) => lock,
        Err(err) => panic!("Error getting write lock: {}", err),
    };

    let mut buffer = String::new();
    //let _ = filelock.file.read_to_string(&mut buffer);
    match filelock.file.read_to_string(&mut buffer) {
        Ok(_) => {},
        Err(e) => panic!("Error parsing the parquet index: {}", e),
    }
    let _ = filelock.unlock();

    return buffer;
}