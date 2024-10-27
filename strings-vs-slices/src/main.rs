// String type is a growable, mutable, owned, UTF-8 encoded string type
// Slices let you reference a contiguous sequence of elements in a collection rather than whole collection. A slice is a kind of reference so it does not have ownership.

// Slices are preferred over strings because they provide memory safety since it provides a view to the original string so it is tied to the original string

fn main() {
  // creating, mutating and deleting from a string
  let mut string = String::from("hello");
  string.push_str(" world");
  string.replace_range(2..3, "replace");

  // creating slice
  let slice = &string[0..5];
}