use anyhow::Result;
use arboard::ImageData;
use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};

use crate::{
    args::{Args, ArgsMetadata, ArgsResult},
    dimension::Dimension,
    unit::Unit,
    validation::{percent_validator, pixels_validator, ratio_validator},
};

/// Simple type alias.
type DimensionTuple = (u32, u32);

/// Returns the dimension selected by the user.
fn get_dimension_selector() -> Result<Dimension> {
    // Use a select to get the unit.
    let index = Select::with_theme(&ColorfulTheme::default())
        .items(Dimension::get_items())
        .default(0)
        .interact_on(&Term::stderr())?;

    index.try_into().map_err(anyhow::Error::msg)
}

/// Prompts the user for a dimension value.
fn get_dimension_value_prompt(dimension: Dimension, is_pixel: bool) -> Result<u32> {
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
fn get_ratio_prompt() -> Result<f32> {
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

/// Returns the unit selected by the user.
fn get_unit_selector() -> Result<Unit> {
    // Use a select to get the unit.
    let index = Select::with_theme(&ColorfulTheme::default())
        .items(Unit::get_items())
        .default(0)
        .interact_on(&Term::stderr())?;

    index.try_into().map_err(anyhow::Error::msg)
}

/// Takes an image and a ratio, returns the new dimensions as a tuple.
fn apply_ratio_to_image(image: &ImageData, ratio: f32) -> DimensionTuple {
    (
        (image.height as f32 * ratio) as u32,
        (image.width as f32 * ratio) as u32,
    )
}

/// Conditionally applies a percent on the provided original size or directly
/// returns the value.
fn maybe_apply_percent(
    original_size: usize,
    target_size_pixels_or_percent: u32,
    is_pixel: bool,
) -> u32 {
    if is_pixel {
        target_size_pixels_or_percent
    } else {
        (original_size as u32 * target_size_pixels_or_percent) / 100
    }
}

/// Takes the image dimensions, the current dimension and either apply the
/// percentage or the inner ratio of the image.
fn resize(
    image_first_dimension: usize,
    current_dimension: u32,
    image_second_dimension: usize,
    is_pixel: bool,
) -> u32 {
    // For percentage, we don't need to calculate any ratio since this is
    // already a ratio on its own.
    if !is_pixel {
        return (image_second_dimension as u32 * current_dimension) / 100;
    }

    // We need to do some casting to keep the ratio correct.
    let ratio: f32 = image_first_dimension as f32 / current_dimension as f32;

    (image_second_dimension as f32 / ratio) as u32
}

/// Creates a full wizard which returns a tuple of (height, width).
/// It will prompt or not the user based on the parsed arguments.
pub(crate) fn create_wizard(args: &Args, image: &ImageData) -> Result<DimensionTuple> {
    match ArgsResult::get(args) {
        // If dimensions are passed, we eventually need to prompt for the
        // missing height or width.
        // Note: we also need to take care of the aspect ratio.
        ArgsResult::Dimensions(
            height,
            width,
            ArgsMetadata {
                is_pixel,
                ignore_aspect_ratio,
            },
        ) => {
            let (height, width) = match (height, width, ignore_aspect_ratio) {
                (Some(height), None, false) => (
                    maybe_apply_percent(image.height, height, is_pixel),
                    resize(image.height, height, image.width, is_pixel),
                ),
                (None, Some(width), false) => (
                    resize(image.width, width, image.height, is_pixel),
                    maybe_apply_percent(image.width, width, is_pixel),
                ),
                (Some(height), None, true) => (
                    maybe_apply_percent(image.height, height, is_pixel),
                    maybe_apply_percent(
                        image.width,
                        get_dimension_value_prompt(Dimension::Width, is_pixel)?,
                        is_pixel,
                    ),
                ),
                (None, Some(width), true) => (
                    maybe_apply_percent(
                        image.height,
                        get_dimension_value_prompt(Dimension::Height, is_pixel)?,
                        is_pixel,
                    ),
                    maybe_apply_percent(image.width, width, is_pixel),
                ),
                (Some(height), Some(width), false) => (
                    maybe_apply_percent(image.height, height, is_pixel),
                    maybe_apply_percent(image.width, width, is_pixel),
                ),
                // We can't have height, width and ignore aspect ratio.
                _ => unreachable!(),
            };

            Ok((height, width))
        }
        // If no flags are passed, we first need to get the unit.
        // In case of a ratio, simply prompt the user.
        // Otherwise, prompt first for the height / width or both and then
        // prompt for the necessary dimension(s).
        // Note: we also need to take care of the aspect ratio.
        ArgsResult::NoFlags => {
            let unit = get_unit_selector()?;

            if unit == Unit::Ratio {
                let ratio = get_ratio_prompt()?;

                // Return the new dimensions based on the ratio.
                Ok(apply_ratio_to_image(image, ratio))
            } else {
                let dimensions_in_pixels = unit == Unit::Pixel;

                let (height, width) = match get_dimension_selector()? {
                    Dimension::Height => {
                        let height =
                            get_dimension_value_prompt(Dimension::Height, dimensions_in_pixels)?;

                        (
                            maybe_apply_percent(image.height, height, dimensions_in_pixels),
                            resize(image.height, height, image.width, dimensions_in_pixels),
                        )
                    }
                    Dimension::Width => {
                        let width =
                            get_dimension_value_prompt(Dimension::Width, dimensions_in_pixels)?;

                        (
                            resize(image.width, width, image.height, dimensions_in_pixels),
                            maybe_apply_percent(image.width, width, dimensions_in_pixels),
                        )
                    }
                    Dimension::Both => (
                        maybe_apply_percent(
                            image.height,
                            get_dimension_value_prompt(Dimension::Height, dimensions_in_pixels)?,
                            dimensions_in_pixels,
                        ),
                        maybe_apply_percent(
                            image.width,
                            get_dimension_value_prompt(Dimension::Width, dimensions_in_pixels)?,
                            dimensions_in_pixels,
                        ),
                    ),
                };

                Ok((height, width))
            }
        }
        // If a ratio is passed, just use it.
        ArgsResult::Ratio(ratio) => Ok(apply_ratio_to_image(image, ratio)),
    }
}
