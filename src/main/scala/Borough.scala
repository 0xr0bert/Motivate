import java.io.{File, PrintWriter}

import scala.collection.mutable

import scalaz._
import Scalaz._

class Borough (val id: Int,
               val totalYears: Int,
               val numberOfPeople: Int,
               val socialConnectivity: Float,
               val subcultureConnectivity: Float,
               val neighbourhoodConnectivity: Float,
               val numberOfSocialNetworkLinks: Int,
               val numberOfNeighbourLinks: Int) extends Runnable {
  var residents: Set[Agent] = Set[Agent]()

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

  def run(): Unit = {
    // Used for monitoring running-time
    val t0 = System.nanoTime()
    setUp()
    println(s"[$id] Agents created")

    // Output to a CSV
    val writer = new PrintWriter(new File(s"output_$id.csv"))
    // Header row for the csv
    writer.println("Day,ActiveMode,ActiveModeCounterToInactiveNorm,InactiveModeCounterToActiveNorm,ActiveNorm,Rain")

    // Start in winter (1st Jan)
    var weather = if (scala.util.Random.nextFloat() > Winter.percentageBadWeather) Good else Bad

    // Record info for day 0
    val firstStats = countStats()
    writer.println(s"0,${firstStats._1},${firstStats._2},${firstStats._3},${firstStats._4},${if (weather == Bad) 1 else 0}")

    for(i <- 1 to totalYears * 365) {
      if (weekday(i)) {
        println(s"[$id] Day: $i")
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
        val stats = countStats()
        writer.println(s"$i,${stats._1},${stats._2},${stats._3},${stats._4},${if (weather == Bad) 1 else 0}")
      }
    }

    // Close the file
    writer.close()
    val t1 = System.nanoTime()
    println(s"[$id] Elapsed time ${t1 - t0}ns")
  }

  /**
    * Generates the agents, allocating them to neighbourhoods, social networks, subcultures
    */
  def setUp(): Unit = {
    var agents: mutable.HashSet[Agent] = mutable.HashSet()
    for (i <- 1 to numberOfPeople) {
      val perceivedEffort: Map[JourneyType, Map[TransportMode, Float]] = Map(
        LocalCommute -> Map(
          Walk -> bound(0, 1, randomNormal(0.2, 0.1)).toFloat,
          Cycle -> bound(0, 1, randomNormal(0.2, 0.3)).toFloat,
          PublicTransport -> bound(0, 1, randomNormal(0.2, 0.2)).toFloat,
          Car -> bound(0, 1, randomNormal(0.2, 0.2)).toFloat
        ),
        CityCommute -> Map(
          Walk -> bound(0, 1, randomNormal(0.7, 0.1)).toFloat,
          Cycle -> bound(0, 1, randomNormal(0.5, 0.3)).toFloat,
          PublicTransport -> bound(0, 1, randomNormal(0.2, 0.2)).toFloat,
          Car -> bound(0, 1, randomNormal(0.2, 0.2)).toFloat
        ),
        DistantCommute -> Map(
          Walk -> 1.0f,
          Cycle -> 1.0f,
          PublicTransport -> bound(0, 1, randomNormal(0.2, 0.2)).toFloat,
          Car -> bound(0, 1, randomNormal(0.2, 0.2)).toFloat
        )
      )

      val subculture = chooseSubculture()
      val neighbourhood = chooseNeighbourhood()
      val journeyType = chooseJourneyType()
      val weatherSensitivity = scala.util.Random.nextFloat()
      val autonomy = scala.util.Random.nextFloat()
      val consistency = scala.util.Random.nextFloat()
      val suggestibility = randomNormal(1.0f, 0.25f).toFloat
      val currentMode = chooseInitialNormAndHabit(
        subculture, subcultureConnectivity, suggestibility, journeyType, perceivedEffort
      )
      val norm = currentMode
      val habit = currentMode

      agents.add(new Agent(
        subculture = subculture,
        neighbourhood = neighbourhood,
        commuteLength = journeyType,
        perceivedEffort = perceivedEffort,
        weatherSensitivity = weatherSensitivity,
        autonomy = autonomy,
        consistency = consistency,
        suggestibility = suggestibility,
        socialConnectivity = socialConnectivity,
        subcultureConnectivity = subcultureConnectivity,
        neighbourhoodConnectivity = neighbourhoodConnectivity,
        currentMode = currentMode,
        habit = habit,
        norm = norm
      ))
      println(s"[$id] Agent: [$i] generated")
    }
    linkAgents(agents, numberOfSocialNetworkLinks, _.socialNetwork)

    val neighbourhoodsToAgents: Map[Neighbourhood, mutable.HashSet[Agent]] = agents.groupBy(_.neighbourhood)
    for ((_, localAgents) <- neighbourhoodsToAgents) linkAgents(localAgents, numberOfNeighbourLinks, _.neighbours)
    residents = agents.toSet
  }

  def chooseSubculture(): Subculture = {
    val x = scala.util.Random.nextFloat()
    if (x <= 1) {
      SubcultureA
    } else {
      SubcultureA
    }
  }

  def chooseNeighbourhood(): Neighbourhood = {
    val x = scala.util.Random.nextFloat()
    if (x <= 1) {
      NeighbourhoodOne
    } else {
      NeighbourhoodOne
    }
  }

  def chooseJourneyType(): JourneyType = {
    val x = scala.util.Random.nextFloat()
    if (x <= 0.33) {
      LocalCommute
    } else if (x <= 0.66) {
      CityCommute
    } else {
      DistantCommute
    }
  }

  /*
   * intial = (subculture * (subcultureConnectivity * suggestibility)) * effort
   * TODO: Currently everyone has an active norm, there should maybe be a degree of randomness
   */
  def chooseInitialNormAndHabit(
                               subculture: Subculture,
                               subcultureConnectivity: Float,
                               suggestibility: Float,
                               commuteLength: JourneyType,
                               perceivedEffort: Map[JourneyType, Map[TransportMode, Float]]
                               ): TransportMode = {
    val subcultureWeight = subcultureConnectivity * suggestibility
    val subcultureDesirabilityWeighted: Map[TransportMode, Float] = subculture.desirability.mapValues(x => x * subcultureWeight)
    val effortForJourneyInversed: Map[TransportMode, Float] = perceivedEffort(commuteLength).mapValues(x => 1.0f - x)
    subcultureDesirabilityWeighted.unionWith(effortForJourneyInversed)(_ * _).maxBy(_._2)._1
  }


  /**
    * Links agents within a network randomly
    *
    * This currently does not terminate
    *
    * @param agents All the agents
    * @param n The number of links a given agent may have
    * @param network The network to link
    */
  def linkAgents(agents: mutable.HashSet[Agent], n: Int, network: Agent => mutable.Set[Agent]): Unit = {
    var unlinkedAgents = agents.clone()

    while (unlinkedAgents.size > 5) {
      if (unlinkedAgents.size < 5) {
        println("")
      }

      val r0 = scala.util.Random.nextInt(unlinkedAgents.size)
      val r1 = scala.util.Random.nextInt(unlinkedAgents.size)

      if (r0 != r1) {
        val agent0 = unlinkedAgents.iterator.drop(r0).next()
        val agent1 = unlinkedAgents.iterator.drop(r1).next()

        network(agent0) += agent1
        network(agent1) += agent0
        if (network(agent0).size >= n) {
          unlinkedAgents.remove(agent0)
        }

        if (network(agent1).size >= n) {
          unlinkedAgents.remove(agent1)
        }
      }
    }
  }

  /**
    * Counts the statistics at the current day
    * @return a tuple (Number of active commutes, Number of active commutes counter to their (inactive) norm,
    *         Number of inactive commutes counter to their norm, Number of agents who's norm is active)
    */
  def countStats(): (Int, Int, Int, Int) = {
    (residents.count(a => a.currentMode == Walk || a.currentMode == Cycle),
      residents.count(a => (a.currentMode == Walk || a.currentMode == Cycle) && (a.norm != Walk && a.norm != Cycle)),
      residents.count(a => (a.currentMode != Walk && a.currentMode != Cycle) && (a.norm == Walk || a.norm == Cycle)),
      residents.count(a => a.norm == Walk || a.norm == Cycle))
  }

  def randomNormal(mean: Double, sd: Double): Double = (scala.util.Random.nextGaussian() * sd) + 1
  def bound(lowerBound: Double, upperBound: Double, x: Double): Double = Math.min(upperBound, Math.max(lowerBound, x))
}
