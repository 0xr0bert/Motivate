package output

import java.io.File

import org.jfree.chart.plot.PlotOrientation
import org.jfree.chart.renderer.xy.XYShapeRenderer
import org.jfree.chart.{ChartFactory, ChartUtils, JFreeChart}
import org.jfree.data.xy.{XYSeries, XYSeriesCollection}

import scala.io.Source

/**
  * Generates the charts for the simulation
  */
object Charts {
  /**
    * Create an XYSeries, x - Day, y - column specified by columnID
    * @param file The CSV file
    * @param columnID The column number (starting from 0, 0 is day)
    * @param lineID The name for this series
    * @return The XYSeries for the data
    */
  def generateXYSeries(file: File, columnID: Short, lineID: String): XYSeries = {
    val series = new XYSeries(lineID)
    // The first 10 days are dropped to allow it to stabilise - line 1 is the header
    Source.fromFile(file).getLines().drop(11).foreach(line => {
      val data = line.split(',')
      series.add(data(0).toInt, data(columnID).toInt)
    })
    series
  }

  /**
    * Generates an XYLineChart for a dataset
    * @param dataset The XYSeriesCollection
    * @param title The graph title
    * @param xHeader The title for the x-axis
    * @param yHeader The title for the y-axis
    * @return The generated chart
    */
  def generateXYLineChart(dataset: XYSeriesCollection,
                          title: String,
                          xHeader: String,
                          yHeader: String): JFreeChart = ChartFactory.createXYLineChart(
    title, xHeader, yHeader, dataset, PlotOrientation.VERTICAL, true, true, false
  )

  /**
    * Gets all CSV files in a directory
    * @param dir The directory to search
    * @return All CSV files found
    */
  def getCSVFilesInDir(dir: File) : Array[File] =
    dir.listFiles().filter(file => file.getName.contains(".csv") && file.isFile)

  /**
    * For all runs of the simulation generate a graph
    * @param columnId The column ID
    * @param outputName The name of the output file
    * @param graphTitle The title of the graph
    * @param xHeader The title of the x-axis
    * @param yHeader The title of the y-axis
    */
  def generateGraphForAllSimulations(columnId: Short,
                                     outputName: String,
                                     graphTitle: String,
                                     xHeader: String,
                                     yHeader: String) : Unit = {
    // Get the CSV files in the output
    val outputDir = new File("output")
    val csvFiles = getCSVFilesInDir(outputDir)

    // The second part of this gets the simulation id
    val filesAndIds = csvFiles.map(file => file -> file.getName.split('_')(1).split('.')(0)).toMap

    // Generates the dataset
    val dataset = new XYSeriesCollection()
    filesAndIds.foreach {case (file, id) =>
      dataset.addSeries(generateXYSeries(file, columnId, id))}

    val chart = generateXYLineChart(dataset, graphTitle, xHeader, yHeader)
    // Bound the graph by the data with a margin
    val range = new XYShapeRenderer().findRangeBounds(dataset)
    chart.getXYPlot.getRangeAxis.setRangeWithMargins(range)

    // Save the graph as a JPEG
    val width = 1280
    val height = 960
    val lineChart = new File(s"output/$outputName")
    ChartUtils.saveChartAsJPEG(lineChart, chart, width, height)
  }
}
