object Simulation {
  val totalYears = 5
  val numberOfPeople = 150000
  val numberOfSimulations = 10

  def main(args: Array[String]): Unit = {
    val t0 = System.nanoTime()

    var threads: Set[Thread] = Set()

    for (i <- 1 to numberOfSimulations) {
      var thread = new Thread(new Borough(i, totalYears, numberOfPeople))
      thread.start()
      threads += thread
    }

    threads.foreach(_.join())
    val t1 = System.nanoTime()
    System.out.println(s"TOTAL RUNNING TIME: ${t1 - t0}ns")
  }
}
