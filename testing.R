rextendr::document()



bf <- rtqc::basecalled_folder$new(rtqc::get_bundled_path("fastq_pass"))
bf$index(threads=1)
summary <- bf$as_sequence_set()$as_summary()
summary$touch()


summary2 <- rtqc::sequence_set_summary$new(rtqc::get_bundled_path("fastq_pass"))
summary2$touch()
summary2$read_bases()
summary2$length_highlights()


dorado_fastq_path <- rtqc::get_bundled_path("barcoded_pass")
file_list <- list.files(dorado_fastq_path,
recursive = TRUE,
pattern = "fq$|fq.gz$|fastq$|fastq.gz$",
ignore.case = TRUE)

bf <- basecalled_folder$new(rtqc::get_bundled_path("barcoded_pass"))
