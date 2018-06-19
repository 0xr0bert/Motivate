//// Subculture as output, not input?
//
//// Global
//val numberOfNeighbourhoods = 20
//val numberOfPeople = 150000
//val years = 1
//val days = years * 365
////def weekday (day: Int) = day % 7 < 4
////def season (day: Int): Season = day match {
////  case x if ((x % 365) >= 0 && (x % 365) < 59) || ((x % 365) >= 334 && (x % 365) < 365) => Winter
////  case x if (x % 365) >= 59 && (x % 365) < 151 => Spring
////  case x if (x % 365) >= 151 && (x % 365) < 243 => Summer
////  case x if (x % 365) >= 243 && (x % 365) < 334 => Autumn
////}
//
//// Agent belongings
//val numberOfNetworkContactsPerAgent = 20   // Vary about distribution, who to include?
//val numberOfSubcultures = 5
//
//val influenceOfNeighbourhoodOnSubculture = 0.8
//val influenceOfNeighbourhoodOnNetwork = 0.6
//val influenceOfSubcultureOnNetwork = 0.8
//
//// Initially, we use the same connectedness values for all agents, but could vary per agent
//val connectednessToNeighbourhood = 0.4
//val connectednessToSubculture = 0.6
//val connectednessToNetwork = 0.8
//
//// Perceived effort
//val activeness = Map (Car -> 0.0, PublicTransport -> 0.2, Cycle -> 1.0, Walk -> 0.9)
//
//case class Neighbourhood (name: String)
//case class Person (journeyType: JourneyType)
////case class Subculture (name: String)
//
//case class Borough (residents: Vector[Person],
//                    neighbourhoods: Map[Person, Neighbourhood],
//                    subcultures: Map[Person, Subculture],
//                    networks: Map[Person, Vector[Person]])
//
////val subcultureA = Subculture ("CultureA")
////val subcultureB = Subculture ("CultureB")
////val subcultureC = Subculture ("CultureC")
////val subcultures = Vector (subcultureA, subcultureB, subcultureC)
////val culturalPreferences = Map (
////  subcultureA -> Map (Car -> 0.9, PublicTransport -> 0.1, Cycle -> 0.0, Walk -> 0.0),
////  subcultureB -> Map (Car -> 0.0, PublicTransport -> 0.2, Cycle -> 1.0, Walk -> 0.9),
////  subcultureC -> Map (Car -> 0.0, PublicTransport -> 0.2, Cycle -> 1.0, Walk -> 0.9))
//
///*
//Time: The time scale modelled is 1 to 5 years, initially considering weekdays only (so about 260 days per year). Each day, there will be a type of weather from a set, derived from the current season.
//
//Cost of action:
//Each agent has an environment-independent capacity for each mode.
//Capacity is influenced by the activeness rating of the mode, the agent's journey type, and the agent's subculture, e.g. whether it is common for members of that group to have access to a car/bike.
//Each neighbourhood has a supportiveness for each mode.
//Capacity, supportiveness and weather together give the cost required for an agent to use a mode for travel on a day.
//
//Psychology: Influencing individual decisions, each agent has two attributes: laziness, corresponding to how much they care about the effort of travel, and consistency, corresponding to how easily they fall into habits. Influencing long-term behaviour, each agent has a level of inertia, corresponding to how easily they change their ways, a level of defiance, corresponding to how much they push against interventions seen as trying to change their ways, and a suggestibility, corresponding to the influence of their peers.
//
//Norms:
//Each agent has a mode of travel they think of as their normal way to travel, their norm, and the mode of travel that they have most recently used, their habit.
//The initial norm will be influenced by the agent's subculture and the supportiveness of their neighbourhood for that mode.
//The initial habit will be the same as the initial norm.
//
//Decisions:
//An agent will decide on the mode to use on a day based on:
//the cost of using each mode, their current norm, and their current habit. These will be weighted by their laziness (increasing the effect of the cost) and their consistency (increasing the effect of the habit).
//
//Influencing norms:
//An agent's habit is set to be the mode most recently used.
//At the end of each day, an agent will determine its new norm based on:
//its current norm, any change in their capacity for a mode, any change in supportiveness of the neighbourhood's environment for a mode, their observations of actions last performed by members of their groups.
//These will be weighted by the agent's inertia (increasing the effect of the current norm) and defiance (increasing/decreasing the effect of the change in supportiveness), and suggestibility (increasing the effect of observed actions of others. The agent's connectedness to each their groups weights how much influence observations of that group has on the decision.
//
//In addition, we could consider:
// - modelling agents' associations between travel modes and their values (such as status), influenced by their subculture
// - modelling weekends and having different journey types, which can influence each other
// - modelling habits built up over longer periods rather than only the previous day
//
//*/