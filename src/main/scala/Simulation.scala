import java.time.Instant

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

    val boroughs = (1 to numberOfSimulations).par.map(i => new Borough(
      id = i,
      totalYears = totalYears,
      numberOfPeople = numberOfPeople,
      socialConnectivity = socialConnectivity,
      subcultureConnectivity = subcultureConnectivity,
      neighbourhoodConnectivity = neighbourConnectivity,
      numberOfSocialNetworkLinks = numberOfSocialLinks,
      numberOfNeighbourLinks = numberOfNeighbourLinks
    ))

    boroughs.foreach(_.run())

    val t1 = Instant.now().getEpochSecond
    System.out.println(s"TOTAL RUNNING TIME: ${t1 - t0}s")
  }
}
