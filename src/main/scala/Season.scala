sealed trait Season extends Serializable with Product {
  val percentageBadWeather: Float
}

// TODO: Change these values to something backed up with data

case object Winter extends Season {
  override val percentageBadWeather: Float = 0.7f
}

case object Spring extends Season {
  override val percentageBadWeather: Float = 0.5f
}

case object Summer extends Season {
  override val percentageBadWeather: Float = 0.2f
}

case object Autumn extends Season {
  override val percentageBadWeather: Float = 0.5f
}