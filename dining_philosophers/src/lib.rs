struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),

        }
    }
}

fn main() {
    let p1 = Philosopher::new("Erykah Badu");
    let p2 = Philosopher::new("Frank Ocean");
    let p3 = Philosopher::new("Adele");
    let p4 = Philosopher::new("Prince");
    let p5 = Philosopher::new("Jean-Michel Basquiat");

}
