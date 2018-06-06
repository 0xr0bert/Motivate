// Are these generated or do predefine neighbourhoods?

sealed trait Neighbourhood extends Serializable with Product {
  val supportiveness: Map[TransportMode, Float]
}
