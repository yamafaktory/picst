use std::ops::RangeInclusive;

use clap::Parser;

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
        "Percentage not in range {}-{}",
        PERCENT_RANGE.start(),
        PERCENT_RANGE.end()
    ))
}

fn ratio_in_range(s: &str) -> Result<f32, String> {
    let ratio: f32 = s
        .parse()
        .map_err(|_| format!("`{}` is not a valid ratio", s))?;

    if RATIO_RANGE.contains(&ratio) {
        return Ok(ratio);
    }

    Err(format!(
        "Ratio not in range {}-{}",
        RATIO_RANGE.start(),
        RATIO_RANGE.end()
    ))
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// Height of the resized image in pixels.
    #[arg(short = 'H', long)]
    pub(crate) height: Option<u32>,

    /// Width of the resized image in pixels.
    #[arg(short, long)]
    pub(crate) width: Option<u32>,

    /// Height of the resized image in percent.
    #[arg(
        conflicts_with = "height",
        conflicts_with = "ratio",
        long,
        value_parser = percent_in_range
    )]
    pub(crate) height_percent: Option<u32>,

    /// Width of the resized image in percent.
    #[arg(conflicts_with = "ratio", conflicts_with = "width", long)]
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

#[test]
fn check_args() {
    use clap::CommandFactory;

    Args::command().debug_assert()
}
