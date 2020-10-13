pub fn run()  // public function run()
{
  // print to console
  println!("Hello World");
  println!("Number : {}", 1);
  println!("{} is from {}", "Brad", "Mars");

  // positiional arguments
  println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mars", "code");

  // names arguments
  println!("{name} likes to play {sport}", name="John", sport="baseball");
}
