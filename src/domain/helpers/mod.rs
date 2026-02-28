use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref VALID_CHARS_REGEX: Regex = Regex::new(r"^[\p{L}\s-]+$").unwrap();
    pub static ref BIO_VALID_CHARS_REGEX: Regex = Regex::new(r"^[\p{L}\p{N}\s._-]+$").unwrap();
    pub static ref EMAIL_REGEX: Regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
}
