pub mod first;
pub mod fourth;
pub mod second;
pub mod third;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn ampersand_in_pattern_does_not_match_non_reference() {
        let x = 42;
        let _y = &x;

        match x {
            // &foo => println!("It did match! {:?}", foo), // indeed, confirmed, but because it won't even compile. You never don't know what type you have.
            _ => println!("Nope, no match!"),
        };
    }
}
