#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod macros;
pub mod generics;
pub mod structs;
pub mod functions;
pub mod boxes;
pub mod closures;
pub mod traits;
pub mod iterators;
pub mod supertraits;
pub mod multipletraits;
pub mod wheres;