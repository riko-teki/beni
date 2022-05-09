use std::fmt::{Display, Formatter};

pub trait Color {
    fn black(self) -> String;
}

impl Color for &str {
    fn black(self) -> String {
        return "black!".to_string();
    }
}

pub struct CsiString {}

impl CsiString {}

impl Display for CsiString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}
