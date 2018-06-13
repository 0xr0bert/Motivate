sealed abstract class Subculture(val desirability: Map[TransportMode, Float])
case object SubcultureA extends Subculture(Map(Car -> 0.0f, PublicTransport -> 0.9f, Cycle -> 0.5f, Walk -> 0.9f))
