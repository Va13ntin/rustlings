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
        if value < 1 {
            // Si la valeur est négative, retourne Err avec CreationError::Negative.
            if value < 0 {
                return Err(CreationError::Negative);
            } else {
                // Si la valeur est zéro, retourne Err avec CreationError::Zero.
                return Err(CreationError::Zero);
            }
        }
        // Si la valeur est positive et non nulle, retourne Ok avec PositiveNonzeroInteger.
        Ok(PositiveNonzeroInteger(value as u64))
    }
}


#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
