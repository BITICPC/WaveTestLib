use std::str::FromStr;
use std::string::ToString;
use std::fmt::{Display, Debug};
use std::cmp::Ordering;
use std::process::exit;

use crate::tokenized::TokenizedRead;
use crate::cmp::compare_floats;


/// Exit code of the process that indicates an `Accepted` result.
pub const EXIT_ACCEPTED: i32 = 0;

/// Exit code of the process that indicates a `Rejected` result.
pub const EXIT_REJECTED: i32 = -1;

/// Exit the program with an accepted result.
pub fn accept<'a>(message: Option<&'a str>) -> ! {
    match message {
        Some(msg) => eprintln!("Accepted: {}", msg),
        None => eprintln!("Accepted.")
    };
    exit(EXIT_ACCEPTED)
}

/// Exit the program with a rejected result. The reason why the solution is
/// rejected should be included in the given message.
pub fn reject<'a>(message: &'a str) -> ! {
    eprintln!("Rejected: {}", message);
    exit(EXIT_REJECTED)
}

/// Macro that provide simplified access to the [`accept`] function. The 
/// arguments to this macro can be empty or the same as those arguments to the 
/// `format!` macro.
#[macro_export]
macro_rules! accept {
    () => {
        $crate::contract::accept(None);
    };
    ($($arg:tt)*) => {
        $crate::contract::accept(Some(format!($($arg)*).as_str()));
    };
}

/// Macro that provide simplified access to the [`reject`] function. The 
/// arguments to this macro are the same as those arguments to the `format!` 
/// macro.
#[macro_export]
macro_rules! reject {
    ($($arg:tt)*) => {
        $crate::contract::reject(format!($($arg)*).as_str());
    };
}

/// Provide a formatted reader that can be used in judge.
pub struct JudgeReader<T: TokenizedRead> {
    inner: T
}

impl<T: TokenizedRead> JudgeReader<T> {
    /// Get the inner reader of the [`JudgeReader`] instance.
    pub fn inner_reader(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Read one token from the underlying reader.
    pub fn read_token(&mut self) -> Option<String> {
        self.inner.read_token()
    }

    /// Read one token from the underlying reader and convert it to the given
    /// type. Panics if the token read cannot be converted to the given type.
    /// Returns `None` if the underlying reader returns `None`.
    pub fn read_token_as<U>(&mut self) -> Option<U>
        where U: FromStr {
        self.read_token()
            .map(|token| U::from_str(token.as_str())
                .ok()
                .expect("failed to convert token to the given type."))
    }

    /// Expect the next token from the inner reader to be the given value's 
    /// string representation. If `ignore_case` is true, then a string 
    /// comparison ignoring ASCII case will be performed.
    pub fn expect_token<U>(&mut self, expected: &U, ignore_case: bool) -> String
        where U: ?Sized + ToString {
        let token = match self.inner.read_token() {
            Some(token) => token,
            None => reject!("Unexpected EOF.")
        };
        let expected = expected.to_string();
        
        let pass = 
            if ignore_case {
                token.eq_ignore_ascii_case(expected.as_str())
            } else {
                token.eq(expected.as_str())
            };
        
        if !pass {
            reject!("expect \"{}\", found \"{}\"", expected, token);
        }

        token
    }

    /// Expect the next token from the inner reader can be converted to the 
    /// given type.
    pub fn expect_type<U>(&mut self) -> U
        where U: FromStr {
        let token = match self.inner.read_token() {
            Some(token) => token,
            None => reject!("Unexpected EOF.")
        };
        match U::from_str(token.as_str()) {
            Ok(value) => value,
            Err(..) => reject!("Unexpected token: \"{}\"", token)
        }
    }

    /// Expect the next token from the inner reader can be converted to the 
    /// given type and satisfies the given predicate.
    pub fn expect_value_that<U, F, E>(&mut self, predicate: F) -> U
        where U: FromStr + Display,
              F: FnOnce(&U) -> Result<(), E>,
              E: Debug {
        let token = match self.inner.read_token() {
            Some(token) => token,
            None => reject!("Unexpected EOF.")
        };
        match U::from_str(token.as_str()) {
            Ok(token_value) => {
                match predicate(&token_value) {
                    Ok(..) => token_value,
                    Err(err) => {
                        reject!("Unexpected value: \"{}\": {:?}", 
                            token_value, err)
                    }
                }
            },
            Err(..) => reject!("Unexpected token: \"{}\"", token)
        }
    }

    /// Expect the next token from the inner reader can be converted to the 
    /// given type and equals the given value.
    pub fn expect_eq<U, V>(&mut self, value: &V) -> U
        where U: FromStr + PartialEq<V> + Display, 
              V: ?Sized + Display {
        self.expect_value_that(|token_value: &U| if token_value.eq(value) {
            Ok(())
        } else {
            Err(format!("expected value: \"{}\"", value))
        })
    }

    /// Expect the next token from the given reader can be converted to the 
    /// given type but not equals to the given value.
    pub fn expect_ne<U, V>(&mut self, value: &V) -> U
        where U: FromStr + PartialEq<V> + Display,
              V: ?Sized + Display {
        self.expect_value_that(|token_value: &U| if token_value.ne(value) {
            Ok(())
        } else {
            Err(())
        })
    }

    /// Expect the next token from the inner reader can be converted to `f64` 
    /// and equals to the given value with an absolute tolerance.
    pub fn expect_float_eq(&mut self, expected: f64, tolerance: f64) -> f64 {
        self.expect_value_that(|value: &f64| 
            match compare_floats(*value, expected, tolerance) {
                Some(Ordering::Equal) => Ok(()),
                _ => Err(format!("expected \"{}\", found \"{}\"", 
                    expected, *value))
            })
    }

    /// Expect the next token from the inner reader can be converted to `f64` 
    /// but not equals to the given value with an absolute tolerance.
    pub fn expect_float_ne(&mut self, expected: f64, tolerance: f64) -> f64 {
        self.expect_value_that(|value: &f64| 
            match compare_floats(*value, expected, tolerance) {
                Some(Ordering::Equal) => Err(format!("unexpected value: \"{}\"", 
                    *value)),
                _ => Ok(())
            })
    }

    /// Expect EOF has been hit on the inner reader.
    pub fn expect_eof(&mut self) {
        match self.inner.read_token() {
            Some(token) => reject!("EOF expected, but found \"{}\"", token),
            None => ()
        }
    }
}


pub mod preclude {
    pub use super::EXIT_ACCEPTED;
    pub use super::EXIT_REJECTED;
    pub use super::accept;
    pub use super::reject;
    pub use super::JudgeReader;
}
