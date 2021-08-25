//Vectors resizable arrays.

use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

  // Re-assign value
  numbers[2] = 20;

  println!("{:?}", numbers);

  // Get single val
  println!("Single Value: {}", numbers[0]);

  // Get array length
  println!("Vector Length: {}", numbers.len());

  // Arrays are stack allocated
  println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

  // Get Slice
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);
}
