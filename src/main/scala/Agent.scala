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

  /**
    * Updates the norm of the agent
    *
    * Uses the following function
    * maximise:
    * v * socialNetwork + w * neighbours + x * subcultureDesirability + y * norm + z * habit + neighbourhoodSupportiveness
    *
    * where:
    * v = socialConnectivity * suggestibility
    * w = neighbourhoodConnectivity * suggestibility
    * x = subcultureConnectivity * suggestibility
    * y = adherence
    * z = consistency
    */
  def updateNorm(): Unit = {
    val socialVals = countInSubgroup(socialNetwork, socialConnectivity * suggestibility)
    val neighbourVals = countInSubgroup(neighbours, neighbourhoodConnectivity * suggestibility)
    val subcultureVals = subculture.desirability.map { case(k, v) => (k, v * subcultureConnectivity * suggestibility)}
    val normVals: Map[TransportMode, Float] = Map(norm -> autonomy)
    val habitVals: Map[TransportMode, Float] = habit.mapValues(_ * consistency)
    val valuesToAdd: List[Map[TransportMode, Float]] =
      List(socialVals, neighbourVals, subcultureVals,normVals, habitVals, neighbourhood.supportiveness)

    norm = valuesToAdd
      .reduce(_.unionWith(_)(_ + _)) // Add together vals with same key
      .maxBy(_._2) // find the max tuple by value
      ._1 // Get the key
  }

  /**
    * Calculates the percentages for each different travel mode in a group of agents, multiplied by some weight
    * @param v an iterable of agents
    * @param weight the weight to multiply by
    * @return a Map of TransportModes to weighted percentages
    */

  private def countInSubgroup(v: Traversable[Agent]): Map[TransportMode, Float] =
    v.groupBy(_.lastMode).mapValues(_.size.toFloat / v.size.toFloat)

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
    lastMode = currentMode
    val lastModeMap: Map[TransportMode, Float] = Map(lastMode -> averageWeight)
    habit = lastModeMap.unionWith(habit.mapValues(_ * (1 - averageWeight)))(_ + _)

    // Cycling or walking in bad weather yesterday, strengthens your resolve to do so again
    // Taking a non-active mode weakens your resolve
    val resolve = if (!changeInWeather && (lastMode == Cycle || lastMode == Walk)) {
      0.1f
    } else if (!changeInWeather) {
      -0.1f
    } else {
      0.0f
    }

    val weatherModifier: Map[TransportMode, Float] = Map(
      Cycle -> (if (weather == Bad) 1.0f - weatherSensitivity + resolve else 1.0f),
      Walk ->  (if (weather == Bad) 1.0f - weatherSensitivity + resolve else 1.0f),
      Car -> 1.0f,
      PublicTransport -> 1.0f
    )

    val normVal: Map[TransportMode, Float] = Map (norm -> autonomy).intersectWith(weatherModifier)(_ * _)

    val habitVal: Map[TransportMode, Float] = habit.mapValues(_ * consistency)

    val valuesToAdd: List[Map[TransportMode, Float]] = List(normVal, habitVal, neighbourhood.supportiveness)

    val intermediate: Map[TransportMode, Float] = valuesToAdd.reduce(_.unionWith(_)(_ + _))
    val effort = perceivedEffort(commuteLength).map { case (k, v) => (k, 1.0f - v) }

    val valuesToMultiply: List[Map[TransportMode, Float]] = List(intermediate, effort)

    currentMode =
      valuesToMultiply
        .reduce(_.unionWith(_)(_ + _)) // Add together vals with same key
        .maxBy(_._2) // find the max tuple by value
        ._1 // Get the key
  }
}
