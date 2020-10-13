// primitive str = immutable fixed length string somewhere in memory
// String = Growable , heap allocated data structure - used when you need to modify or own string data

pub fn run()
{
  let hello = "Hello";  // str string

  let mut hello2 = String::from("Hello");

  println!("{}", hello);

  println!("Length: {}", hello2.len());

  hello2.push('W');
  hello2.push_str("ollo");

  // capacity in bytes
  println!("Capacity : {}", hello2.capacity());

  // is empty
  println!("Is empty: {}", hello2.is_empty());
}
