sealed trait TransportMode extends Serializable with Product
case object Car extends TransportMode
case object PublicTransport extends TransportMode
case object Cycle extends TransportMode
case object Walk extends TransportMode
