

pub mod exceptions {

    use std::fmt::Display;
    use std::fmt::Debug;
    use std::error::Error;
    use std::fmt::Result;
    use std::fmt::Formatter;

    /*
     * Exception: DuplicateSandwichException
     * Signifies dulicate sandwich.
     */
    pub struct DuplicateSandwichException {
    }
    impl Error for DuplicateSandwichException {
    }
    impl Display for DuplicateSandwichException {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            return write!(f, "Dupicate Sandwich, raising Exception");
        }
    }
    impl Debug for DuplicateSandwichException {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            return write!(f, "Dupicate Sandwich, raising Exception");
        }
    }

    /*
     * Exception: InvalidSandwichData
     * Signifies invalid sandwich data.
     */
    pub struct InvalidSandwichData {
    }
    impl Error for InvalidSandwichData {
    }
    impl Display for InvalidSandwichData {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            return write!(f, "Invalid Sandwich, raising Exception");
        }
    }
    impl Debug for InvalidSandwichData {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            return write!(f, "Invalid Sandwich, raising Exception");
        }
    }

    /*
     * Exception: SandwichNotFoundException
     * Signifies sandwich data not found.
     */
    pub struct SandwichNotFoundException {
    }
    impl Error for SandwichNotFoundException {
    }
    impl Display for SandwichNotFoundException {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            return write!(f, "Sandwich not found, raising Exception");
        }
    }
    impl Debug for SandwichNotFoundException {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            return write!(f, "Sandwich not found, raising Exception");
        }
    }

}