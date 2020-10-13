// cannot change the size runtime
// cannot assing less numbers of values than declared

use std::mem;

pub fn run()
{
  let numbers: [i32;5] = [1,3,4,5, 7];

  println!("{:?}", numbers);

  // access single values
  println!("Single value : {}", numbers[0]);
  
  // length of array
  println!("lenght value : {}", numbers.len());

  // arrays are stack allocated
  println!("array occupies {} bytes",mem::size_of_val(&numbers));

  // get slice
  let slice: &[i32] = &numbers[0..2];
  println!("Slicie is {:?}", slice);

}
