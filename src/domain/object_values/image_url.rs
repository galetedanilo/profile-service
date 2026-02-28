use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ImageUrl(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum ImageUrlError {
    #[error("Invalid URL format")]
    InvalidUrl,

    #[error("URL must start with http:// or https://")]
    InvalidScheme,

    #[error("URL must end with a valid image extension (.jpg, .jpeg, .png, .gif)")]
    InvalidExtension,

    #[error("URL cannot be empty")]
    Empty,

    #[error("URL is too long (maximum {0} characters)")]
    TooLong(usize),
}

impl ImageUrl {
    const MAX_LENGTH: usize = 2048;

    pub fn try_new(value: String) -> Result<Self, ImageUrlError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ImageUrlError::Empty);
        }

        if trimmed.len() > Self::MAX_LENGTH {
            return Err(ImageUrlError::TooLong(Self::MAX_LENGTH));
        }

        if !trimmed.starts_with("http://") && !trimmed.starts_with("https://") {
            return Err(ImageUrlError::InvalidScheme);
        }

        let valid_extensions = [".jpg", ".jpeg", ".png", ".gif"];
        if !valid_extensions.iter().any(|ext| trimmed.ends_with(ext)) {
            return Err(ImageUrlError::InvalidExtension);
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for ImageUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for ImageUrl {
    type Error = ImageUrlError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl TryFrom<&str> for ImageUrl {
    type Error = ImageUrlError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_new(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_image_url_empty_should_empty_error() {
        let image_url = ImageUrl::try_new("".to_string());
        assert!(matches!(image_url, Err(ImageUrlError::Empty)));
    }

    #[test]
    fn when_image_url_too_long_should_too_long_error() {
        let long_url = "https://example.com/".repeat(200); // 200 * 19 = 3800 characters
        let image_url = ImageUrl::try_new(long_url);
        assert!(matches!(image_url, Err(ImageUrlError::TooLong(_))));
    }

    #[test]
    fn when_image_url_invalid_scheme_should_invalid_scheme_error() {
        let image_url = ImageUrl::try_new("ftp://example.com/image.jpg".to_string());
        assert!(matches!(image_url, Err(ImageUrlError::InvalidScheme)));
    }

    #[test]
    fn when_image_url_invalid_extension_should_invalid_extension_error() {
        let image_url = ImageUrl::try_new("https://example.com/image.txt".to_string());
        assert!(matches!(image_url, Err(ImageUrlError::InvalidExtension)));
    }

    #[test]
    fn when_image_url_valid_should_create_image_url() {
        let url = "https://example.com/image.jpg";
        let image_url = ImageUrl::try_new(url.to_string()).unwrap();
        assert_eq!(image_url.as_ref(), url);
    }

    #[test]
    fn when_image_url_valid_should_create_image_url_from_str() {
        let url = "https://example.com/image.jpg";
        let image_url = ImageUrl::try_from(url).unwrap();
        assert_eq!(image_url.as_ref(), url);
    }

    #[test]
    fn when_image_url_valid_should_create_image_url_from_string() {
        let url = "https://example.com/image.jpg".to_string();
        let image_url = ImageUrl::try_from(url.clone()).unwrap();
        assert_eq!(image_url.as_ref(), url);
    }

    #[test]
    fn when_image_url_valid_should_display_image_url() {
        let url = "https://example.com/image.jpg";
        let image_url = ImageUrl::try_new(url.to_string()).unwrap();
        assert_eq!(format!("{}", image_url), url);
    }
}
