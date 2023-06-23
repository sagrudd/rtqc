use extendr_api::prelude::*;

mod fastq;
mod fq_threaded;
mod filehandlers;
mod arrow;

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


/// Prepare an arrow file from the parquet elements in current directory 
/// @export
#[extendr]
fn form_arrow(dir: &str) {
  arrow::prepare_arrow(dir);
}


// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rtqc;
    fn index_fastq;
    fn index_fastq_list;
    fn form_arrow;
}
