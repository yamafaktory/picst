use std::convert::TryFrom;

use anyhow::Result;

/// Units used for the select prompt.
static UNITS: &[&str; 3] = &["Pixel", "Percentage", "Ratio"];

/// Enumeration for the unit.
#[derive(Debug, PartialEq)]
pub(crate) enum Unit {
    Pixel = 0,
    Percentage = 1,
    Ratio = 2,
}

impl Unit {
    /// Returns the static units.
    pub(crate) fn get_items() -> &'static [&'static str; 3] {
        UNITS
    }
}

impl TryFrom<usize> for Unit {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            u if u == Unit::Pixel as usize => Ok(Unit::Pixel),
            u if u == Unit::Percentage as usize => Ok(Unit::Percentage),
            u if u == Unit::Ratio as usize => Ok(Unit::Ratio),
            // Unreachable.
            _ => Err("Index cannot be converted to unit."),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{Unit, UNITS};

    #[test]
    fn check_unit() {
        assert_eq!(Unit::get_items(), UNITS);

        let zero_to_unit: Unit = 0usize.try_into().unwrap();
        assert_eq!(zero_to_unit, Unit::Pixel);

        let one_to_unit: Unit = 1usize.try_into().unwrap();
        assert_eq!(one_to_unit, Unit::Percentage);

        let two_to_unit: Unit = 2usize.try_into().unwrap();
        assert_eq!(two_to_unit, Unit::Ratio);
    }
}
