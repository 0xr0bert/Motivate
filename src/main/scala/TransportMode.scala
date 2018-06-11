sealed trait TransportMode extends Serializable with Product {
  val effort: Float
}

case object Car extends TransportMode {
  override val effort: Float = 0.0f
}

case object PublicTransport extends TransportMode {
  override val effort: Float = 0.2f
}

case object Cycle extends TransportMode {
  override val effort: Float = 1.0f
}

case object Walk extends TransportMode {
  override val effort: Float = 0.9f
}