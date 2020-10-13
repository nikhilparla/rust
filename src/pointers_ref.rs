// reference pointers - points to resource in memory

pub fn run()
{
  // primitive array
  let arr1 = [1,2,3];
  let arr2 = arr1;

  println!("values : {:?}", (arr1, arr2));


  // with non-primitives, if you assign anonther variiable to a pience of data, the first 
  // variable will no longer hold the value, you need to use a reference to point to the 
  // resource

  let vec1 = vec![1,2,3];
  let vec2 = &vec1;
  // println!("values : {:?}", (vec1, vec2)); --> ILLEGAL
  println!("values : {:?}", (&vec1, vec2));
}
