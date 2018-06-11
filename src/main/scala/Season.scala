sealed trait Season extends Serializable with Product {
  val percentageBadWeather: Float
}

/*
 Based of Met Office data for Northolt https://www.metoffice.gov.uk/public/weather/climate/gcptq81bc#?tab=climateTables
 Days in which there is over 1mm of rainfall
 */

case object Winter extends Season {
  override val percentageBadWeather: Float = 0.3455f
}

case object Spring extends Season {
  override val percentageBadWeather: Float = 0.3098f
}

case object Summer extends Season {
  override val percentageBadWeather: Float = 0.2696f
}

case object Autumn extends Season {
  override val percentageBadWeather: Float = 0.3275f
}