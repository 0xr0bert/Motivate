import scalaz._
import Scalaz._

import scala.collection.mutable
/*
 Could psychological variables use a random value from a normal distribution mean = 1, s.d. = 0.25,
 then this value could be multiplied by the global importance, to generate the specific importance to the agent

 Should psychological factors be between 0-2?

 Should car / bike ownership be stored in agent?

 TODO: What parameters are needed, and how do they impact the functions of Agent
 */
/**
  * An agent for the model
  * @param subculture The demographic subculture the agent belongs to
  * @param neighbourhood The neighbourhood the agent lives in
  * @param commuteLength The JourneyType that signifies the commute distance
  * @param perceivedEffort The perceived effort of a mode of transport, for a given journey length
  *                        values should be between 0-1
  * @param weatherSensitivity How sensitive the agent is to weather (between 0-1)
  * @param autonomy How important the agent's norm is in deciding their mode of travel (between 0-1)
  * @param consistency How important the agent's habit is in deciding their mode of travel (between 0-1)
  * @param suggestibility How suggestible the agent is to the influence of their social network,
  *                       subculture, and neighbourhood (between 0-2):
  *                       1 = No adjustment
  *                       < 1 = Less important
  *                       > 1 = More important
  * @param socialConnectivity How connected the agent is to their social network
  * @param subcultureConnectivity How connected the agent is to their subculture
  * @param neighbourhoodConnectivity How connected the agent is to their neighbourhood
  * @param currentMode How the agent is currently going to work
  * @param lastMode The last commuting mode used
  * @param norm The preferred commuting mode
  */
class Agent(val subculture: Subculture,
            val neighbourhood: Neighbourhood,
            val commuteLength: JourneyType,
            val perceivedEffort: Map[JourneyType, Map[TransportMode, Float]],
            val weatherSensitivity: Float,
            val autonomy: Float,
            val consistency: Float,
            val suggestibility: Float,
            val socialConnectivity: Float,
            val subcultureConnectivity: Float,
            val neighbourhoodConnectivity: Float,
            val averageWeight: Float,
            var habit: Map[TransportMode, Float],
            var currentMode: TransportMode,
            var lastMode: TransportMode,
            var norm: TransportMode
           ) {
  var socialNetwork: mutable.Set[Agent] = mutable.Set()
  var neighbours: mutable.Set[Agent] = mutable.Set()

  def calculateModeBudget(): Map[TransportMode, Float] = {
    // TODO: This creates very high values for the preferred mode, and very low for all others
    val socialVals = countInSubgroup(socialNetwork).mapValues(_ * socialConnectivity)
    val neighbourVals = countInSubgroup(neighbours).mapValues(_ * neighbourhoodConnectivity)
    val valuesToAverage = List(socialVals, neighbourVals, subculture.desirability.mapValues(_ * subcultureConnectivity), habit)

    // Find the average
    val intermediate = valuesToAverage
        .reduce(
          _.unionWith(_)(_ + _)
        )
        .mapValues(_ / valuesToAverage.size)

    // Make it so the the max mode has a budget of 1, therefore at least one mode is always possible
    val weight = 1.0f / intermediate
      .maxBy(_._2)
      ._2

    val budget = intermediate
      .mapValues(_ * weight)

    norm = budget
      .maxBy(_._2)
      ._1

    budget
  }

  def calculateCost(weather: Weather, changeInWeather: Boolean): Map[TransportMode, Float] = {
    // TODO: How to fit weather in here

    val valuesToAverage = List(commuteLength.cost, neighbourhood.supportiveness.mapValues(v => 1.0f - v))
    // Find the average
    var intermediate = valuesToAverage
      .reduce(
        _.unionWith(_)(_ + _)
      )
      .mapValues(_ / valuesToAverage.size)

    if (weather == Bad) {
      // Cycling or walking in bad weather yesterday, strengthens your resolve to do so again, therefore reducing the cost
      // Taking a non-active mode weakens your resolve, therefore increasing the cost
      val resolve = if (!changeInWeather && (lastMode == Cycle || lastMode == Walk)) {
        0.1f
      } else if (!changeInWeather) {
        -0.1f
      } else {
        0.0f
      }

      intermediate = Map(
        Cycle -> (intermediate(Cycle) * ((1.0f + weatherSensitivity) + resolve)),

        Walk -> (intermediate(Walk) * ((1.0f + weatherSensitivity) + resolve)),

        Car -> intermediate(Car),
        PublicTransport -> intermediate(PublicTransport)
      )
    }

    // Make it so the the min mode has a at most a cost of 1, therefore at least one mode is always possible
    val intermediate_min = intermediate
      .minBy(_._2)
      ._2

    if (intermediate_min > 1.0f) {
      val weight = 1.0f / intermediate_min
      intermediate.mapValues(_ * weight)
    } else {
      intermediate
    }
  }

  /**
    * Calculates the percentages for each different travel mode in a group of agents, multiplied by some weight
    * @param v an iterable of agents
    * @return a Map of TransportModes to weighted percentages
    */
  private def countInSubgroup(v: Traversable[Agent]): Map[TransportMode, Float] =
    v.groupBy(_.lastMode).mapValues(_.size.toFloat / v.size.toFloat)


  def updateHabit(): Unit = {
    lastMode = currentMode
    val lastModeMap: Map[TransportMode, Float] = Map(lastMode -> averageWeight)
    habit = lastModeMap.unionWith(habit.mapValues(_ * (1 - averageWeight)))(_ + _)
  }

  /**
    * Choose a new mode of travel
    *
    * maximise:
    * ((autonomy * norm) + (consistency * habit) + supportiveness)) * weather * effort
    *
    * @param weather the weather
    * @param changeInWeather whether there has been a change in the weather
    */
  def choose(weather: Weather, changeInWeather: Boolean): Unit = {
    val budget: Map[TransportMode, Float] = calculateModeBudget()
    val cost: Map[TransportMode, Float] = calculateCost(weather, changeInWeather)
//    if (weather == Bad && budget.maxBy(_._2)._1 == Cycle) {
//      print("BREAK")
//    }
    currentMode =
      budget
        .filter(pair => pair._2 >= cost(pair._1))
        .map {case (k, v) => (k, v - cost(k))}
        .maxBy(_._2)
        ._1
  }
}
