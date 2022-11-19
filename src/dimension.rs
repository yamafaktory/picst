use std::convert::TryFrom;

use anyhow::Result;

/// Units used for the select prompt.
static DIMENSIONS: &[&str; 3] = &["Height", "Width", "Both"];

/// Enumeration for the dimension.
#[derive(Debug, PartialEq)]
pub(crate) enum Dimension {
    Height = 0,
    Width = 1,
    Both = 2,
}

impl Dimension {
    /// Returns the name associated with the variant.
    pub(crate) fn get_name(self) -> &'static str {
        match self {
            Dimension::Height => DIMENSIONS[0],
            Dimension::Width => DIMENSIONS[1],
            Dimension::Both => DIMENSIONS[2],
        }
    }

    /// Returns the static dimensions.
    pub(crate) fn get_items() -> &'static [&'static str; 3] {
        DIMENSIONS
    }
}

impl TryFrom<usize> for Dimension {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            u if u == Dimension::Height as usize => Ok(Dimension::Height),
            u if u == Dimension::Width as usize => Ok(Dimension::Width),
            u if u == Dimension::Both as usize => Ok(Dimension::Both),
            // Unreachable.
            _ => Err("Index cannot be converted to dimension."),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{Dimension, DIMENSIONS};

    #[test]
    fn check_dimension() {
        assert_eq!(Dimension::Height.get_name(), "Height");
        assert_eq!(Dimension::Width.get_name(), "Width");

        assert_eq!(Dimension::get_items(), DIMENSIONS);

        let zero_to_dimension: Dimension = 0usize.try_into().unwrap();
        assert_eq!(zero_to_dimension, Dimension::Height);

        let one_to_dimension: Dimension = 1usize.try_into().unwrap();
        assert_eq!(one_to_dimension, Dimension::Width);

        let two_to_dimension: Dimension = 2usize.try_into().unwrap();
        assert_eq!(two_to_dimension, Dimension::Both);
    }
}
