// Option enum lets you return `Some` value or `None` value

fn find_first_a(s: &String) -> Option<i32> {
  for (idx, ch) in s.chars().enumerate() {
    if ch == 'a' {
      return Some(idx as i32);
    }
  }

  return None
}

fn main() {
  let s = String::from("sanchet");
  match find_first_a(&s) {
    Some(idx) => println!("First a index in string {s} is: {idx}"),
    None => println!("'a' char not found")
  };
}
