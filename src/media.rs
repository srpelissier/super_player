pub trait Playable {
   fn play(&self); // like an abstract method in Java
                   // it's called an Instance method in Rust

   fn pause() {
      // like a static method in Java, can be invoked without an instance
      // it's called an Associated method in Rust
      println!("Paused");
   }
}
