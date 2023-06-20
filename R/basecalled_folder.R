
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
        #' @param use_unclassified - when using demultiplexed datasets, include
        #' the unclassified sequences as a valid "barcode"
    initialize = function(seq_path,
                          use_unclassified = private$.use_unclassified) {
      private$.use_unclassified <- use_unclassified
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
                 "\tfile_count=", private$file_count, "\n",
                 "\tvalid=", private$valid))
    },

    #' @description
    #' the index method will index the available sequence files to recover
    #' an object that is not entirely dissimilar to the original
    #' sequencing_summary file that was produced by the guppy software
    #'
    #' @param threads the number of threads to run in parallel
    index = function(threads = 2) {
      index_fastq_list(file.path(private$seq_path, private$.file_list), tempdir(), threads)
    }
  ),

  private = list(
    .use_unclassified = FALSE,
    .sample_sheet = NULL,
    .file_list = NULL,
    seq_path = NULL,
    barcoded = FALSE,
    file_format = NULL,
    file_count = 0,
    barcode_count = 0,
    valid = FALSE,

    #' evaluates the defined private$seq_path for sequence files that are the
    #' result of a base calling analysis using either MinKNOW or Dorado. This
    #' method will support FASTQ, SAM and BAM file formats.
    sequence_scan = function() {
      private$.file_list <- list.files(private$seq_path,
                          recursive = TRUE,
                          pattern = "fq$|fq.gz$|fastq$|fastq.gz$",
                          ignore.case = TRUE)

      # question one - are the sequences all FASTQ / BAM / SAM
      ffiles <- factor(rep("UNKNOWN", length(private$.file_list)),
                       levels = c("UNKNOWN", "FASTQ", "BAM", "SAM"))
      ffiles <- ffiles[
        grepl("fq$|fq.gz$|fastq$|fastq.gz$", private$.file_list,
              ignore.case = TRUE)] <- "FASTQ"
      ftypes <- length(unique(ffiles))

      # question two - is the directory depth for sequences homogeneous?
      depths <- unlist(
        lapply(private$.file_list, function(x) {
          length(unlist(strsplit(x, .Platform$file.sep)))
        }))
      udepth <- unique(depths)

      # split read files into barcode groups
      if (length(udepth) == 1 && udepth <= 2 && ftypes == 1) {
        if (udepth == 1) {
          private$valid <- TRUE
          private$file_format <- unique(ffiles)
          private$file_count <- length(private$.file_list)
        } else if (udepth == 2) {
          fframe <- do.call(rbind, strsplit(private$.file_list,
                                            .Platform$file.sep))
          if (is.null(private$.sample_sheet)) {
            # unclassified is off by default
            if (!private$.use_unclassified) {
              rm_unclassified <- which(fframe[, 1] == "unclassified")
              fframe <- fframe[-rm_unclassified, ]
              private$.file_list <- private$.file_list[-rm_unclassified]
            }
            retain_barcodes <- which(grepl("^barcode[0-9]+$|unclassified",
                                           fframe[, 1]))
            fframe <- fframe[retain_barcodes, ]
            private$.file_list <- private$.file_list[retain_barcodes]
            if (length(unique(fframe[, 1])) >= 1) {
              private$valid <- TRUE
              private$barcoded <- TRUE
              private$file_format <- unique(ffiles)
              private$file_count <- nrow(fframe)
              private$barcode_count <- length(unique(fframe[, 1]))
            }
          }
        }
      }
    }
  )
)
