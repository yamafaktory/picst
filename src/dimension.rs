use std::convert::TryFrom;

use anyhow::Result;
use arboard::ImageData;
use dialoguer::{console::Term, theme::ColorfulTheme, Confirm, Input, Select};

use crate::{
    args::{Args, ArgsResult},
    validation::{percent_validator, pixels_validator, ratio_validator},
};

/// Units used for the select prompt.
static UNITS: &[&str; 3] = &["Pixel", "Percentage", "Ratio"];

/// Enumeration for the dimension.
enum Dimension {
    Height,
    Width,
}

impl Dimension {
    /// Returns the name associated with the variant.
    fn get_name(self) -> &'static str {
        match self {
            Dimension::Height => "Height",
            Dimension::Width => "Width",
        }
    }
}

/// Enumeration for the unit.
#[derive(Debug, PartialEq)]
enum Unit {
    Pixel = 0,
    Percentage = 1,
    Ratio = 2,
}

impl Unit {
    /// Returns the static units.
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

fn get_preserve_aspect_ratio() -> Result<()> {
    if Confirm::new()
        .with_prompt("Do you want to continue?")
        .interact()?
    {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
    }

    Ok(())
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
fn get_dimension(dimension: Dimension, is_pixel: bool) -> Result<u32> {
    // Use a prompt to get the desired value.
    let value: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(dimension.get_name())
        .validate_with({
            move |input: &String| -> Result<(), String> {
                // Validate the input as pixels or percent.
                if is_pixel {
                    pixels_validator(input).map(|_| ())
                } else {
                    percent_validator(input).map(|_| ())
                }
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
            move |input: &String| -> Result<(), String> {
                // Validate the input as ratio.
                ratio_validator(input).map(|_| ())
            }
        })
        .interact_text()?;

    // This can't fail since parsing has been safely checked above.
    value.parse::<f32>().map_err(anyhow::Error::msg)
}

/// Creates a full wizard which returns all the necessary information.
/// It will prompt or not the user based on the parsed arguments.
pub(crate) fn create_wizard(args: &Args, image: &ImageData) -> Result<WizardResult> {
    dbg!(ArgsResult::get(args));
    match ArgsResult::get(args) {
        // If dimensions are passed, we eventually need to prompt for the
        // missing height or width.
        // We also need to take care of the aspect ratio.
        ArgsResult::Dimensions(height, width, dimensions_in_pixels, ignore_aspect_ratio) => {
            let (height, width) = match (height, width, ignore_aspect_ratio) {
                (Some(height), None, false) => {
                    (height, (image.height as u32 / height) * image.width as u32)
                }
                (Some(height), None, true) => (
                    height,
                    get_dimension(Dimension::Width, dimensions_in_pixels)?,
                ),
                (None, Some(width), false) => {
                    ((image.width as u32 / width) * image.height as u32, width)
                }
                (None, Some(width), true) => (
                    (image.width as u32 / width) * image.height as u32,
                    get_dimension(Dimension::Height, dimensions_in_pixels)?,
                ),
                (Some(height), Some(width), false) => (height, width),
                // We can't have height, width and ignore aspect ratio.
                _ => unreachable!(),
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
                let is_pixel = unit == Unit::Pixel;
                let height = get_dimension(Dimension::Height, is_pixel)?;
                let width = get_dimension(Dimension::Width, is_pixel)?;

                Ok(WizardResult::Dimensions(height, width, unit == Unit::Pixel))
            }
        }
        // If a ratio is passed, just use it.
        ArgsResult::Ratio(ratio) => Ok(WizardResult::Ratio(ratio)),
    }
}

#[cfg(test)]
mod tests {

    use super::{Dimension, Unit, UNITS};

    #[test]
    fn check_dimension() {
        assert_eq!(Dimension::Height.get_name(), "Height");
        assert_eq!(Dimension::Width.get_name(), "Width");
    }

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
