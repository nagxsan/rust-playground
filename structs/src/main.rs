struct User {
  name: String,
  email: String,
  age: u8,
}

// Tuple struct
struct Color(u16, u16, u16);

// Unit like struct
struct AlwaysEqual;

#[derive(Debug)] // this specifies a default implementation for Debug trait for this struct
struct Point { x: i32, y: i32 }

fn build_user(name: String, email: String) -> User {
  // field init shorthand where the key, value need not be specified explicitly.
  User {
    name,
    email,
    age: 24
  }
}

fn main() {
  let user = User {
    name: String::from("Sanchet"),
    email: String::from("sanchet@email.com"),
    age: 23
  };

  println!("name: {}", user.name);

  let mut mutable_user = User {
    name: String::from("Joey"),
    email: String::from("joey@tribbiani.com"),
    age: 27
  };
  println!("mutableUser age before change: {}", mutable_user.age);
  mutable_user.age = 26;
  println!("mutableUser age after change: {}", mutable_user.age);

  // Entire instance must be mutable, certain fields cannot be made mutable

  build_user(String::from("Chandler"), String::from("chandler@bing.com"));

  // Using the struct update syntax, we can destructure the existing struct to use its
  // fields to update other users
  let _updated_user = User {
    email: String::from("newuser@user.com"),
    ..user
  };
  // NOTE: Since we are also moving the data `name` from user into _updated_user, therefore
  // `user` would now no longer be valid. This is so because `name` is a String that is a
  // non-Copy type. Therefore, the ownership of the data, now changes into _updated_user.
  // Thus, `user` cannot be used anymore.
  // If the `name` and `email` fields had been redeclared and the `age` field be taken from
  // user then in that situation, `age` value would have been copied since it has the `Copy`
  // trait.

  let red_color = Color(255, 0, 0); // tuple struct declaration
  
  let always_equal = AlwaysEqual;

  // Borrowing fields of a struct would make `p` and `p.x` temporarily lose its permissions
  let mut p = Point { x: 0, y: 0 };
  let x = &mut p.x;
  let y = &mut p.y;
  *x += 1;
  *y += 1;
  println!("{} {}", p.x, p.y);

  println!("point is: {p:#?}"); // debug mode output

  dbg!(&p);
}
