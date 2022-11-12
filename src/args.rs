use std::ops::RangeInclusive;

use clap::Parser;
use itertools::all;

const PERCENT_RANGE: RangeInclusive<u32> = 1..=100;
const RATIO_RANGE: RangeInclusive<f32> = 0.0..=1.0;

fn percent_in_range(s: &str) -> Result<u32, String> {
    let percent: u32 = s
        .parse()
        .map_err(|_| format!("`{}` is not a valid percentage", s))?;

    if PERCENT_RANGE.contains(&percent) {
        return Ok(percent);
    }

    Err(format!(
        "Percentage must be an integer between {} and {}",
        PERCENT_RANGE.start(),
        PERCENT_RANGE.end()
    ))
}

fn ratio_in_range(s: &str) -> Result<f32, String> {
    let ratio: f32 = s
        .parse()
        .map_err(|_| format!("`{}` is not a valid ratio", s))?;

    // We need to exclude zero too here (not ideal with a float).
    if RATIO_RANGE.contains(&ratio) && ratio.is_normal() {
        return Ok(ratio);
    }

    Err(String::from("Ratio must a float number > 0 and <= 1"))
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// Height of the resized image in pixels.
    /// Can be combined with `width` in pixels.
    /// Cannot be combined with `width` in percent.
    #[arg(short = 'H', long)]
    pub(crate) height: Option<u32>,

    /// Width of the resized image in pixels.
    /// Can be combined with `height` in pixels.
    /// Cannot be combined with `height` in percent.
    #[arg(short, long)]
    pub(crate) width: Option<u32>,

    /// Height of the resized image in percent.
    /// Can be combined with `width` in percent.
    /// Cannot be combined with `width` in pixels.
    #[arg(
        conflicts_with = "height",
        conflicts_with = "width",
        conflicts_with = "ratio",
        long,
        value_parser = percent_in_range
    )]
    pub(crate) height_percent: Option<u32>,

    /// Width of the resized image in percent.
    /// Can be combined with height in percent.
    /// Cannot be combined with height in pixels.
    #[arg(
        conflicts_with = "height",
        conflicts_with = "width",
        conflicts_with = "ratio",
        long,
        value_parser = percent_in_range
    )]
    pub(crate) width_percent: Option<u32>,

    /// Ratio to resize the image.
    #[arg(
        conflicts_with = "height",
        conflicts_with = "height_percent",
        conflicts_with = "width",
        conflicts_with = "width_percent",
        long,
        value_parser = ratio_in_range
    )]
    pub(crate) ratio: Option<f32>,
}

/// Result of parsing the arguments as an enumeration.
pub(crate) enum ArgsResult {
    /// Dimensions variant as a tuple of (height, width, dimensions in pixels).
    Dimensions(Option<u32>, Option<u32>, bool),
    /// No flags variant.
    NoFlags,
    /// Ratio variant.
    Ratio(f32),
}

impl ArgsResult {
    pub(crate) fn get() -> Self {
        let args = Args::parse();

        // Check if no flags are provided and return the corresponding variant.
        if all(
            [
                args.height,
                args.height_percent,
                args.width,
                args.width_percent,
            ],
            |arg| arg.is_none(),
        ) && args.ratio.is_none()
        {
            return ArgsResult::NoFlags;
        }

        // Check if the ratio is provided and return the corresponding variant.
        if let Some(ratio) = args.ratio {
            return ArgsResult::Ratio(ratio);
        }

        let has_height = args.height.is_some();
        let has_width = args.width.is_some();

        // Re-map the height.
        let height = if has_height {
            args.height
        } else {
            args.height_percent
        };

        // Re-map the width.
        let width = if has_width {
            args.width
        } else {
            args.width_percent
        };

        let dimensions_in_pixels = has_height || has_width;

        // Finally return the dimensions variant.
        ArgsResult::Dimensions(height, width, dimensions_in_pixels)
    }
}

#[test]
fn check_args() {
    use clap::CommandFactory;

    Args::command().debug_assert()
}
