class Neighbourhood (val supportiveness: Map[TransportMode, Float])

case object NeighbourhoodOne extends Neighbourhood(
  Map(Car -> 0.9f, Cycle -> 0.7f, Walk -> 0.8f, PublicTransport -> 0.9f)
)

case object NeighbourhoodTwo extends Neighbourhood(
  Map(Car -> 0.5f, Cycle -> 0.7f, Walk -> 0.8f, PublicTransport -> 1.0f)
)

case object NeighbourhoodThree extends Neighbourhood(
  Map(Car -> 0.9f, Cycle -> 0.2f, Walk -> 0.6f, PublicTransport -> 0.5f)
)

case object NeighbourhoodFour extends Neighbourhood(
  Map(Car -> 0.2f, Cycle -> 0.9f, Walk -> 0.9f, PublicTransport -> 0.9f)
)
