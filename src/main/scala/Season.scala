/*
 Based of Met Office data for Northolt https://www.metoffice.gov.uk/public/weather/climate/gcptq81bc#?tab=climateTables
 Days in which there is over 1mm of rainfall
 */
sealed abstract class Season (val percentageBadWeather: Float)
case object Winter extends Season (0.3455f)
case object Spring extends Season (0.3098f)
case object Summer extends Season (0.2696f)
case object Autumn extends Season (0.3275f)
