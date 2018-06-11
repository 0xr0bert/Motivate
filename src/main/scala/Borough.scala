import java.io.{File, PrintWriter}

object Borough {
  var residents: Set[Agent] = Set[Agent]()
  val totalYears: Int = 5
  val numberOfPeople: Int = 150000

  /**
    * Determines whether a given day is a weekday
    * @param day the day number
    * @return true if the day is a weekday
    */
  def weekday (day: Int): Boolean = day % 7 < 4

  /**
    * Gets the season for a given day
    * @param day the day number
    * @return the Season for that day
    */
  def season (day: Int): Season = day match {
    case x if ((x % 365) >= 0 && (x % 365) < 59) || ((x % 365) >= 334 && (x % 365) < 365) => Winter
    case x if (x % 365) >= 59 && (x % 365) < 151 => Spring
    case x if (x % 365) >= 151 && (x % 365) < 243 => Summer
    case x if (x % 365) >= 243 && (x % 365) < 334 => Autumn
  }

  def main(args: Array[String]): Unit = {
    // Used for monitoring running-time
    val t0 = System.nanoTime()
    setUp()
    println("Agents created")

    // Output to a CSV
    val writer = new PrintWriter(new File("output.csv"))
    // Header row for the csv
    writer.println("Day,ActiveMode,ActiveNorm")

    // Record info for day 0
    val firstActive = countActive()
    writer.println(s"0,${firstActive._1},${firstActive._2}")

    // Start in winter (1st Jan)
    var weather = if (scala.util.Random.nextFloat() > Winter.percentageBadWeather) Good else Bad

    for(i <- 1 to totalYears * 365) {
      if (weekday(i)) {
        // Get the current season, and work out the weather for the season
        val currentSeason = season(i)
        val randomFloat = scala.util.Random.nextFloat()
        val newWeather = if (randomFloat > currentSeason.percentageBadWeather) Good else Bad

        // For each resident choose their transport mode
        residents.foreach(_.chooseMode(newWeather, weather != newWeather))
        weather = newWeather

        // For each resident, update their norm
        residents.foreach(_.updateNorm())

        // Count the number of active commutes, and write them to the csv file
        val active = countActive()
        writer.println(s"$i,${active._1},${active._2}")
      }
    }

    // Close the file
    writer.close()
    val t1 = System.nanoTime()
    println(s"Elapsed time ${t1 - t0}ns")
  }

  /**
    * Generates the agents, allocating them to neighbourhoods, social networks, subcultures
    */
  def setUp(): Unit = {
    for (i <- 1 to 150000) {
      // TODO: Generate agents
    }
  }

  /**
    * Counts the number of commutes that were active (Walking or Cycling)
    * @return a tuple (Number of active commutes, Number of agents who's norm is active)
    */
  def countActive(): (Int, Int) = {
    (residents.count((a: Agent) => a.currentMode == Walk || a.currentMode == Cycle),
    residents.count((a: Agent) => a.norm == Walk || a.norm == Cycle))
  }
}
