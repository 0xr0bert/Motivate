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

    val boroughs = for (i <- 1 to numberOfSimulations) yield new Borough(
      id = i,
      totalYears = totalYears,
      numberOfPeople = numberOfPeople,
      socialConnectivity = socialConnectivity,
      subcultureConnectivity = subcultureConnectivity,
      neighbourhoodConnectivity = neighbourConnectivity,
      numberOfSocialNetworkLinks = numberOfSocialLinks,
      numberOfNeighbourLinks = numberOfNeighbourLinks
    )

    boroughs.par.foreach(_.run())

    System.out.println("Generating graphs")
    Charts.generateGraphForAllSimulations(1, "ActiveMode.jpg", "Active Mode", "Day", "Number of people with an active mode")
    Charts.generateGraphForAllSimulations(2, "ActiveModeCounterToInactiveNorm.jpg", "Active Mode Counter To Inactive Norm", "Day", "Number of people with an active mode counter to inactive norm")
    Charts.generateGraphForAllSimulations(3, "InactiveModeCounterToActiveNorm.jpg", "Inactive Mode Counter to Inactive Norm", "Day", "Number of people with an inactive mode counter to active norm")
    Charts.generateGraphForAllSimulations(4, "ActiveNorm.jpg", "Active Norm", "Day", "Number of people with an active norm")


    val t1 = Instant.now().getEpochSecond
    System.out.println(s"TOTAL RUNNING TIME: ${t1 - t0}s")
  }
}
