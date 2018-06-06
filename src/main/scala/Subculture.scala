sealed trait Subculture {
  val preferences: Map[TransportMode, Float]
}

case object Pedestrian extends Subculture {
  override val preferences: Map[TransportMode, Float] =
    Map (Car -> 0.0f, PublicTransport -> 0.9f, Cycle -> 0.5f, Walk -> 0.9f) // What values?
}

case object Cycling extends Subculture {
  override val preferences: Map[TransportMode, Float] =
    Map (Car -> 0.0f, PublicTransport -> 0.9f, Cycle -> 0.5f, Walk -> 0.9f) // What values?
}

case object Driving extends Subculture {
  override val preferences: Map[TransportMode, Float] =
    Map (Car -> 0.0f, PublicTransport -> 0.9f, Cycle -> 0.5f, Walk -> 0.9f) // What values?
}