// TODO: Add some cultures here
sealed abstract class Subculture(val desirability: Map[TransportMode, Float])
case object SubcultureA extends Subculture(Map(Car -> 0.8f, PublicTransport -> 0.5f, Cycle -> 0.9f, Walk -> 0.7f))
case object SubcultureB extends Subculture(Map(Car -> 0.9f, PublicTransport -> 0.8f, Cycle -> 0.6f, Walk -> 0.7f))
case object SubcultureC extends Subculture(Map(Car -> 0.4f, PublicTransport -> 0.5f, Cycle -> 0.9f, Walk -> 0.9f))
