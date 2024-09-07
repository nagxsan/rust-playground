fn main() {
  // A string slice is a reference to a part of a string
  let s = String::from("hello world");
  let first_word = &s[0..5]; // slice that points to `hello` in s
  let second_word = &s[6..11]; // slice that points to `world` in s
  let s2 = &s; // immutable reference to string s.
  println!("first_word: {}, second_word: {}, s2: {}", first_word, second_word, s2);

  // if slice is from the start just do &s[..5]
  // if slice is till the end just do &s[6..]
  // entire string slice: &s[..]
}
