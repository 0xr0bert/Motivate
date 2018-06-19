import java.io.{File, PrintWriter}
import java.time.Instant

import scala.collection.JavaConverters._
import scala.collection.mutable
import scalaz._
import Scalaz._
import org.apache.commons.math3.util.Pair
import org.apache.commons.math3.distribution.{EnumeratedDistribution, NormalDistribution}

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
    val t0 = Instant.now().getEpochSecond

    // Create the agents
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

    // The simulation starts running here
    for(i <- 1 to totalYears * 365) {

      // Only consider weekdays
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

    // Report the running time for this thread
    val t1 = Instant.now().getEpochSecond
    println(s"[$id] Elapsed time ${t1 - t0}s")
  }

  /**
    * Generates the agents, allocating them to neighbourhoods, social networks, subcultures
    */
  def setUp(): Unit = {
    // Create a mutable set of agents
    var agents: mutable.HashSet[Agent] = mutable.HashSet()

    // Create the agents
    for (i <- 1 to numberOfPeople) {
      /*
       * perceivedEffort should be between 0 and 1, and is bound as such.
       *
       * Active modes have a higher s.d. as people's perception of the effort will vary much more than say driving.
       */
      val perceivedEffort: Map[JourneyType, Map[TransportMode, Float]] = Map(
        LocalCommute -> Map(
          Walk -> bound(0, 1, randomNormal(0.2, 0.3)).toFloat,
          Cycle -> bound(0, 1, randomNormal(0.2, 0.3)).toFloat,
          PublicTransport -> bound(0, 1, randomNormal(0.2, 0.1)).toFloat,
          Car -> bound(0, 1, randomNormal(0.2, 0.1)).toFloat
        ),
        CityCommute -> Map(
          Walk -> bound(0, 1, randomNormal(0.7, 0.3)).toFloat,
          Cycle -> bound(0, 1, randomNormal(0.5, 0.3)).toFloat,
          PublicTransport -> bound(0, 1, randomNormal(0.2, 0.1)).toFloat,
          Car -> bound(0, 1, randomNormal(0.2, 0.1)).toFloat
        ),
        DistantCommute -> Map(
          Walk -> 1.0f,
          Cycle -> 1.0f,
          PublicTransport -> bound(0, 1, randomNormal(0.2, 0.1)).toFloat,
          Car -> bound(0, 1, randomNormal(0.2, 0.1)).toFloat
        )
      )
      // Choose the subculture, neighbourhood and journeyType
      // TODO: Make neighbourhoods reflect cultures
      val subculture = chooseSubculture()
      val neighbourhood = chooseNeighbourhood()
      val journeyType = chooseJourneyType()

      // Assign random floats from a U[0,1] distribution to weatherSensitivity, autonomy, and consistency
      val weatherSensitivity = scala.util.Random.nextFloat()
      val autonomy = scala.util.Random.nextFloat()
      val consistency = scala.util.Random.nextFloat()

      // Suggestibility is from a N[1,0.25] distribution
      val suggestibility = randomNormal(1.0f, 0.25f).toFloat


      // The currentMode, norm and habit are initially all equal
      val currentMode = chooseInitialNormAndHabit(
        subculture, subcultureConnectivity, suggestibility, journeyType, perceivedEffort
      )
      val norm = currentMode
      val habit = currentMode

      // Create the agent and add it to the HashSet
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
      println(s"[$id] Agent $i generated")
    }
    // Create the social network
    linkAgents(agents, numberOfSocialNetworkLinks, _.socialNetwork)

    // Create a 'social network' for the neighbourhood
    val neighbourhoodsToAgents: Map[Neighbourhood, mutable.HashSet[Agent]] = agents.groupBy(_.neighbourhood)
    for ((_, localAgents) <- neighbourhoodsToAgents) linkAgents(localAgents, numberOfNeighbourLinks, _.neighbours)

    residents = agents.toSet
  }

  def chooseSubculture(): Subculture = {
    val x = scala.util.Random.nextFloat()
    if (x <= 0.33) {
      SubcultureA
    } else if (x <= 0.66){
      SubcultureB
    } else {
      SubcultureC
    }
  }

  def chooseNeighbourhood(): Neighbourhood = {
    val x = scala.util.Random.nextFloat()
    if (x <= 0.25) {
      NeighbourhoodOne
    } else if (x <= 0.5){
      NeighbourhoodTwo
    } else if (x <= 0.75) {
      NeighbourhoodThree
    } else {
      NeighbourhoodFour
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
    val effortForJourneyInverted: Map[TransportMode, Float] = perceivedEffort(commuteLength).mapValues(x => 1.0f - x)
    subcultureDesirabilityWeighted.unionWith(effortForJourneyInverted)(_ * _).maxBy(_._2)._1
  }


  /**
    * Generates a random scale-free network using a preferential attachment mechanism
    *
    * https://en.wikipedia.org/wiki/Barab%C3%A1si%E2%80%93Albert_model
    *
    * @param agents All the agents
    * @param n The minimum number of links an agent should have
    * @param network The network to link
    */
  def linkAgents(agents: mutable.HashSet[Agent], n: Int, network: Agent => mutable.Set[Agent]): Unit = {
    var linkedAgents: mutable.HashSet[Agent] = mutable.HashSet()

    for (agent <- agents) {
      if (linkedAgents.size < n) {
        for (linkedAgent <- linkedAgents) {
          network(agent).add(linkedAgent)
          network(linkedAgent).add(agent)
        }
      } else {
        val totalDegree = linkedAgents.map(a => network(a).size).sum
        val probabilities: java.util.List[Pair[Agent, java.lang.Double]] = linkedAgents
          .map(a => new Pair[Agent, java.lang.Double](a, java.lang.Double.valueOf(network(a).size / totalDegree)))
          .toList
          .asJava

        val distribution = new EnumeratedDistribution[Agent](probabilities)

        val friends = distribution.sample(n, new Array[Agent](n))

        for (friend <- friends) {
          network(friend).add(agent)
          network(agent).add(friend)
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

  /**
    * Returns a random number from a normal distribution
    * @param mean the mean of the normal distribution
    * @param sd the standard deviation of the normal distribution
    * @return a random number from the distribution
    */
  def randomNormal(mean: Double, sd: Double): Double = new NormalDistribution(mean, sd).sample()

  /**
    * Bounds a number above or below
    * @param lowerBound the lower bound
    * @param upperBound the upper bound
    * @param x the value to be bound
    * @return if x < lowerBound then lowerBound, if x > upperBound then upperBound, else x
    */
  def bound(lowerBound: Double, upperBound: Double, x: Double): Double = Math.min(upperBound, Math.max(lowerBound, x))
}