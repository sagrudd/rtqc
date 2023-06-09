# Generated by extendr: Do not edit by hand

# nolint start

#
# This file was created with the following call:
#   .Call("wrap__make_rtqc_wrappers", use_symbols = TRUE, package_name = "rtqc")

#' @docType package
#' @usage NULL
#' @useDynLib rtqc, .registration = TRUE
NULL

#' perform an index of fastq entry metadata
#' @export
index_fastq <- function(fq_path, dir) .Call(wrap__index_fastq, fq_path, dir)

#' perform an index of multiple fastq entry metadata
#' @export
index_fastq_list <- function(file_list, dir, threads) .Call(wrap__index_fastq_list, file_list, dir, threads)

#' Prepare an arrow file from the parquet elements in current directory; return information
#' as to whether the parquet universe is up-to-date. 
#' 
#' boolean result; true means that new content has been indexed / merged
#' @export
form_arrow <- function(dir) .Call(wrap__form_arrow, dir)

#' Get the path for the monolithic arrow file
#' @export
get_arrow_path <- function(dir) .Call(wrap__get_arrow_path, dir)

#' calculate mean quality score from an ASCII quality string 
#' @export
get_qscore <- function(qualstr) .Call(wrap__get_qscore, qualstr)

#' calculate mean quality score from a vector of phred scores
#' @export
get_mean_qscore <- function(phred_scores) .Call(wrap__get_mean_qscore, phred_scores)

#' get the information on what has already been parsed within the working folder
#' @export
get_indexed_tuples <- function(dir) .Call(wrap__get_indexed_tuples, dir)


# nolint end
