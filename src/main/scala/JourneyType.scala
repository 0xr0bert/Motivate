sealed abstract class JourneyType(val cost: Map[TransportMode, Float])
case object LocalCommute extends JourneyType(Map(
  Car -> 0.1f,
  PublicTransport -> 0.1f,
  Walk -> 0.2f,
  Cycle -> 0.4f
))
case object CityCommute extends JourneyType(Map(
  Car -> 0.1f,
  PublicTransport -> 0.1f,
  Walk -> 0.9f,
  Cycle -> 0.6f
))
case object DistantCommute extends JourneyType (Map(
  Car -> 0.1f,
  PublicTransport -> 0.1f,
  Walk -> 1.1f, // This should make it impossible
  Cycle -> 1.1f // This should make it impossible
))
