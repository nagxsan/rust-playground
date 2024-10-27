fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
  if a > b {
    a
  } else {
    b
  }
}

fn main() {
  let big_i32 = largest(2, 3);
  let big_char = largest('a', 'c');
  println!("{big_i32}");
  println!("{big_char}");
}