// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.

struct ColorClassicStruct {
    // TODO: Something goes here
    colour: String,
    hex: String,
}
#[derive(Debug)]
struct ColorTupleStruct(String, String);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct {
            colour: String::from("green"),
            hex: String::from("0x27AE60"),
        };

        assert_eq!(green.colour, "green");
        assert_eq!(green.hex, "0x27AE60");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ColorTupleStruct(String::from("green"), String::from("0x27AE60"));

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "0x27AE60");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
