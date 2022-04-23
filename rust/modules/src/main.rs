extern crate modules;

fn main() {
    modules::foo::foo();
    use modules::foo;
    foo::foo();
    foo::bee::bee();
    modules::top::call_bee();

    // vecotrs
    modules::vectors::vectors();

}