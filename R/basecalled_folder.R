
#' R6 Class for handling folders of basecalled sequence from dorado or MinKNOW
#'
#' @description
#' `basecalled_folder` class provides an infrastructure for working with
#' the output from sequence basecalling.
#'
#' @import R6
#'
#' @export
basecalled_folder <- R6::R6Class(
  classname = "basecalled_folder",

  public = list(

    #' @description
        #' constructor to initialise an instance of the basecalled_folder
        #' class
        #'
        #' @param seq_path - the sequence path to investigate
    initialize = function(seq_path) {
      private$seq_path <- seq_path
      private$sequence_scan()
    },

    #' @description
    #' this print method overrides the standard print function as included with
    #' R6 objects - this is to better define what is contained within the object
    #' and to provide better `rtqc` abstraction
    #'
    #' @param ... additional stuff passed on
    #'
    #' @return nothing (at present) - output to stdout
    print = function(...) {
      cat(paste0("<rtqc::", class(self)[1], ">\n",
                 "\tseq_path=", private$seq_path, "\n",
                 "\tbarcoded_data=", private$barcoded, "\n",
                 "\tbarcode_count=", private$barcode_count, "\n",
                 "\tfile_format=", private$file_format, "\n",
                 "\tfile_count=", private$file_count))
    }
  ),

  private = list(
    seq_path = NULL,
    barcoded = FALSE,
    file_format = NULL,
    file_count = 0,
    barcode_count = 0,

    #' evaluates the defined private$seq_path for sequence files that are the
    #' result of a base calling analysis using either MinKNOW or Dorado. This
    #' method will support FASTQ, SAM and BAM file formats.
    sequence_scan = function() {
      print("sequence_scan::")

      files <- list.files(dorado_fastq_path,
                          recursive = TRUE,
                          pattern = "fq$|fq.gz$|fastq$|fastq.gz$",
                          ignore.case = TRUE)

      # question one - are the sequences all FASTQ / BAM / SAM
      ffiles <- factor(rep("UNKNOWN", length(files)),
                       levels = c("UNKNOWN", "FASTQ", "BAM", "SAM"))
      ffiles <- ffiles[
        grepl("fq$|fq.gz$|fastq$|fastq.gz$", files, ignore.case = TRUE)] <-
        "FASTQ"
      length(unique(ffiles))

      # question two - is the directory depth for sequences homogeneous?
      depths <- unlist(
        lapply(files, function(x) {
          length(unlist(strsplit(x, .Platform$file.sep)))
        }))

      udepth <- unique(depths)
      # split read files into barcode groups
    }
  )
)
