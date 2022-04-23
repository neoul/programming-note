pub mod bar;
pub mod foo;
pub mod top {
    pub fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    pub fn call_bee() {
        super::foo::bee::bee();
        // ::foo::bee::bee();
        use super::foo::bee::bee;
        bee();

    }
}
pub mod vectors;