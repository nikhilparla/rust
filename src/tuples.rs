// Tuples group together values of differnt types
// max 12 elements
// similar to structure but has more functions to operate on it

pub fn run()
{
  let person: (&str, &str, i8) = ("Brad", "Mars", 37);

  println!("{} is from {} and is {}", person.0,person.1, person.2);
}
