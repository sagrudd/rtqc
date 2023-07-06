library(shiny)

myApp <- function(...) {

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

      # output$distPlot <- renderPlot({
      #     # generate bins based on input$bins from ui.R
      #     x    <- faithful[, 2]
      #     bins <- seq(min(x), max(x), length.out = input$bins + 1)
      #
      #     # draw the histogram with the specified number of bins
      #     hist(x, breaks = bins, col = 'darkgray', border = 'white',
      #          xlab = 'Waiting time to next eruption (in mins)',
      #          main = 'Histogram of waiting times')
      # })


      output$distPlot <- renderPlot({
        ii <- rtqc::Infographic$new()

        ii$add(InfographicItem$new(key="read count", value=input$bins, icon="fa-apple"))
        ii$add(InfographicItem$new(key="bases", value="142", icon="fa-barcode"))
        ii$shinyplot()
      }
      )
  }

  # Run the application
  shinyApp(ui = ui, server = server)
}
