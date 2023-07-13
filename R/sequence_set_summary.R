
#' R6 Class for reporting sequence data facets
#'
#' @description
#' `sequence_set_summary` class provides tools and widgets for the presentation
#' of sequence_set information
#'
#' @import R6
#'
#' @export
sequence_set_summary <- R6::R6Class(
  classname = "sequence_set_summary",

  public = list(
    #' @description
    #' Constructor method for `sequence_set_summary` requires just a
    #' sequence_set object that will provide the other file interaction
    #' capabilities
    #'
    #' @param sequence_set an existing sequence_set object
    initialize = function(path=NULL, sequence_set=NULL, threads=1) {

      if (!is.null(path)) {
        bf <- rtqc::basecalled_folder$new(path)
        bf$index(threads=threads)
        private$sequence_set <- bf$as_sequence_set()
      } else if (!is.null(sequence_set)) {
        private$sequence_set <- sequence_set
      }
    },


    shiny_touch = function() {
      result <- list()

      delta <- private$poke_parquet()

      if (delta || is.null(private$data)) {
        private$data <- private$sequence_set$data()

        result$files <- length(unique(private$data$file_of_origin))
        result$reads <- self$read_count()
        result$bases <- self$read_bases()
        result$length <- sprintf(mean(private$data$read), fmt = "%#.1f")
        result$quality <- sprintf(get_mean_qscore(private$data$quality), fmt = "%#.2f")
        return(result)
      }

      NULL
    },


    get_sequence_set = function() {
      return(private$sequence_set)
    },


    touch = function() {

      delta <- FALSE

      # are we up-to-date? Check first whether there are new files available
      if (private$sequence_set$get_bc_folder()$is_new_sequence_data()) {
        cat(paste0("sequence collection has new content", "\n"))
      }



      while (private$sequence_set$get_bc_folder()$status() == FALSE) {
        cat(paste0("sequence collection is being indexed - waiting", "\n"))
        Sys.sleep(0.5)
      }

      delta <- private$poke_parquet()

      if (delta || is.null(private$data)) {
        cat(paste0("reloading the stored data object", "\n"))
        private$data <- private$sequence_set$data()
      }

    },

    get_data = function() {
      return(private$data)
    },


    #' @description
    #' summary aggregates several pieces of information into a single call
    #' that are routinely shown in infographics and dashboards
    #'  - number of sequence reads
    #'  - number of bases read
    #'  - number of barcodes
    #'  - length min, max, mean, median
    #'  - quality min, max, mean, median
    #'  - flowcell id(s)
    #'  - earliest read timestamp
    #'  - latest read timestamp
    #'  - runtime duration
    #'  - active pores (as pores seen within e.g. 15 minutes of latest read)
    #'
    summary = function() {
      i <- 1
    },

    #' @description
    #' returns the number of sequence reads that are defined within the object
    read_count = function() {
      nrow(private$data)
    },

    #' @description
    #' returns a numeric defining the number of bases observed
    #'
    #' @param scale - the scale to use when reporting the number of bases; this
    #' can be Mb, Gb, Tb (at this moment) - or (auto) that will select an
    #' appropriate scale
    read_bases = function(scale="Gb", dpoints=2) {
      bases <- sum(private$data$read)
      if (toupper(scale) == "KB") {
        bases <- bases / 1000
      } else if (toupper(scale) == "MB") {
        bases <- bases / 1000000
      } else if (toupper(scale) == "GB") {
        bases <- bases / 1000000000
      } else if (toupper(scale) == "TB") {
        bases <- bases / 1000000000000
      }
      fstr <- sprintf("%%#.%df", dpoints)
      val <- sprintf(bases, fmt = fstr)
      list(res=val, raw=bases, unit=scale, str=paste0(val, " (", scale, ")"))
    },

    quality_highlights = function() {
      min_quality = 9
      max_quality = 15
      mean_quality = 10
      median_quality = 12

      return(1)
    },


    length_highlights = function() {
      list(
        shortest = min(private$data$read),
        longest = max(private$data$read),
        n50 = n50calc(private$data$read),
        n90 = n50calc(private$data$read, n=0.9),
        mean = mean(private$data$read),
        median = median(private$data$read)
      )
    },

    temporal_highlights = function(unit="hours") {
      earliest = 1
      latest = 2
      duration = 6
      t50 = 2 # hours from start of run
      t90 = 3

      return(1)
    },


    pore_highlights = function() {
      return(1)
    }

  ),

  private = list(
    ping = 1,
    sequence_set = NULL,
    data = NULL,

    poke_parquet = function() {
      if (private$sequence_set$sync()) {
        cat(paste0("sequence dataset has been updated ...", "\n"))
        return(TRUE)
      } else {
        return(FALSE)
      }
    }
  )
)
