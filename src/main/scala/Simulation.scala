import java.time.Instant

import output.Charts

object Simulation {
  val totalYears = 3
  val numberOfPeople = 30000
  val numberOfSimulationsPerScenario = 4
  val socialConnectivity = 0.7f
  val subcultureConnectivity = 0.5f
  val neighbourConnectivity = 0.3f
  val numberOfSocialLinks = 10
  val numberOfNeighbourLinks = 10
  val daysInHabitAverage = 30
  val scenarios: Map[String, Scenario] = Map(
    "pre intervention" -> Scenario(Set(
      Neighbourhood(Map(Car -> 0.9f, Cycle -> 0.7f, Walk -> 0.8f, PublicTransport -> 0.9f)),
      Neighbourhood(Map(Car -> 0.5f, Cycle -> 0.7f, Walk -> 0.8f, PublicTransport -> 1.0f)),
      Neighbourhood(Map(Car -> 0.9f, Cycle -> 0.2f, Walk -> 0.6f, PublicTransport -> 0.5f)),
      Neighbourhood(Map(Car -> 0.2f, Cycle -> 0.9f, Walk -> 0.9f, PublicTransport -> 0.9f))
    )),
    "post intervention" -> Scenario(Set(
      Neighbourhood(Map(Car -> 0.9f, Cycle -> 0.7f, Walk -> 0.8f, PublicTransport -> 0.9f)),
      Neighbourhood(Map(Car -> 0.5f, Cycle -> 0.7f, Walk -> 0.8f, PublicTransport -> 1.0f)),
      Neighbourhood(Map(Car -> 0.7f, Cycle -> 0.8f, Walk -> 0.6f, PublicTransport -> 0.5f)), // this line is changed
      Neighbourhood(Map(Car -> 0.2f, Cycle -> 0.9f, Walk -> 0.9f, PublicTransport -> 0.9f))
    ))
  )

  def main(args: Array[String]): Unit = {
    val t0 = Instant.now().getEpochSecond

    val weatherPattern = (0 to totalYears * 365).map(i =>
      i -> {
        val currentSeason = season(i)
        val randomFloat = scala.util.Random.nextFloat()
        if (randomFloat > currentSeason.percentageBadWeather) Good else Bad }
    ).toMap

    val boroughs = for ((scenarioID, scenario) <- scenarios; i <- 1 to numberOfSimulationsPerScenario) yield new Borough(
      id = s"$scenarioID-$i",
      scenario = scenario,
      totalYears = totalYears,
      numberOfPeople = numberOfPeople,
      socialConnectivity = socialConnectivity,
      subcultureConnectivity = subcultureConnectivity,
      neighbourhoodConnectivity = neighbourConnectivity,
      numberOfSocialNetworkLinks = numberOfSocialLinks,
      numberOfNeighbourLinks = numberOfNeighbourLinks,
      daysInHabitAverage = daysInHabitAverage,
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
