rextendr::document()


dorado_fastq_path <- rtqc::get_bundled_path("fastq_pass")
single_fastq <- file.path(
  dorado_fastq_path, list.files(dorado_fastq_path)[1])

bf <- basecalled_folder$new(dorado_fastq_path)
bf$index(threads=1)
bf$status()

bf$status()

bf$status()

seqset <- bf$as_sequence_set()
seqset$sync()
