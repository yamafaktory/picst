use clap::{error::ErrorKind, CommandFactory, Parser};
use dialoguer::console::style;
use itertools::all;

use crate::validation::{percent_validator, ratio_validator};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// Height of the resized image in pixels.
    /// Can be combined with `width` in pixels.
    /// Cannot be combined with `width` in percent.
    #[arg(conflicts_with = "ratio", long, short = 'H')]
    pub(crate) height: Option<u32>,

    /// Width of the resized image in pixels.
    /// Can be combined with `height` in pixels.
    /// Cannot be combined with `height` in percent.
    #[arg(conflicts_with = "ratio", long, short)]
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
        long,
        conflicts_with = "ignore_aspect_ratio",
        value_parser = ratio_validator
    )]
    pub(crate) ratio: Option<f32>,

    /// Ignore aspect ratio.
    #[arg(conflicts_with = "ratio", long)]
    pub(crate) ignore_aspect_ratio: bool,
}

impl Args {
    /// Custom parser with some additional checking.
    pub(crate) fn custom_parse() -> Args {
        let args = Args::parse();
        let mut cmd = Args::command();

        // The `ignore_aspect_ratio` flag can be mixed with `height` and `width`.
        if (args.height.is_some() && args.width.is_some()) && args.ignore_aspect_ratio {
            cmd.error(
                ErrorKind::ArgumentConflict,
                // Note: there's no coloring API exposed with Clap.
                // https://github.com/clap-rs/clap/issues/2035
                format!(
                    "The argument '{}' cannot be used with '{}' and '{}'",
                    style("--ignore-aspect-ratio").yellow(),
                    style("--height").yellow(),
                    style("--width").yellow()
                ),
            )
            .exit();
        }

        // The `ignore_aspect_ratio` flag can be mixed with `height-percent` and `width-percent`.
        if (args.height_percent.is_some() && args.height_percent.is_some())
            && args.ignore_aspect_ratio
        {
            cmd.error(
                ErrorKind::ArgumentConflict,
                format!(
                    "The argument '{}' cannot be used with '{}' and '{}'",
                    style("--ignore-aspect-ratio").yellow(),
                    style("--height-percent").yellow(),
                    style("--width-percent").yellow()
                ),
            )
            .exit();
        }

        args
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct ArgsMetadata {
    pub(crate) is_pixel: bool,
    pub(crate) ignore_aspect_ratio: bool,
}

impl ArgsMetadata {
    fn new(is_pixel: bool, ignore_aspect_ratio: bool) -> Self {
        Self {
            is_pixel,
            ignore_aspect_ratio,
        }
    }
}

/// Result of parsing the arguments as an enumeration.
#[derive(Debug, PartialEq)]
pub(crate) enum ArgsResult {
    /// Dimensions variant as a tuple of (height, width, dimensions in pixels,
    /// ignore aspect ratio).
    Dimensions(Option<u32>, Option<u32>, ArgsMetadata),
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

        let has_pixel_height = args.height.is_some();
        let has_pixel_width = args.width.is_some();

        // Re-map the height.
        let height = if has_pixel_height {
            args.height
        } else {
            args.height_percent
        };

        // Re-map the width.
        let width = if has_pixel_width {
            args.width
        } else {
            args.width_percent
        };

        // Finally return the dimensions variant.
        ArgsResult::Dimensions(
            height,
            width,
            ArgsMetadata::new(
                has_pixel_height || has_pixel_width,
                args.ignore_aspect_ratio,
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use clap::{CommandFactory, Parser};

    use super::{Args, ArgsMetadata, ArgsResult};

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
            ArgsResult::Dimensions(Some(10), Some(20), ArgsMetadata::new(true, false))
        );
    }

    #[test]
    fn check_args_result_height_only_pixels() {
        assert_eq!(
            get_args_result("--height 10"),
            ArgsResult::Dimensions(Some(10), None, ArgsMetadata::new(true, false))
        );
    }

    #[test]
    fn check_args_result_width_only_pixels() {
        assert_eq!(
            get_args_result("--width 10"),
            ArgsResult::Dimensions(None, Some(10), ArgsMetadata::new(true, false))
        );
    }

    #[test]
    fn check_args_result_full_dimensions_percent() {
        assert_eq!(
            get_args_result("--height-percent 10 --width-percent 20"),
            ArgsResult::Dimensions(Some(10), Some(20), ArgsMetadata::new(false, false))
        );
    }

    #[test]
    fn check_args_result_height_only_percent() {
        assert_eq!(
            get_args_result("--height-percent 10"),
            ArgsResult::Dimensions(Some(10), None, ArgsMetadata::new(false, false))
        );
    }

    #[test]
    fn check_args_result_width_only_percent() {
        assert_eq!(
            get_args_result("--width-percent 10"),
            ArgsResult::Dimensions(None, Some(10), ArgsMetadata::new(false, false))
        );
    }

    #[test]
    fn check_args_result_height_only_ignore_aspect_ratio() {
        assert_eq!(
            get_args_result("--height 10 --ignore-aspect-ratio"),
            ArgsResult::Dimensions(Some(10), None, ArgsMetadata::new(true, true))
        );
    }

    #[test]
    fn check_args_result_width_only_ignore_aspect_ratio() {
        assert_eq!(
            get_args_result("--width 10 --ignore-aspect-ratio"),
            ArgsResult::Dimensions(None, Some(10), ArgsMetadata::new(true, true))
        );
    }

    #[test]
    fn check_args_result_height_only_percent_ignore_aspect_ratio() {
        assert_eq!(
            get_args_result("--height-percent 10 --ignore-aspect-ratio"),
            ArgsResult::Dimensions(Some(10), None, ArgsMetadata::new(false, true))
        );
    }

    #[test]
    fn check_args_result_width_only_percent_ignore_aspect_ratio() {
        assert_eq!(
            get_args_result("--width-percent 10 --ignore-aspect-ratio"),
            ArgsResult::Dimensions(None, Some(10), ArgsMetadata::new(false, true))
        );
    }

    #[test]
    fn check_args_result_ratio() {
        assert_eq!(get_args_result("--ratio 0.7"), ArgsResult::Ratio(0.7));
    }
}
