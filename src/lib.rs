#[derive(Debug, PartialEq)]
pub enum MyError {
    ErrorType(Details),
}

#[derive(Debug, PartialEq)]
pub enum Details {
    NoDetails,
}

pub fn return_result() -> Result<(), MyError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_error_type() {
        let err = return_result().unwrap_err();
        assert_eq!(err, MyError::ErrorType);
    }
}
