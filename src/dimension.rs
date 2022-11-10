use std::convert::TryFrom;

use anyhow::Result;
use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};

static UNITS: &[&str; 3] = &["Pixel", "Percentage", "Ratio"];

/// Enumeration for the dimension.
pub(crate) enum Dimension {
    Height,
    Width,
}

impl Dimension {
    fn get_name(self) -> &'static str {
        match self {
            Dimension::Height => "Height",
            Dimension::Width => "Width",
        }
    }
}

/// Enumeration for the operation.
pub(crate) enum Unit {
    Pixel = 0,
    Percentage = 1,
    Ratio = 2,
}

impl Unit {
    fn get_items() -> &'static [&'static str; 3] {
        UNITS
    }

    fn get_name(self) -> &'static str {
        match self {
            Unit::Pixel => UNITS[0],
            Unit::Percentage => UNITS[1],
            Unit::Ratio => UNITS[2],
        }
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

/// Returns the unit selected by the user.
pub(crate) fn get_unit() -> Result<Unit> {
    // Use a select to get the unit.
    let index = Select::with_theme(&ColorfulTheme::default())
        .items(Unit::get_items())
        .default(0)
        .interact_on(&Term::stderr())?;

    index.try_into().map_err(anyhow::Error::msg)
}

/// Returns the provided default dimension or prompt the user for a value.
pub(crate) fn get_dimension(dimension: Dimension, maybe_value: Option<u32>) -> Result<u32> {
    // Skip prompt if a default is provided.
    if let Some(value) = maybe_value {
        return Ok(value);
    }

    // Use a prompt to get the desired value.
    let value: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(dimension.get_name())
        .validate_with({
            move |input: &String| -> Result<(), &str> {
                // Check if parsing the input to a u32 is fine.
                if input.parse::<u32>().is_ok() {
                    return Ok(());
                }

                Err("Please enter a valid number!")
            }
        })
        .interact_text()?;

    // This can't fail since parsing has been safely checked above.
    value.parse::<u32>().map_err(anyhow::Error::msg)
}
