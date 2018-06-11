import scalaz.Scalaz._
/*
 Could psychological variables use a random value from a normal distribution mean = 1, s.d. = 0.25,
 then this value could be multiplied by the global importance, to generate the specific importance to the agent

 Should psychological factors be between 0-2?

 Should car / bike ownership be stored in agent?

 TODO: What parameters are needed, and how do they impact the functions of Agent
 */

/***
  * An Agent for the model
  *
  * @param subculture                The mobility subculture the agent is a member of
  * @param subcultureConnectivity    Importance of subculture to individual agent for making decisions about commuting.
  *                                  1 = No adjustment.
  *                                  < 1 = Down weighting of importance
  *                                  > 1 = Up weighting of importance.
  *                                  0 = No importance/ ignore factor.
  * @param neighbourhood             The neighbourhood in which the agent lives
  * @param neighbourhoodConnectivity Importance of local neighbourhood to individual agent
  *                                  for making decisions about commuting.
  *                                  1 = No adjustment.
  *                                  < 1 = Down weighting of importance
  *                                  > 1 = Up weighting of importance.
  *                                  0 = No importance/ ignore factor.
  * @param distance The JourneyType that signifies the commute distance
  * @param currentMode How the agent is currently going to work
  * @param habit Commuting mode last used
  * @param norm Preferences for a particular commuting mode
  * @param adherence How important is an agent’s subculture in directing their norm?
  *                  Score from 0-1. 0 - No effect.
  * @param neighbourSuggestiblility How important are the observed habits of neighbours in directing an agent’s norm?
  *                                 Score from 0-1. 0 - No effect
  * @param autonomy How important group norms are to an agent choosing their commuting mode
  *                 Score from 0-1. 0 - No effect
  * @param consistency How much of an impact habits have on present choice of commuting mode
  *                    Score from 0-1. 0 - No effect
  * @param defiance How much the supportiveness of the physical environment
  *                 conditions an agents choice of commuting mode
  *                 Score from 0-1. 0 - No effect
  * @param weatherSensitivity How much weather affects an agent.
  *                           Score from 0-1. 0 - No effect.
  */
class Agent(
           val subculture: Subculture,
           val subcultureConnectivity: Float,
           val neighbourhood: Neighbourhood,
           val neighbourhoodConnectivity: Float,
           val distance: JourneyType,
           var currentMode: TransportMode,
           var habit: TransportMode,
           var norm: TransportMode,
           val adherence: Float,
           val neighbourSuggestiblility: Float,
           val autonomy: Float,
           val consistency: Float,
           val defiance: Float,
           val weatherSensitivity: Float,
           val laziness: Float
           ) {
  var socialNetwork: Set[Agent] = _
  var socialConnectivity: Float = _
  var socialSuggestibility: Float = _

  // TODO: The neighbourhood isn't used in this
  /**
    * Updates the norm of the agent
    */
  def updateNorm(): Unit = {
    val socialWeight = socialConnectivity * socialSuggestibility
    val subcultureWeight = subcultureConnectivity * adherence
    val neighbourhoodWeight = neighbourhoodConnectivity * neighbourSuggestiblility
    val social = countInSubgroup(socialNetwork, socialWeight)
    val subcultureVals = subculture.preferences.map { case(k, v) => (k, v * subcultureWeight) }
    val normVal: Map[TransportMode, Double] = Map (norm -> 1.0 * autonomy)
    val habitVal: Map[TransportMode, Double] = Map (habit -> 1.0 * consistency)
    val valuesToMultiply: List[Map[TransportMode, Double]] = List(social, subcultureVals, normVal, habitVal)
    // Multiplies all the values, and chooses the maximum one
    norm = valuesToMultiply.reduce(_.intersectWith(_)(_ * _)).maxBy(_._2)._1
  }

  private def countInSubgroup(v: Set[Agent], weight: Float): Map[TransportMode, Double] = {
    v.groupBy(_.habit).mapValues(_.size.toDouble * weight / v.size)
  }

  def chooseMode2(weather: Weather, changeInWeather: Boolean): Unit = {
    /*
     * capacity = activeness * feasibility
     * cost = capacity * supportiveness * weather
     * decision = (laziness * cost) * (autonomy * norm) * (consistency * habit)
     *
     * TODO: This needs to be checked for correctness
     */
    habit = currentMode

    val feasibility = subculture.feasibility(distance)
    val capacity: Map[TransportMode, Double] = feasibility.map { case (k, v) => (k, v.toDouble * k.effort)}
    // TODO: How does weather fit in to here
    val cost: Map[TransportMode, Double] = capacity.intersectWith(neighbourhood.supportiveness)(_ * _)

    val normVal: Map[TransportMode, Double] = Map (norm -> 1.0 * autonomy)
    val habitVal: Map[TransportMode, Double] = Map (habit -> 1.0 * consistency)
    val lazyCost: Map[TransportMode, Double] = cost.map { case (k, v) => (k, v * laziness)}

    val valuesToMultiply: List[Map[TransportMode, Double]] = List(lazyCost, normVal, habitVal)
    // TODO: Should this be max or min
    currentMode = valuesToMultiply.reduce(_.intersectWith(_)(_ * _)).maxBy(_._2)._1
  }

  /*
  Every agent has a variable sensitivity to bad weather, from being very hardy to very averse.
  Each agent has a ‘weather_sensitivity’ variable drawn at agent initiation from a continuous uniform
  distribution U(0,1). Currently, the effect of habit either strengthens or weakens the random value drawn to compare
  against weather sensitivity by +/- 0.1. Effectively, cycling or walking in bad weather yesterday strengthens your
  resolve to do it again, whereas switching to a non-active mode weakens your resolve to commute actively on the next
  day of bad weather. Currently, any negative change to the weather sparks a new decision and hence intermittent periods
  of bad weather are independent of one another.
   */
  def chooseMode(weather: Weather, changeInWeather: Boolean) : Unit = {
    match weather {
      case Good => currentMode = norm
      case Bad => currentMode = choose(changeInWeather)
    }

    habit = currentMode
  }

  // TODO: Shouldn't the suggestibilities be used here?
  /**
    * Weather should be bad if this is called
    * @param changeInWeather if the weather has changed (to be bad)
    * @return the chosen TransportMode
    */
  private def choose(changeInWeather: Boolean) : TransportMode = {
    if (!changeInWeather && (norm == Walk || norm == Cycle)) {
      // Weather is consistently bad and active mode
      val r = scala.util.Random
      var randomValue = r.nextDouble()
      // Strengthen or dampen based on habit
      // TODO: Should these be the other way around?
      if (habit == Walk || habit == Cycle) {
        randomValue -= 0.1
      } else {
        randomValue += 0.1
      }

      if (weatherSensitivity > randomValue) {
        // Ignore the bad weather
        norm
      } else if (habit == PublicTransport || habit == Car) {
        habit
      } else {
        PublicTransport
      }

      /*
      else if (!car) {
        return PublicTransport
      } else {
        val randomInt = r.nextInt(2)
        if (randomInt == 0) {
          return PublicTransport
        } else {
          return Car
        }
      }
       */
    } else if (!changeInWeather) {
      norm
    } else if (changeInWeather && (norm == Walk || norm == Cycle)) {
      val r = scala.util.Random
      val randomValue = r.nextDouble()
      if (weatherSensitivity > randomValue) {
        norm
      } else {
        PublicTransport
      }
      /*
      else if (!car) {
        return PublicTransport
      } else {
        val randomInt = r.nextInt(2)
        if (randomInt == 0) {
          return PublicTransport
        } else {
          return Car
        }
      }
       */
    } else {
      norm
    }
  }
}
