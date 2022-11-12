use std::convert::TryFrom;

use anyhow::Result;
use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};

use crate::args::ArgsResult;

static UNITS: &[&str; 3] = &["Pixel", "Percentage", "Ratio"];

/// Enumeration for the dimension.
enum Dimension {
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

/// Enumeration for the unit.
#[derive(PartialEq)]
enum Unit {
    Pixel = 0,
    Percentage = 1,
    Ratio = 2,
}

impl Unit {
    fn get_items() -> &'static [&'static str; 3] {
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

/// Wizard result as an enumeration.
pub(crate) enum WizardResult {
    /// Dimensions variant as a tuple of (height, width, dimensions in pixels).
    Dimensions(u32, u32, bool),
    // Ratio variant.
    Ratio(f32),
}

/// Returns the unit selected by the user.
fn get_unit() -> Result<Unit> {
    // Use a select to get the unit.
    let index = Select::with_theme(&ColorfulTheme::default())
        .items(Unit::get_items())
        .default(0)
        .interact_on(&Term::stderr())?;

    index.try_into().map_err(anyhow::Error::msg)
}

/// Prompts the user for a dimension.
fn get_dimension(dimension: Dimension) -> Result<u32> {
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

/// Prompts the user for a ratio.
fn get_ratio() -> Result<f32> {
    // Use a prompt to get the desired value.
    let value: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Ratio")
        .validate_with({
            move |input: &String| -> Result<(), &str> {
                // Check if parsing the input to a f32 is fine.
                if input.parse::<f32>().is_ok() {
                    return Ok(());
                }

                Err("Please enter a valid number!")
            }
        })
        .interact_text()?;

    // This can't fail since parsing has been safely checked above.
    value.parse::<f32>().map_err(anyhow::Error::msg)
}

/// Creates a full wizard which returns all the necessary information.
/// It will prompt or not the user based on the parsed arguments.
pub(crate) fn create_wizard() -> Result<WizardResult> {
    match ArgsResult::get() {
        // If dimensions are passed, we eventually need to prompt for the
        // missing height or width.
        ArgsResult::Dimensions(height, width, dimensions_in_pixels) => {
            let height = match height {
                Some(height) => height,
                None => get_dimension(Dimension::Height)?,
            };
            let width = match width {
                Some(width) => width,
                None => get_dimension(Dimension::Width)?,
            };

            Ok(WizardResult::Dimensions(
                height,
                width,
                dimensions_in_pixels,
            ))
        }
        // If no flags are passed, we first need to get the unit.
        // In case of a ratio, prompt the user.
        // Otherwise, prompt fot the height and the width.
        ArgsResult::NoFlags => {
            let unit = get_unit()?;

            if unit == Unit::Ratio {
                let ratio = get_ratio()?;

                Ok(WizardResult::Ratio(ratio))
            } else {
                let height = get_dimension(Dimension::Height)?;
                let width = get_dimension(Dimension::Width)?;

                Ok(WizardResult::Dimensions(height, width, unit == Unit::Pixel))
            }
        }
        // If a ratio is passed, just use it.
        ArgsResult::Ratio(ratio) => Ok(WizardResult::Ratio(ratio)),
    }
}
