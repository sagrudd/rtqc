use std::path::Path;
use extendr_api::{rprintln, print_r_output};
use std::path::PathBuf;

use polars_core::prelude::*;
use polars_io::prelude::*;
use std::fs::File;


pub static ARROW_PARQUET_LOG: &str = "parquet_elements_mapped.txt";

static REFERENCE_FILE: &str = "SequenceSet.parquet";

#[allow(unused_assignments)]
pub fn prepare_arrow(dir: &str) -> bool {
    let fq_pairs = crate::filehandlers::load_fq_index_pairs(dir);
    let pq_mapped = crate::filehandlers::load_parquet_registrations(dir);

    // from the files available; score to see what is potentially novel - no novelty; no need to process
    let mut novel_elements: Vec<String> = Vec::new();
    let fastq_keys = &fq_pairs.keys().cloned().collect::<Vec<String>>();
    for fastq_key in fastq_keys {
        let parquet_val = fq_pairs.get(fastq_key).unwrap();
        if !pq_mapped.contains(parquet_val) {
            novel_elements.push(String::from(parquet_val));
        }
    }

    if novel_elements.len() == 0 {
        return false;
    }
    // flag to see if anything has been modified (requiring save)
    let mut modified = false;
    let reference_parquet = get_arrow_path(dir);

    let mut df: Option<DataFrame> = None;
    
    if pq_mapped.len() == 0 { // we must have unique elements to be here
        let novel_item = novel_elements.pop().unwrap();
        crate::filehandlers::register_parquet_merge(&dir, &novel_item);        
        df = clone_parquet(&novel_item, &reference_parquet.clone());
    } else {
        let x = reference_parquet.clone().into_os_string().into_string().unwrap();
        df = Some(read_parquet(&x));
    }
        

    while novel_elements.len() > 0 {
        let novel_item = novel_elements.pop().unwrap();

        crate::filehandlers::register_parquet_merge(&dir, &novel_item);
        
        let sub_df = read_parquet(&novel_item);
        let dg = df.as_mut().unwrap(); 
        dg.vstack_mut(&sub_df).expect("Error merging fastq blocks");
        modified = true;
    } 
    
    if modified {
        write_parquet(&reference_parquet, &mut df.clone().unwrap());
    }

    let _ = df.clone().unwrap().get_column_names();
    //rprintln!("{:?}", &df);

    return true;

}


pub fn get_arrow_path(dir: &str) -> PathBuf {
    return Path::new(dir).join(REFERENCE_FILE);
}


fn clone_parquet(source: &String, dest: &PathBuf) -> Option<DataFrame> {
    rprintln!("cloning sequence file {:?}", &source);
    let mut df = read_parquet(source);
    write_parquet(dest, &mut df);
    return Some(df);
}


fn read_parquet(source: &String) -> DataFrame {
    let r = File::open(source).unwrap();
    let reader = ParquetReader::new(r);
    let data = Some(reader.finish());
    let df = data.unwrap().unwrap();
    return df;
}

fn write_parquet(dest: &PathBuf, df: &mut DataFrame)  {
    let mut file = File::create(&dest).expect("could not create file");
    ParquetWriter::new(&mut file)
        .finish(df)
        .expect("failed to write parquet");
}