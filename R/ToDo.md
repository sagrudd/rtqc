0.1.0 -
  [ ] multi-flowcell data - dashboards aggregating & displaying multiple runs
      [ ] decision making on where information can be best shared?
          [ ] S3 bucket
          [ ] ODBC? MySQL / PostgreSQL
          [ ] EPI2ME Labs folder ...
0.0.5 -
  [ ] Ingress of FASTQ files by barcoded folder structure
  [ ] Ingress of FASTQ files by sample sheet
0.0.4 -
  [ ] Export of FASTQ in "real time" using time stamp information; 
      speed-up/slow-down
0.0.3 -
  [ ] Export of FASTQ ordered by position in sequence_set 
  [ ] Implementation of filters to remove reads from sequence_set by characteristic
  [ ] Testing framework and robust breaking unit tests
0.0.2 -
  [ ] Shiny server / infrastructure to reflect evolving sequence_set
  [ ] presentation of QC plots and summary statistics
  [ ] Inclusion of calculated metrics in the rtqc tibble
  [ ] Implementation of dynamic filters to update view of sequence_set
      [ ] Filter by min-length / max-length
      [ ] Filter by min-quality / max-quality
0.0.1 -
  [x] Ingress of FASTQ files (not barcoded)
  [x] Ingress implemented a background/parallel process
  [x] Creation of sequence_set object
  [x] Sequence_set object updated on a file-by-file ingress basis
    parquet format
  
