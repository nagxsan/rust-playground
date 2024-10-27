// this takes and modifies ownership of strings throughout the function call and return
fn longest_string(a: String, b: String) -> String {
  if a.len() > b.len() {
    a
  } else {
    b
  }
}

// this takes references and returns the longer string
// here we get an error while returning that we are missing lifetime specifier
// this is so because we are passing `references` of the strings, the original owners of the string are still the variables whose references are passed into this function.
// therefore there may arise a condition where the reference returned by longest_string_str may point to some value but the variable owning that string has gone out of scope. Dangling pointer issue which Rust avoids.

// the function is taking arguments as references and returns a reference, but compiler does not know the return reference is part of which argument.
// fn longest_string_str(a: &str, b: &str) -> &str {
//   if a.len() > b.len() {
//     a
//   } else {
//     b
//   }
// }

// Rust compiler wants to know how does the returned value relates to the lifetimes of the two arguments passed to it.
// Solution: specify lifetimes using generic lifetime annotation
// this specifies that the returned answer would be valid only till both the arguments are valid
fn longest_string_str<'a>(first: &'a str, second: &'a str) -> &'a str {
  if first.len() > second.len() {
    first
  } else {
    second
  }
}

// structs with lifetimes
struct User<'a> {
  name: &'a str
}

struct User2<'a, 'b> {
  first_name: &'a str,
  last_name: &'b str
}

fn main() {
  let longest_str;
  let s1 = String::from("a");
  let s2 = String::from("abc");
  longest_str = longest_string(s1, s2);
  println!("{longest_str}");

  let longest_str_lifetime;
  let s1 = String::from("a");
  {
    let s2 = String::from("abcsdf");

    longest_str_lifetime = longest_string_str(&s1, &s2);
    println!("{longest_str_lifetime}");
  }
  // printing `longest_str_lifetime` here would throw an error since s2 does not live long enough till here
  // println!("{longest_str_lifetime}");


  // structs with lifetimes
  // why we need this? to know how long the struct instance can live
  let user;
  {
    let name = String::from("Sanchet");
    user = User {
      name: &name
    };
    println!("User name: {}", user.name);
  }
  // printing `user.name` here would throw error since the `name` variable does not live long enough
  // println!("User name: {}", user.name);

  let user2: User2;
  let first_name = String::from("Sanchet");
  {
    let last_name = String::from("Nagarnaik");
    user2 = User2 { first_name: &first_name, last_name: &last_name };
    println!("user last name: {}", user2.last_name);
  }
  // cannot use `user2` here as it's lifetime has ended when `last_name` went out of scope.
  // println!("user first name: {}", user2.first_name);
}