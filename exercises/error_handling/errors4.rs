// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.


#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Check if the value is less than or equal to zero
        if value <= 0 {
            // Return appropriate error based on the value
            if value < 0 {
                Err(CreationError::Negative)
            } else {
                Err(CreationError::Zero)
            }
        } else {
            // Safe to convert to u64 and return Ok
            Ok(PositiveNonzeroInteger(value as u64))
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(Err(CreationError::Negative), PositiveNonzeroInteger::new(-10));
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
