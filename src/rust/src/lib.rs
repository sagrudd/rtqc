use extendr_api::prelude::*;

mod fastq;


/// perform an index of fastq entry metadata
/// @export
#[extendr]
fn index_fastq(fq_path: &str, dir: &str) -> extendr_api::Robj {
    let index = fastq::index_fastq(fq_path, dir);

    match index {
        Some(index) => return r!(Some(index)),
        None => return r!(NULL),
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rtqc;
    fn index_fastq;
}
