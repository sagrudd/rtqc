//use std::collections::HashMap;
use std::path::Path;
use extendr_api::{rprintln, print_r_output};

pub static ARROW_PARQUET_LOG: &str = "parquet_elements_mapped.txt";

pub fn prepare_arrow(dir: &str) {
    let fq_pairs = crate::filehandlers::load_fq_index_pairs(dir);
    let pq_mapped = crate::filehandlers::load_parquet_registrations(dir);

    for (_, parquet) in fq_pairs {
            if !pq_mapped.contains(&parquet) {
                let parquet_file = Path::new(&parquet);
                rprintln!("including {:?} into arrow object", parquet_file.file_name().unwrap());
            }
        }
}