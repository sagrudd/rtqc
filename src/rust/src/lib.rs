use extendr_api::prelude::*;

mod fastq;
mod fq_threaded;
mod filehandlers;

/// perform an index of fastq entry metadata
/// @export
#[extendr]
fn index_fastq(fq_path: &str, dir: &str) -> extendr_api::Robj {

    let index = fastq::index_fastq(fq_path, dir);

    match index {
        Ok(index) => {
          let mut collapsed_results: Vec<String> = Vec::new();
          let (src, parq) = index;
          collapsed_results.push(String::from(src));
          collapsed_results.push(String::from(parq));
          return r!(collapsed_results)
        },
        Err(_e) => return r!(NULL),
    }
}


/// perform an index of multiple fastq entry metadata
/// @export
#[extendr]
fn index_fastq_list(file_list: &[Rstr], dir: &str, threads: u8) -> extendr_api::Robj {
  let xlist: Vec<&str> = file_list.iter().map(|x: &Rstr| x.as_str()).collect();
  let x = fq_threaded::index_fastq_list(xlist, dir, threads);

  let mut collapsed_results: Vec<String> = Vec::new();
  for i in x.iter() {
    let (src, parq) = i;
    collapsed_results.push(String::from(src));
    collapsed_results.push(String::from(parq));
  }
  return r!(collapsed_results);
}


/// get the information on what has already been parsed within the working folder
/// @export
#[extendr]
fn get_indexed_tuples(dir: &str) -> extendr_api::Robj {
  let fq_pairs = crate::filehandlers::load_fq_index_pairs(dir);
  let mut collapsed_results: Vec<String> = Vec::new();
  for i in fq_pairs.iter() {
    let (src, parq) = i;
    collapsed_results.push(String::from(src));
    collapsed_results.push(String::from(parq));
  }
  return r!(collapsed_results);
}


/* 
/// Prepare an arrow file from the parquet elements in current directory; return information
/// as to whether the parquet universe is up-to-date. 
/// 
/// boolean result; true means that new content has been indexed / merged
/// @export
#[extendr]
fn form_arrow(dir: &str) -> extendr_api::Robj {
  return r!(arrow::prepare_arrow(dir));
}
*/

/* 
/// Get the path for the monolithic arrow file
/// @export
#[extendr]
fn get_arrow_path(dir: &str) -> extendr_api::Robj {
  //return Rstr::from_string(arrow::get_arrow_path(dir).as_os_str().to_str().unwrap());
  return r!(arrow::get_arrow_path(dir).as_os_str().to_str().unwrap());
}
*/

/// calculate mean quality score from an ASCII quality string 
/// @export
#[extendr]
fn get_qscore(qualstr: &str) -> extendr_api::Robj {
  return r!(fastq::u8_to_mean_q(qualstr.as_bytes()));
}


/// calculate mean quality score from a vector of phred scores
/// @export
#[extendr]
fn get_mean_qscore(phred_scores: &[f64]) -> extendr_api::Robj {
  let basequals: Vec<f64> = phred_scores.iter().map(|q| 10_f64.powf(q / -10_f64)).collect::<Vec<f64>>();
  return r!(-10_f32 * (basequals.iter().sum::<f64>() as f32 / basequals.len() as f32).log10());
}


// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rtqc;
    fn index_fastq;
    fn index_fastq_list;
    // fn form_arrow;
    // fn get_arrow_path;
    fn get_qscore;
    fn get_mean_qscore;
    fn get_indexed_tuples;
}
