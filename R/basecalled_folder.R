
#' R6 Class for handling folders of basecalled sequence from dorado or MinKNOW
#'
#' @description
#' `basecalled_folder` class provides an infrastructure for working with
#' the output from sequence basecalling.
#'
#' @import R6
#' @import tibble
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
        #' @param cache_dir - where should intermediate files be stored; this
        #' will utilise a temporary directory by default
    initialize = function(seq_path,
                          use_unclassified = private$.use_unclassified,
                          cache_dir = tempdir()) {
      private$.use_unclassified <- use_unclassified
      private$seq_path <- seq_path
      private$cache_dir <- cache_dir
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
      if (private$check_indexing()) {
        private$index_process <- callr::r_bg(
          function(x, y, z) rtqc::index_fastq_list(x, y, z),
          args = list(file.path(private$seq_path, private$.file_list$files),
                      private$cache_dir,
                      threads)

        )
      }
    },

    #' @description
    #' prepares a `sequence_set` object from the current basecalled folder
    as_sequence_set = function() {
      return(rtqc::sequence_set$new(self))
    },

    #' @description
    #' a public accessory method to check on what is happening with the
    #' current `basecalled_folder`
    status = function(echo = FALSE) {
      # print some friendly information on the object ...

      # what is the state of the indexing process
      return(private$check_indexing(echo = echo))
    },


    get_cache_dir = function() {
      return(private$cache_dir)
    },

    is_new_sequence_data = function() {
      if (nrow(private$.file_list) < nrow(private$get_sequence_file_list())) {
        return(TRUE)
      }
      return(FALSE)
    },

    get_list = function() {
      return(private$get_sequence_file_list())
    }
  ),

  private = list(
    .use_unclassified = FALSE,
    .sample_sheet = NULL,
    .file_list = NULL,
    seq_path = NULL,
    barcode_count = 0,
    barcoded = FALSE,
    cache_dir = NULL,
    file_format = NULL,
    file_count = 0,
    index_process = NULL,
    valid = FALSE,


    get_sequence_file_list = function() {
      file_list <- list.files(private$seq_path,
                              recursive = TRUE,
                              pattern = "fq$|fq.gz$|fastq$|fastq.gz$",
                              ignore.case = TRUE)

      insufficient_readable <- simpleError("No files of defined format found")
      ambiguous_format <- simpleError("Ambiguous format; more than one possibility")
      depth_collision <- simpleError("There are sequence reads at more than one depth level")

      filetib <- tryCatch(
        {
          ffiles <- factor(rep("UNKNOWN", length(file_list)),
                           levels = c("UNKNOWN", "FASTQ", "BAM", "SAM"))
          ffiles[
            which(grepl("fq$|fq.gz$|fastq$|fastq.gz$", file_list,
                  ignore.case = TRUE))] <- "FASTQ"

          depths <- unlist(
            lapply(file_list, function(x) {
              length(unlist(strsplit(x, .Platform$file.sep)))
            }))
          barcode_assignment <- rep("unbarcoded", length(file_list))

          filetib <- tibble::tibble(files=file_list, barcode=barcode_assignment, types=ffiles, depths=depths)

          ftypes <- length(unique(filetib$types))

          if (ftypes == 0) stop(insufficient_readable)
          else if (ftypes > 1) stop(ambiguous_format)

          # question two - is the directory depth for sequences homogeneous?
          udepth <- unique(filetib$depths)
          if (length(udepth) != 1) stop(depth_collision)

          if (length(udepth) == 1 && udepth <= 2 && ftypes == 1) {
            if (udepth == 1) {
              private$valid <- TRUE
              private$file_format <- unique(ffiles)
              private$file_count <- length(private$.file_list)
            } else if (udepth == 2) {
              fframe <- do.call(rbind, strsplit(file_list,
                                                .Platform$file.sep))
              filetib$barcode <- fframe[,1]
              if (is.null(private$.sample_sheet)) {
                # unclassified is off by default
                if (!private$.use_unclassified) {
                  rm_unclassified <- which(filetib$barcode == "unclassified")
                  if (length(rm_unclassified) > 0) {
                    filetib <- filetib[-rm_unclassified,]
                  }
                }
                retain_barcodes <- which(grepl("^barcode[0-9]+$|unclassified",
                                               filetib$barcode))
                filetib <- filetib[retain_barcodes,]
                if (length(unique(filetib$barcode)) >= 1) {
                  private$valid <- TRUE
                  private$barcoded <- TRUE
                  private$file_format <- unique(ffiles)
                  private$file_count <- nrow(fframe)
                  private$barcode_count <- length(unique(filetib$barcode))
                }
              }
            }
          }
        },
        error = function(x) {
          return(x)
        },
        finally = {
          return(filetib)
        }
      )

      return(filetib)
    },


    #' evaluates the defined private$seq_path for sequence files that are the
    #' result of a base calling analysis using either MinKNOW or Dorado. This
    #' method will support FASTQ, SAM and BAM file formats.
    sequence_scan = function() {
      private$.file_list <- private$get_sequence_file_list()
    },

    check_indexing = function(echo = FALSE) {
      if (is.null(private$index_process)) {
        if (echo) cat(paste("Indexing process is not running", "\n"))
        return(TRUE)
      } else if (private$index_process$is_alive()) {
        if (echo) cat(paste("Indexing process is currently running", "\n"))
        return(FALSE)
      } else if (!private$index_process$is_alive()) {
        if (echo) cat(paste("Indexing process has completed", "\n"))
        private$index_process <- NULL
        return(TRUE)
      }
    }
  )
)
