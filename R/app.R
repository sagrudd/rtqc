library(shiny)
library(ipc)
library(future)
plan(multisession)

# The expectation here is that this Shiny application is called directly with
# a path that is either "complete" or still generating data - there is a
# possibility that the indexing of the sequence content will be performed by
# an accessory nextflow project; this R package should also have the abilities
# to alone do what Nextflow may also be doing ...


myApp <- function(path, ...) {

  seq_summary <- sequence_set_summary$new(path, threads=2)
  untouched <- TRUE

  # Define UI for application that draws a histogram
  ui <- fluidPage(

      # Application title
      titlePanel("RTQC - realtime review of nanopore sequence data"),

      # Sidebar with a slider input for number of bins
      sidebarLayout(
          sidebarPanel(
              sliderInput("bins",
                          "Number of bins:",
                          min = 1,
                          max = 50,
                          value = 30)
          ),

          # Show a plot of the generated distribution
          mainPanel(
             plotOutput("distPlot")
          )
      )
  )

  # Define server logic required to draw a histogram
  server <- function(input, output) {

      queue <- shinyQueue()
      queue$consumer$start(100) # Execute signals every 100 milliseconds

      reads_processed <- reactiveVal(value="0")
      files_processed <- reactiveVal(value="0")
      bases_processed <- reactiveVal(value="0")
      mean_length <- reactiveVal(value="0")
      mean_quality <- reactiveVal(value="0")

      future({

        while (1) {

          # result <- data.frame(count=i)
          # change value

          if (seq_summary$get_sequence_set()$sync() || untouched) {
            untouched <- FALSE
          facets <- seq_summary$shiny_touch()
          if (is.list(facets)) {

            queue$producer$fireAssignReactive("files_processed", facets$files)
            queue$producer$fireAssignReactive("reads_processed", facets$reads)
            queue$producer$fireAssignReactive("bases_processed", facets$bases$str)
            queue$producer$fireAssignReactive("mean_length", facets$length)
            queue$producer$fireAssignReactive("mean_quality", facets$quality)
          }
          }
          Sys.sleep(1)
        }
      })

      output$distPlot <- renderPlot({
        ii <- rtqc::Infographic$new()
        ii$columns <- 5
        ii$add(InfographicItem$new(key="fastq files", value=req(files_processed()), icon="fa-copy"))
        ii$add(InfographicItem$new(key="read count", value=req(reads_processed()), icon="fa-align-left"))
        ii$add(InfographicItem$new(key="bases", value=req(bases_processed()), icon="fa-arrow-up"))
        ii$add(InfographicItem$new(key="mean length", value=req(mean_length()), icon="fa-minus"))
        ii$add(InfographicItem$new(key="mean quality", value=req(mean_quality()), icon="fa-search"))
        ii$shinyplot()
      }
      )
  }

  # Run the application
  shinyApp(ui = ui, server = server)
}
