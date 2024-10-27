// A trait defines a functionality that a particular type has and can share with other types.
// Traits are used to define shared behaviour in an abstract way.
// Trait bounds are used to specify that a generic type can be any type that has a certain behaviour

// Whatever struct implements a trait, it must have a definition for the function specified in the trait

pub trait Summary {
  fn summarize(&self) -> String;
}

// This has a default implementation of summarize method. If the struct has an implementation that defines this method, it will override the default method otherwise the struct will still have this method even though it need not be implemented explicitly.
pub trait SummaryDefault {
  fn summarize_default(&self) -> String {
    return String::from("Summarize");
  }
}

struct User {
  name: String,
  age: u32
}

// User struct implements the Summary trait
impl Summary for User {
  fn summarize(&self) -> String {
    return format!("User has name {} and age {}", self.name, self.age);
  }
}

impl SummaryDefault for User {}

// Traits as parameters
// When we want to pass a struct reference to a function but only those struct instances must be passed that implement the given trait
pub fn notify(item: &impl Summary) -> String {
  return format!("Notification: {}", item.summarize());
}

// `impl Trait` is actually syntax sugar for `trait bounds`
// The following syntax explanation is: define a function `notify_trait_bound` which takes in an argument `item` of a generic type T however T is constrained on implementing the Summary and SummaryDefault traits, i.e. only accept instances of that struct which has implemented the Summary and SummaryDefault traits.
// The generic T is bound to the traits Summary and SummaryDefault
pub fn notify_trait_bound<T: Summary + SummaryDefault>(item: &T) -> String {
  return format!("Trait Bound Notification: {}", item.summarize());
}

fn main() {
  let user = User {
    name: String::from("sanchet"),
    age: 23
  };

  println!("{}", user.summarize());
  
  let notification = notify(&user);
  println!("{notification}");

  let notification_trait_bound = notify_trait_bound(&user);
  println!("{notification_trait_bound}");
}