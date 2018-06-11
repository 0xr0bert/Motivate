import java.io.{File, PrintWriter}

object Borough {
  var residents: Set[Agent] = Set[Agent]()
  var day: Int = 0
  val totalYears: Int = 5
  val numberOfPeople: Int = 150000

  def weekday (day: Int): Boolean = day % 7 < 4
  def season (day: Int): Season = day match {
    case x if ((x % 365) >= 0 && (x % 365) < 59) || ((x % 365) >= 334 && (x % 365) < 365) => Winter
    case x if (x % 365) >= 59 && (x % 365) < 151 => Spring
    case x if (x % 365) >= 151 && (x % 365) < 243 => Summer
    case x if (x % 365) >= 243 && (x % 365) < 334 => Autumn
  }

  def main(args: Array[String]): Unit = {
    setUp()
    println("Agents created")

    val writer = new PrintWriter(new File("output.csv"))
    writer.println("Day,ActiveMode,ActiveNorm")

    val firstActive = countActive()
    writer.println(s"0,${firstActive._1},${firstActive._2}")

    var weather = if (scala.util.Random.nextFloat() > Winter.percentageBadWeather) Good else Bad

    for(i <- 1 to totalYears * 365) {
      if (weekday(i)) {
        val currentSeason = season(i)
        val randomFloat = scala.util.Random.nextFloat()
        val newWeather = if (randomFloat > currentSeason.percentageBadWeather) Good else Bad

        if (weather == newWeather) residents.foreach(_.chooseMode(newWeather, false))
        else residents.foreach(_.chooseMode(newWeather, true))
        weather = newWeather

        residents.foreach(_.updateNorm())

        val active = countActive()
        writer.println(s"$i,${active._1},${active._2}")
      }
    }

    writer.close()
  }

  def setUp(): Unit = {
    for (i <- 1 to 150000) {
      // Generate agents
    }
  }

  def countActive(): (Int, Int) = {
    (residents.count((a: Agent) => a.currentMode == Walk || a.currentMode == Cycle),
    residents.count((a: Agent) => a.norm == Walk || a.norm == Cycle))
  }
}
