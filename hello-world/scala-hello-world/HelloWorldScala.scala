object HelloWorldScala {
  def main(args: Array[String]) = {
    val name = if args.length > 0 then args(0) else "Nobody";
    println(s"Hello $name");
  }
}
