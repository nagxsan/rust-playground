// Option enum lets you return `Some` value or `None` value (None handling)
// Result enum lets you return `Ok` or `Err` value (Error handling)

use std::{fmt::Error, fs};

fn find_first_a(s: &String) -> Option<i32> {
  for (idx, ch) in s.chars().enumerate() {
    if ch == 'a' {
      return Some(idx as i32);
    }
  }

  return None
}

fn custom_read(path: String) -> Result<String, String> {
  let result = fs::read_to_string(path);
  match result {
    Ok(content) => Ok(String::from(content)),
    Err(e) => Err(String::from("Could not read file"))
  }
}

fn main() {
  let s = String::from("sanchet");
  match find_first_a(&s) {
    Some(idx) => println!("First a index in string {s} is: {idx}"),
    None => println!("'a' char not found")
  };

  let file_text = custom_read(String::from("hel1lo.txt"));
  match file_text {
    Ok(content) => println!("{}", content),
    Err(e) => println!("File read error: {e}")
  }
}
