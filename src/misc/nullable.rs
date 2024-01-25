use std::fmt::{self, Display};

pub struct Nullable<T>(pub Option<T>);

impl<T> Nullable<T> {
    pub fn new(value: Option<T>) -> Self {
        Self(value)
    }

    pub fn into_inner(self) -> Option<T> {
        self.0
    }
}

impl<T: Display> Display for Nullable<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(value) => f.write_fmt(format_args!("{}", value)),
            None => f.write_str("-"),
        }
    }
}
