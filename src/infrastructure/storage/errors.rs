use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct MultiError {
    errors: Vec<Box<dyn Error>>,
}


impl MultiError {
    pub fn new(errors: Vec<Box<dyn Error>>) -> Self {
        MultiError { errors }
    }
}
impl Display for MultiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, error) in self.errors.iter().enumerate() {
            write!(f, "{}: {}", i, error)?;
        }
        Ok(())
    }
}

impl Error for MultiError {}