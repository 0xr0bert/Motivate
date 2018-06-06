/*
 Could psychological variables use a random value from a normal distribution mean = 1, s.d. = 0.25,
 then this value could be multiplied by the global importance, to generate the specific importance to the agent.git s
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
           val weatherSensitivity: Float
           ) {
  var socialNetwork: Vector[Agent] = _
  var socialConnectivity: Float = _
  var socialSuggestibility: Float = _
}