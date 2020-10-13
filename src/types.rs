/*
  Primitive types
  Integers : u8, i8, u16, i32, i128 etc
  Floats : f32, f64
  Booleab (bool)
  Characters (car)
  Tuples
  Arrays

  Rust is statically typed language, need to know the type at compile time, however
  compiler can infer type based on value
*/

pub fn run()
{
  // default is "i32"
  let x = 3;

  // default float iis f32
  let y = 6.7;

  // add explicit type
  let z: i64 = 2394857;

  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  // boolean
  let is_active = true;

  println!("{:?}", (x,y,z,is_active));










}
