use fastq::RefRecord;
use fastq::{parse_path, Record};
use polars::prelude::*;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::path::Path;
use std::str;
use std::path::PathBuf;
use temp_file_name::{self, TempFilePath};
use extendr_api::{rprintln, print_r_output};
// use std::thread;
use crate::filehandlers::register_fq_index_pair;


pub static FASTQ_PARQUET_LOG: &str = "fq_index_pair.txt";


fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn index_fastq<'a>(fq_path: &'a str, dir: &'a str) -> Result<(String, String), String> {
    let path = Path::new(fq_path);
    // rprintln!("indexing fastq [{}] on thread {:?}", path.file_name().unwrap().to_str().unwrap(), thread::current().id());

    let y: String = String::from(PathBuf::from(fq_path).file_name().unwrap().to_str().unwrap());
    let x: &'static str = string_to_static_str(y);
    let mut dest = PathBuf::from(dir);
    let mut written = false;

    // check that fastq specified exists
    if !Path::new(fq_path).exists() {
        let estr = format!("file [{}] not found", fq_path);
        rprintln!("{}", estr.as_str());
        return Err::<(String, String), String>(estr);
    } else {

        // create a path to which the output should be written (or handle if the file already exists)
        dest = PathBuf::from(dir).join(fq_path.temp_filename("parquet"));

        // if the parquet file already exists - just move on ...
        if !dest.exists() {

            parse_path(Some(fq_path), |parser| {
                let nthreads = 1;

                let results: Vec<DataFrame> = parser
                    .parallel_each(nthreads, |record_sets| {
                        let mut fq: Vec<FastqEntry> = Vec::new();
                        for record_set in record_sets {
                            for record in record_set.iter() {
                            
                                let fastq_read = hack_fastq(x, record);
                                // println!("{}", fastq_read);
                                fq.push(fastq_read);
                            }
                        }

                        let df: DataFrame = struct_to_dataframe!(fq, [accession, file_of_origin, runid, read, ch,
                            start_time,
                            flow_cell_id,
                            protocol_group_id,
                            sample_id,
                            parent_read_id,
                            basecall_model_version_id, quality]).unwrap();

                        return df;
                    })
                    .expect("Invalid fastq file");


                let mut dg2 = DataFrame::default();
                for x in results {
                    dg2.vstack_mut(&x).expect("Error merging fastq blocks");
                }

                // rprintln!("{:?}", dg2.get_column_names());

                
                let mut file = File::create(&dest).expect("could not create file");
                ParquetWriter::new(&mut file)
                    .finish(&mut dg2)
                    .expect("failed to write parquet");
                written = true;
            })
            .expect("Invalid compression");
        }
    }   
    if written {
        register_fq_index_pair(dir, path.to_str().unwrap(), dest.to_str().unwrap());
    }

    if dest.exists() {
        return Ok((path.to_str().map(String::from).unwrap(),
            dest.to_str().map(String::from).unwrap()));
    } else {
        return Err(String::from("something not quite right"));
    }

    //let rfc3339 = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")?;
}

fn hack_fastq(file_str: &str, record: RefRecord<'_>) -> FastqEntry {
    let s = match str::from_utf8(record.head()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    //let file_str = String::from("");

    let parts: Vec<String> = s.split(" ").map(|s| s.to_string()).collect();
    let accession = parts.first().cloned().unwrap();
    let mut fastq_meta: HashMap<String, String> = HashMap::new();
    for part in parts {
        if part.contains("=") {
            let splitter: Vec<&str> = part.splitn(2, '=').collect();
            fastq_meta.insert(
                splitter.get(0).unwrap().to_string(),
                splitter.get(1).unwrap().to_string(),
            );
        }
    }

    /* 
    let mut basequals: Vec<f32> = Vec::new();
    for b in record.qual() {
        basequals.push(10_f32.powf(f32::from(b - 33) / -10_f32));
    }
    let meanerror = basequals.iter().sum::<f32>() as f32 / basequals.len() as f32;
    let meanscore = -10_f32 * meanerror.log10();
    */


    return FastqEntry {
        file_of_origin: Some(file_str.to_string()),
        accession: Some(accession),
        runid: pick_feature_str("runid", &fastq_meta),
        read: pick_feature_u32("read", &fastq_meta),
        ch: pick_feature_u32("ch", &fastq_meta),
        start_time: pick_feature_str("start_time", &fastq_meta),
        flow_cell_id: pick_feature_str("flow_cell_id", &fastq_meta),
        protocol_group_id: pick_feature_str("protocol_group_id", &fastq_meta),
        sample_id: pick_feature_str("sample_id", &fastq_meta),
        parent_read_id: pick_feature_str("parent_read_id", &fastq_meta),
        basecall_model_version_id: pick_feature_str("basecall_model_version_id", &fastq_meta),
        quality: Some(u8_to_mean_q(record.qual())),

        ..Default::default()
    };
}


pub fn u8_to_mean_q(qual: &[u8]) -> f32 {
    let basequals: Vec<f32> = qual.iter().map(|b| 10_f32.powf(f32::from(b - 33) / -10_f32)).collect::<Vec<f32>>();
    let meanscore = -10_f32 * (basequals.iter().sum::<f32>() as f32 / basequals.len() as f32).log10();
    return meanscore;
}


fn pick_feature_str(key: &str, meta: &HashMap<String, String>) -> Option<String> {
    let x = meta.get(key);
    match x {
        Some(x) => return Some(x.to_string()),
        None => return None,
    }
}

fn pick_feature_u32(key: &str, meta: &HashMap<String, String>) -> Option<u32> {
    let x = meta.get(key);
    match x {
        Some(x) => {
            let num: u32 = x.trim().parse().unwrap();
            return Some(num);
        }
        None => return None,
    }
}

#[derive(Debug)]
pub struct FastqEntry {
    pub file_of_origin: Option<String>,
    pub accession: Option<String>,
    pub runid: Option<String>,
    pub read: Option<u32>,
    pub ch: Option<u32>,
    pub start_time: Option<String>,
    pub flow_cell_id: Option<String>,
    pub protocol_group_id: Option<String>,
    pub sample_id: Option<String>,
    pub parent_read_id: Option<String>,
    pub basecall_model_version_id: Option<String>,
    pub quality: Option<f32>,
}

impl Default for FastqEntry {
    fn default() -> FastqEntry {
        FastqEntry {
            file_of_origin: None,
            accession: None,
            runid: None,
            read: None,
            ch: None,
            start_time: None,
            flow_cell_id: None,
            protocol_group_id: None,
            sample_id: None,
            parent_read_id: None,
            basecall_model_version_id: None,
            quality: None,
        }
    }
}

macro_rules! struct_to_dataframe {
    ($input:expr, [$($field:ident),+]) => {
        {
            // Extract the field values into separate vectors
            $(let mut $field = Vec::new();)*

            for e in $input.into_iter() {
                $($field.push(e.$field);)*
            }
            df! {
                $(stringify!($field) => $field,)*
            }
        }
    };
}
pub(crate) use struct_to_dataframe;

impl fmt::Display for FastqEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.accession {
            Some(x) => write!(f, "({})", x),
            None => write!(f, "fubar fastq"),
        }
    }
}
