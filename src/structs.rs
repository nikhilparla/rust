// used to create custom data types

// traditiional struct
struct Color{
  red: u8,
  green: u8,
  blue: u8,
}

// tuple struct
struct Color2(u8, u8, u8);

struct Person{
  first_name: String,
  last_name: String
}

// create some functions associated with person struct
impl Person
{
  // construct a person, takes two params and returns a new Person
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string()
    }
  }

  fn full_name(&self) -> String  // just like this
  {
    format!("{} {}", self.first_name, self.last_name)
  }

  // set last name
  fn set_last_name(&mut self, last: &str)  // just like this
  {
    self.last_name = last.to_string();
  }

  // set name to tuple
  fn to_tuple(self) -> (String, String)   // return a tuple of two strings
  {
    (self.first_name, self.last_name)   // no semicolon when returing something
  }
}


pub fn run()
{
  let mut c = Color {
    red: 255,
    green: 0,
    blue: 0,
  };

  println!("Color : {} {} {}", c.red, c.green, c.blue);

  c.red = 200;

  println!("Color : {} {} {}", c.red, c.green, c.blue);

  let mut c2 = Color2(223, 0, 0);
  println!("Color : {} {} {}", c2.0, c2.1, c2.2);

  let mut p = Person::new("John", "Doe");
  println!("Person {} {}", p.first_name, p.last_name);

  println!("Person full name:  {}", p.full_name());
  
  let mut m = Person::new("Mary", "Doe");
  println!("Person full name:  {}", m.full_name());
  m.set_last_name("William");
  println!("Person full name:  {}", m.full_name());

  println!("Person tuple:  {:?}", m.to_tuple());  // check the "debug tray" inside curly brackets



}
