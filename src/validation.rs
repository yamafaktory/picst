///  Validator for percent.
///  Tries to parse as string slice to a `u32`.
pub(crate) fn percent_validator(s: &str) -> Result<u32, String> {
    match s.parse::<u32>() {
        Ok(parsed) => {
            if parsed == 0 || parsed == 100 {
                Err(String::from(
                    "Percentage must be a non-zero integer, one hundred excluded.",
                ))
            } else {
                Ok(parsed)
            }
        }
        Err(_) => Err(format!("`{s}` can't be parsed as a percentage.")),
    }
}

///  Validator for pixels.
///  Tries to parse as string slice to a `u32`.
pub(crate) fn pixels_validator(s: &str) -> Result<u32, String> {
    match s.parse::<u32>() {
        Ok(parsed) => {
            if parsed == 0 {
                Err(String::from("Pixels must be a non-zero integer."))
            } else {
                Ok(parsed)
            }
        }
        Err(_) => Err(format!("`{s}` can't be parsed as pixels.")),
    }
}

///  Validator for ratio.
///  Tries to parse as string slice to a `f32`.
pub(crate) fn ratio_validator(s: &str) -> Result<f32, String> {
    match s.parse::<f32>() {
        Ok(parsed) => {
            if parsed <= 0. || parsed == 1. {
                Err(String::from(
                    "Ratio must be a non-zero float number, one excluded.",
                ))
            } else {
                Ok(parsed)
            }
        }
        Err(_) => Err(format!("`{s}` can't be parsed as a ratio.")),
    }
}

#[cfg(test)]
mod tests {
    use super::{percent_validator, pixels_validator, ratio_validator};

    #[test]
    fn check_percent_validator() {
        assert!(percent_validator("nope").is_err());
        assert!(percent_validator("0").is_err());
        assert!(percent_validator("100").is_err());
        assert!(percent_validator("1").is_ok());
    }

    #[test]
    fn check_pixels_validator() {
        assert!(pixels_validator("nope").is_err());
        assert!(pixels_validator("0").is_err());
        assert!(pixels_validator("1").is_ok());
    }

    #[test]
    fn check_ratio_validator() {
        assert!(ratio_validator("nope").is_err());
        assert!(ratio_validator("-1.").is_err());
        assert!(ratio_validator("0.").is_err());
        assert!(ratio_validator("0.7").is_ok());
    }
}
