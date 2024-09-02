fn main() {

  // Variables live on the stack frame defined within the function scope
  let a = 5;

  // When an expression reads a variable, it copies the value from the stack frame for usage
  let mut b = a;
  b += 1;

  println!("a = {a}, b = {b}"); // a = 5, b = 6

  // A pointer is a value that describes a location in memory.
  // The value that a pointer points-to is called its pointee.
  // One common way to make a pointer is to allocate memory in the heap.
  // Data can live indefinitely in the heap.
  // Heap data is NOT TIED to a specific stack frame.

  let _a = Box::new([0; 1_000_000]);
  // _a is a pointer that points to the 1 million long array which is stored in the heap
  // Box is used to create data on the heap
  
  // A stack frame is automatically deallocated when function ends.
  // How is heap data deallocated? If manually deallocation is allowed, then there
  // may be dangling pointers issue. The pointer would point to an invalid
  // memory location. Therefore, Rust does not allow this
  // When a heap-allocated variable goes out of scope, the heap data
  // corresponding to that variable is deallocated.

  let _x = 4;
  {
    let _y = Box::new(5);
    // inside this scope, the pointer _y points to the location that holds the value 5
  }
  // outside the scope, since _y does not exist, Rust frees up the location that held 5

  // Now consider this code:
  let a = Box::new([2,3,4]);
  let b = a;
  
  // Here, let's say that we want to free the memory space, then it would free
  // for both a and b making it a double clear.
  // So to prevent this, Rust introduces the concept of ownership.
  // First `a` has the ownership to the Box.
  // When we assign the pointer `a` to the variable `b`, we render `a` moot.
  // In doing so if `a` is used again in the code, it would throw error
  // since the ownership of `a`'s data has transferred to `b`

  //  If a variable owns a box, when Rust deallocates the variable’s frame, then Rust deallocates the box’s heap memory.
  // Now, `b` owns the Box so ONLY WHEN scope of `b` ends will the heap data be
  // deallocated. If the ownership is transferred, then the heap data would still
  // remain, now bound to the new variable

  let a = Box::new(6);
  {
    let _b = a;
  }

  // Below code would not compile since the ownership of a has been moved so `a` is
  // now undefined.
  // println!("a = {}", a);

  let first = String::from("Hello");
  let greeting = make_greeting(first);
  println!("{greeting}");

  // In the above example:
  // first points to the heap location where "Hello" is stored.
  // When the `make_greeting` function is called, the ownership of `first` is
  // transferred to `name` rendering `first` moot. Now `name` appends John to the
  // string and gives the ownership to `greeting`. Since `name` exists within the scope of
  // `make_greeting`, when the function returns, the variable scope for `name`
  // ends and it is removed. Now `greeting` owns the heap data. `Hello John`.

  // Remember: it's not a problem that first points to deallocated memory.
  // It's a problem that we tried to use first after it became invalid.

  // Moved heap data principle: if a variable x moves ownership of heap
  // data to another variable y, then x cannot be used after the move.

  let b = Box::new(0);
  let b2 = b;
  // println!("{}", b); Incorrect since b's ownership has been transferred
  move_a_box(b2);

  // Ownership is primarily a discipline of heap management:
  // - All heap data must be owned by exactly one variable.
  // - Rust deallocates heap data once its owner goes out of scope.
  // - Ownership can be transferred by moves, which happen on assignments and function calls.
  // - Heap data can only be accessed through its current owner, not a previous owner.

}

fn make_greeting(mut name: String) -> String {
  name.push_str("John");
  name
}

fn move_a_box(b: Box<i32>) {
  println!("{}", b);
}