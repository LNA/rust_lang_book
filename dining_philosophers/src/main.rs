struct Philosopher {
name: String,
}

imp Philosopher {
  fn new(name: &str) -> Philosopher {
    Philosopher {
name: name.to_string(),

    }
  }
}

fn main() {
  let p1 = Philosopher::new("Janelle Monet");
  let p2 = Philosopher::new("Prince");
  let p3 = Philosopher::new("Grace Jones");
  let p4 = Philosopher::new("Erykah Badu");
  let p5 = Philosopher::new("SZA");
}

