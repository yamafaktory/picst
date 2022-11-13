use dialoguer::console::{style, Emoji};
use image::{ImageBuffer, Rgba};
use indicatif::HumanDuration;
use tokio::time::Instant;

static CLIPBOARD: Emoji<'_, '_> = Emoji("📋 ", "");
static HEIGHT: Emoji<'_, '_> = Emoji("↕️ ", "");
static WIDTH: Emoji<'_, '_> = Emoji("↔️ ", "");
static ZAP: Emoji<'_, '_> = Emoji("⚡", "");

type ImageBufferU8 = ImageBuffer<Rgba<u8>, Vec<u8>>;

/// Item produced by the stream.
/// Note: The fields are kept private and exposed by the implementations.
pub(crate) struct ResizedImage {
    image_buffer: ImageBufferU8,
    original_height: u32,
    original_width: u32,
    start_time: Instant,
}

impl ResizedImage {
    pub(crate) fn new(
        image_buffer: ImageBufferU8,
        original_height: u32,
        original_width: u32,
        start_time: Instant,
    ) -> Self {
        Self {
            image_buffer,
            original_height,
            original_width,
            start_time,
        }
    }

    pub(crate) fn get_buffer(&mut self) -> &ImageBufferU8 {
        &self.image_buffer
    }

    pub(crate) fn get_stats(&mut self) {
        let print_dimension = |dimension: u32| style(format!("{}{}", dimension, "px")).magenta();

        println!(
            "{}{}{}.",
            ZAP,
            style("Processing done in ").bold().dim(),
            style(HumanDuration(self.start_time.elapsed())).magenta()
        );
        println!(
            "{}Height: {} -> {}.",
            HEIGHT,
            print_dimension(self.original_height),
            print_dimension(self.image_buffer.height())
        );
        println!(
            "{}Width: {} -> {}.",
            WIDTH,
            print_dimension(self.original_width),
            print_dimension(self.image_buffer.width())
        );
        println!(
            "{}Resized image successfully moved to the clipboard.",
            CLIPBOARD
        );
        // New line for readability.
        println!();
    }
}
