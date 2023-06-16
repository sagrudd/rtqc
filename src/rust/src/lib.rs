  use extendr_api::prelude::*;
  use std::path::Path;
  use fastq::{parse_path, Record};
  use std::str;
  use std::collections::HashMap;
  use fastq::RefRecord;
  use polars::prelude::*;
  use std::fmt;
  use std::fs::File;


  #[extendr]
  fn index_fastq(fq_path: &str) -> extendr_api::Robj {

    println!("indexing fastq [{}]", fq_path);

    let dest = "/tmp/df.parquet";
    let mut written = false;

    // check that fastq specified exists
    if !Path::new(fq_path).exists() {
      eprintln!("file [{}] not found", fq_path);
    } else {
      parse_path(Some(fq_path), |parser| {
          let nthreads = 12;

          let results: Vec<DataFrame> = parser.parallel_each(nthreads, |record_sets| {

            let mut fq: Vec<FastqEntry> = Vec::new();
              for record_set in record_sets {
                  for record in record_set.iter() {
                    let fastq_read = hack_fastq(record);
                    // println!("{}", fastq_read);
                    fq.push(fastq_read);
                  }
              }

              let df: DataFrame = struct_to_dataframe!(fq, [accession, runid, read]).unwrap();
              //println!("{:?}", df);

              return df;
          }).expect("Invalid fastq file");

          //println!("{}", results.iter().sum::<usize>());

          //let new_df = DataFrame::concat(results).unwrap();

          let mut dg2 = DataFrame::default();
          for x in results {
            dg2.vstack_mut(&x).expect("Error merging fastq blocks");
          }

          let mut file = File::create(&dest).expect("could not create file");
          ParquetWriter::new(&mut file).finish(&mut dg2).expect("failed to write parquet");
          written = true;

      }).expect("Invalid compression");

    }

    if written {
      return r!(&dest);
    } else {
      return r!(NULL);
    }

    //let rfc3339 = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")?;
    
  }


  fn hack_fastq(record: RefRecord<'_>) -> FastqEntry {
    let s = match str::from_utf8(record.head()) {
      Ok(v) => v,
      Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let parts: Vec<String> = s.split(" ").map(|s| s.to_string()).collect();
    let accession = parts.first().cloned().unwrap();
    let mut fastq_meta: HashMap<String, String> = HashMap::new();
    for part in parts {
      if part.contains("=") {
          let splitter: Vec<&str> = part.splitn(2, '=').collect();
          fastq_meta.insert(splitter.get(0).unwrap().to_string(), splitter.get(1).unwrap().to_string());
      }
    }

    return FastqEntry { 
      accession: Some(accession), 
      runid: pick_feature_str("runid", &fastq_meta),
      read: pick_feature_u32("read", &fastq_meta),
      ch: pick_feature_u32("ch", &fastq_meta),
      start_time: pick_feature_str("start_time", &fastq_meta),
      protocol_group_id: pick_feature_str("protocol_group_id", &fastq_meta),
      sample_id: pick_feature_str("sample_id", &fastq_meta),
      parent_read_id: pick_feature_str("parent_read_id", &fastq_meta),
      basecall_model_version_id: pick_feature_str("basecall_model_version_id", &fastq_meta),

      ..Default::default() 
    }; 
    
  }


  fn pick_feature_str(key: &str, meta: &HashMap<String, String> ) -> Option<String> {
    let x = meta.get(key);
    match x {
      Some(x) => return Some(x.to_string()),
      None => return None
    }
  }

  fn pick_feature_u32(key: &str, meta: &HashMap<String, String> ) -> Option<u32> {
    let x = meta.get(key);
    match x {
      Some(x) => {
        let num: u32 = x.trim().parse().unwrap();
        return Some(num)
      },
      None => return None
    }
  }


  #[derive(Debug)]
  pub struct FastqEntry {
      pub accession: Option<String>,
      pub runid: Option<String>,
      pub read: Option<u32>,
      ch: Option<u32>,
      start_time: Option<String>,
      protocol_group_id: Option<String>,
      sample_id: Option<String>,
      parent_read_id: Option<String>,
      basecall_model_version_id: Option<String>,
  }


  impl Default for FastqEntry {
    fn default() -> FastqEntry {
      FastqEntry {
        accession: None,
        runid: None,
        read: None,
        ch: None,
        start_time: None,
        protocol_group_id: None,
        sample_id: None,
        parent_read_id: None,
        basecall_model_version_id: None,
        }
    }
}

#[macro_export]
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


impl fmt::Display for FastqEntry {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self.accession {
      Some(x) => write!(f, "({})", x),
      None => write!(f, "fubar fastq"),
    }
  }
}




  // Macro to generate exports.
  // This ensures exported functions are registered with R.
  // See corresponding C code in `entrypoint.c`.
  extendr_module! {
      mod rtqc;
      fn index_fastq;
  }
