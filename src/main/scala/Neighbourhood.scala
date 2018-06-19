// Are these generated or do predefine neighbourhoods?

sealed abstract class Neighbourhood (val supportiveness: Map[TransportMode, Float])

case object NeighbourhoodOne extends Neighbourhood(
  Map(Car -> 0.9f, Cycle -> 0.7f, Walk -> 0.8f, PublicTransport -> 0.9f)
)
