sealed trait JourneyType extends Serializable with Product
case object LocalCommute extends JourneyType
case object CityCommute extends JourneyType
case object DistantCommute extends JourneyType