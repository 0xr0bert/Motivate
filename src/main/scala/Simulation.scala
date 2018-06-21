import java.time.Instant

import output.Charts

object Simulation {
  val totalYears = 1
  val numberOfPeople = 30000
  val numberOfSimulations = 5
  val socialConnectivity = 0.7f
  val subcultureConnectivity = 0.5f
  val neighbourConnectivity = 0.3f
  val numberOfSocialLinks = 10
  val numberOfNeighbourLinks = 10

  def main(args: Array[String]): Unit = {
    val t0 = Instant.now().getEpochSecond

    val weatherPattern = (0 to totalYears * 365).map(i =>
      i -> {
        val currentSeason = season(i)
        val randomFloat = scala.util.Random.nextFloat()
        if (randomFloat > currentSeason.percentageBadWeather) Good else Bad }
    ).toMap

    val boroughs = for (i <- 1 to numberOfSimulations) yield new Borough(
      id = i,
      totalYears = totalYears,
      numberOfPeople = numberOfPeople,
      socialConnectivity = socialConnectivity,
      subcultureConnectivity = subcultureConnectivity,
      neighbourhoodConnectivity = neighbourConnectivity,
      numberOfSocialNetworkLinks = numberOfSocialLinks,
      numberOfNeighbourLinks = numberOfNeighbourLinks,
      weatherPattern = weatherPattern
    )

    boroughs.par.foreach(_.run())

    System.out.println("Generating graphs")
    Charts.generateGraphForAllSimulations(1, "ActiveMode.jpg", "Active Mode", "Day", "Number of people with an active mode")
    Charts.generateGraphForAllSimulations(2, "ActiveModeCounterToInactiveNorm.jpg", "Active Mode Counter To Inactive Norm", "Day", "Number of people with an active mode counter to inactive norm")
    Charts.generateGraphForAllSimulations(3, "InactiveModeCounterToActiveNorm.jpg", "Inactive Mode Counter to Active Norm", "Day", "Number of people with an inactive mode counter to active norm")
    Charts.generateGraphForAllSimulations(4, "ActiveNorm.jpg", "Active Norm", "Day", "Number of people with an active norm")


    val t1 = Instant.now().getEpochSecond
    System.out.println(s"TOTAL RUNNING TIME: ${t1 - t0}s")
  }

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
}
