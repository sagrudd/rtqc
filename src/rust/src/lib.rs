use extendr_api::prelude::*;

mod fastq;

#[extendr]
fn index_fastq(fq_path: &str) -> extendr_api::Robj {
    let index = fastq::index_fastq(fq_path);

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
