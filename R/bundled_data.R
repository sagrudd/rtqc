
#' extract the path from a bundled file
#'
#' A collection of files are distributed with the rtqc package; this
#' accessory method is aimed to abbreviate the technical code for the
#' identification of these files and to clean the vignettes and code examples.
#'
#' @param file a known filename (or its prefix)
#'
#' @return a file
#'
#' @export
get_bundled_path <- function(file) {
  datadir <- file.path(system.file(package = "rtqc"), "extdata")
  return(
    file.path(datadir, list.files(datadir, pattern = file)[1])
  )
}
