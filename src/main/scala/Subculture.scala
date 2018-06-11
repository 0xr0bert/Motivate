sealed trait Subculture extends Serializable with Product {
  val preferences: Map[TransportMode, Float]
  val feasibility: Map[JourneyType, Map[TransportMode, Float]]
}

// TODO: Add real values here

case object Pedestrian extends Subculture {
  override val preferences: Map[TransportMode, Float] =
    Map (Car -> 0.0f, PublicTransport -> 0.9f, Cycle -> 0.5f, Walk -> 0.9f) // What values?
  override val feasibility: Map[JourneyType, Map[TransportMode, Float]] =
    Map (LocalCommute -> Map (),
      CityCommute -> Map(),
      DistantCommute -> Map())
}

case object Cycling extends Subculture {
  override val preferences: Map[TransportMode, Float] =
    Map (Car -> 0.0f, PublicTransport -> 0.9f, Cycle -> 0.5f, Walk -> 0.9f) // What values?
  override val feasibility: Map[JourneyType, Map[TransportMode, Float]] =
    Map (LocalCommute -> Map (),
      CityCommute -> Map(),
      DistantCommute -> Map())
}

case object Driving extends Subculture {
  override val preferences: Map[TransportMode, Float] =
    Map (Car -> 0.0f, PublicTransport -> 0.9f, Cycle -> 0.5f, Walk -> 0.9f) // What values?
  override val feasibility: Map[JourneyType, Map[TransportMode, Float]] =
    Map (LocalCommute -> Map (),
      CityCommute -> Map(),
      DistantCommute -> Map())
}
