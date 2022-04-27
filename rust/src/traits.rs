
trait Animal {
    // associated function for the trait to be implemented
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;
    fn talk(&self) { // default method provided by the trait.
        println!("{} says {}", self.name(), self.noise());
    }
}

struct Sheep {naked: bool, name: &'static str}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }
    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is alread naked", self.name);
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    // `Self` is the implementor type: `Sheep`.
    fn new(name: &'static str) -> Self {
        Sheep { naked: false, name }
    }
    fn name(&self) -> &'static str {
        return self.name;
    }
    fn noise(&self) -> &'static str {
        if self.is_naked() {
            return "baaaaaah?";
        }
        return "baah!";
    }
    // overridden talk()
    fn talk(&self) {
        println!("{} says {}", self.name, self.noise());
    }
}

struct Cow {name: &'static str}

impl Animal for Cow {
    fn new(name: &'static str) -> Self {
        Cow { name }
    }
    fn name(&self) -> &'static str {
        return self.name;
    }
    fn noise(&self) -> &'static str {
        return "mooooo!";
    }
    // overridden talk()
    fn talk(&self) {
        println!("{} says {}", self.name, self.noise());
    }
}

pub fn example() {
    let mut dolly: Sheep = Animal::new("Dolly");
    // let mut dolly = Sheep::new("DDolly");
    // let mut dolly = Sheep {naked : false, name: "DDDolly"};
    dolly.talk();
    dolly.shear();
    dolly.talk();

    // Derive: Provides Basic implmentations of some std traits
    #[derive(PartialEq, PartialOrd, Debug)]
    struct Centimeters(f64);

    #[derive(PartialEq, PartialOrd, Debug)]
    struct Inches(i32);
    impl Inches {
        fn centimeters(&self) -> Centimeters {
            // destructuring by let
            let &Inches(inches) = self;
            Centimeters(inches as f64 * 2.54)
        }
    }
    let foot = Inches(12);
    let meter = Centimeters(100.0);
    let cmp = if foot.centimeters() > meter {
        "smaller"
    } else {
        "bigger"
    };
    println!("foot({:?}) is {:?} meter({:?}).", foot, cmp, meter);

}