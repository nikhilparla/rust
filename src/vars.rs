// variables hold primitive data for references to data
// variiables are immutable by default
// rust is a block-scoped language

pub fn run()
{
  let name = "Brad";
  let age = 37;
  // age = 38; --> illegal. age is immutaable
  
  let mut addr = "Troy";
  addr = "Detroit"; // Legal

  println!("My name is {} and I live in {}", name, addr);

  // constants
  const ID: i32 = 001;  // need to add a type when using consts
  println!("ID = {}", ID);
  
  // multiple assingns
  let (my_name, my_age) = ("Brad", 37);
}
