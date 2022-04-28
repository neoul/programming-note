pub fn example() {
    trait A {
        fn get(&self) -> String;
    }

    trait B {
        fn get(&self) -> u8;
    }

    struct S {
        a: String,
        b: u8,
    }
    
    impl A for S {
        fn get(&self) -> String {
            return self.a.clone();
        }
    }

    impl B for S {
        fn get(&self) -> u8 {
            return self.b;
        }
    }

    let s = S {
        a: "AAA".to_string(),
        b: 42,
    };

    // multiple `get` found rustc E0034
    // https://doc.rust-lang.org/error-index.html#E0034
    // let a = s.get();

    let a = <S as A>::get(&s);
    let b = <S as B>::get(&s);
    println!("{},{}", a, b);
}
