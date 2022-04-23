
pub struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    pub fn x(&self) -> &T {
        &self.x
    }
    pub fn y(&self) -> &T {
        return &self.y;
    }
    pub fn new(_x: T, _y: T) -> Point<T> {
        Point { x: _x, y: _y }
    }
}

#[test]
fn point_test() {
    // panic!("Painc under testing")
}