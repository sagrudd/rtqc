

#' report the N50 (or other value) of sequence lengths from a length vector
#'
#' @description
#' Trivial method to report the N50 from a sequence collection
#'
#' @param data - the vector of sequence lengths
#' @param n - the N value to present (0.5 by default)
#'
#' @export
n50calc = function(data, n = 0.5) {
  len_sorted <- sort(data, decreasing = TRUE)
  len_sorted[cumsum(len_sorted) >= sum(len_sorted)*n][1]
}
