use extendr_api::prelude::*;

mod fastq;
mod fq_threaded;


/// perform an index of fastq entry metadata
/// @export
#[extendr]
fn index_fastq(fq_path: &str, dir: &str) -> extendr_api::Robj {

    let index = fastq::index_fastq(fq_path, dir);

    match index {
        Ok(index) => return r!(Some(index)),
        Err(_e) => return r!(NULL),
    }
}


/// perform an index of multiple fastq entry metadata
/// @export
#[extendr]
fn index_fastq_list(file_list: &[Rstr], dir: &str, threads: u8) {

  println!("requested threads {}", threads);
  let xlist: Vec<&str> = file_list.iter().map(|x: &Rstr| x.as_str()).collect();

  let _x = fq_threaded::index_fastq_list(xlist, dir);


}


// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rtqc;
    fn index_fastq;
    fn index_fastq_list;
}
