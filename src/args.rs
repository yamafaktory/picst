use clap::Parser;
use itertools::all;

use crate::validation::{percent_validator, ratio_validator};

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
        value_parser = percent_validator
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
        value_parser = percent_validator
    )]
    pub(crate) width_percent: Option<u32>,

    /// Ratio to resize the image.
    #[arg(
        conflicts_with = "height",
        conflicts_with = "height_percent",
        conflicts_with = "width",
        conflicts_with = "width_percent",
        long,
        value_parser = ratio_validator
    )]
    pub(crate) ratio: Option<f32>,
}

/// Result of parsing the arguments as an enumeration.
#[derive(Debug, PartialEq)]
pub(crate) enum ArgsResult {
    /// Dimensions variant as a tuple of (height, width, dimensions in pixels).
    Dimensions(Option<u32>, Option<u32>, bool),
    /// No flags variant.
    NoFlags,
    /// Ratio variant.
    Ratio(f32),
}

impl ArgsResult {
    pub(crate) fn get(args: &Args) -> Self {
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

#[cfg(test)]
mod tests {
    use clap::{CommandFactory, Parser};

    use super::{Args, ArgsResult};

    fn get_args_result(flags: &str) -> ArgsResult {
        let args = Args::parse_from(format!("picst {}", flags).split_whitespace());

        ArgsResult::get(&args)
    }

    #[test]
    /// This is a global test to verify that parsing the arguments doesn't break.
    fn check_args() {
        Args::command().debug_assert()
    }

    #[test]
    fn check_args_result_no_flags() {
        assert_eq!(get_args_result(""), ArgsResult::NoFlags);
    }

    #[test]
    fn check_args_result_full_dimensions_pixels() {
        assert_eq!(
            get_args_result("--height 10 --width 20"),
            ArgsResult::Dimensions(Some(10), Some(20), true)
        );
    }

    #[test]
    fn check_args_result_height_only_pixels() {
        assert_eq!(
            get_args_result("--height 10"),
            ArgsResult::Dimensions(Some(10), None, true)
        );
    }

    #[test]
    fn check_args_result_width_only_pixels() {
        assert_eq!(
            get_args_result("--width 10"),
            ArgsResult::Dimensions(None, Some(10), true)
        );
    }

    #[test]
    fn check_args_result_full_dimensions_percent() {
        assert_eq!(
            get_args_result("--height-percent 10 --width-percent 20"),
            ArgsResult::Dimensions(Some(10), Some(20), false)
        );
    }

    #[test]
    fn check_args_result_height_only_percent() {
        assert_eq!(
            get_args_result("--height-percent 10"),
            ArgsResult::Dimensions(Some(10), None, false)
        );
    }

    #[test]
    fn check_args_result_width_only_percent() {
        assert_eq!(
            get_args_result("--width-percent 10"),
            ArgsResult::Dimensions(None, Some(10), false)
        );
    }

    #[test]
    fn check_args_result_ratio() {
        assert_eq!(get_args_result("--ratio 0.7"), ArgsResult::Ratio(0.7));
    }
}
