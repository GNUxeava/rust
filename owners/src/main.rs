use std::rc::Rc; // Import reference counter
fn main() {
    let owner = Rc::new(8); // Create a new reference counter
    println!("Owners: {}", Rc::strong_count(&owner));
    { // Create a new closure using {}
      println!("New closure");
      let owner2 = owner.clone(); // clone of the owner
      // To access the value within, you can use *owner2
      println!("Owners: {}", Rc::strong_count(&owner));
      { // new closure
        println!("New closure");
        let owner3 = owner.clone(); 
        println!("Owners: {}", Rc::strong_count(&owner));
        println!("Leaving closure, owner3 dropped");
      }
      println!("Owners: {}", Rc::strong_count(&owner));
      println!("Leaving closure, owner2 dropped");
    }
    println!("Owners: {}", Rc::strong_count(&owner));
}
