#[derive(Debug, PartialEq, Eq)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[allow(dead_code)]
fn add_two(a: i32) -> i32 {
    if a > 100 {
        panic!("value must be less than or equal to 100.");
    }
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller), "{:?}", larger);
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(!smaller.can_hold(&larger), "{:?}", smaller);
    }

    #[test]
    fn it_adds_two() {
        // assert_eq!와 assert_ne!는 
        // 각각 ==과 != 연산자 사용하므로
        // PartialEq와 Debug 트레잇을 구현해 함
        assert_eq!(4, add_two(2));
        assert_ne!(4, add_two(3));
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        add_two(200);
    }

    #[test]
    #[should_panic(expected = "value must be less than or equal to 100")]
    fn greater_than_100_2() {
        add_two(200);
    }
}