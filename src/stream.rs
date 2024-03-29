use std::io::Read;

use anyhow::Result;
use arboard::Clipboard;
use async_stream::try_stream;
use futures::Stream;
use image::{imageops, RgbaImage};
use itertools::equal;
use tokio::time::{sleep, Duration, Instant};

use crate::{
    args::Args, resized_image::ResizedImage, spinner::display_spinner, wizard::create_wizard,
};

static SLEEP_TIME_MS: u64 = 250;

/// Main loop stream polling on the clipboard content.
pub(crate) fn get_stream(args: Args) -> impl Stream<Item = Result<ResizedImage>> {
    try_stream! {
        // Get a new instance of the clipboard.
        let mut clipboard = Clipboard::new().unwrap();

        // Keep track of the previous images for comparison.
        let mut previous_image: Option<Vec<u8>>= None;

        loop {
            if let Ok(image) = clipboard.get_image() {
                // Use a bool to flag for noop.
                let mut skip_iteration = false;

                // Convert the clipboard image to an image buffer.
                let image_buffer = RgbaImage::from_raw(image.width as u32, image.height as u32, image.bytes.to_vec()).unwrap();

                // Try to get the bytes from the new image buffer.
                let maybe_bytes: Result<Vec<u8>, _> = image_buffer.bytes().collect();

                if let Ok(bytes) = maybe_bytes {
                    // Try to unwrap the previous image.
                    if let Some(ref buffer) = previous_image {
                        // If we have a mismatch, we assume that we have
                        // a new image from the clipboard.
                        // Store it.
                        if equal(&bytes, buffer) {
                            skip_iteration = true;
                        } else {
                            previous_image = Some(bytes);
                        }
                    }
                }

                if !skip_iteration {
                    // Create a wizard to handle all the necessary user prompts.
                    let (height, width) = create_wizard(&args, &image)?;

                    // Keep track of the start time of the resize operation.
                    let start_time = Instant::now();

                    // Display a spinner and get a closure to end it.
                    let on_done = display_spinner();

                    // Proceed with the image resizing operation.
                    let resized_buffer = imageops::resize(&image_buffer, width, height, imageops::FilterType::Lanczos3);

                    // Try to get the bytes from the new image buffer.
                    let maybe_resized_bytes: Result<Vec<u8>, _> = resized_buffer.bytes().collect();

                    if let Ok(bytes) = maybe_resized_bytes {
                        // Keep track of the resized image which is going
                        // to be move to the clipboard.
                        previous_image = Some(bytes);

                        // Stop the spinner.
                        on_done();

                        yield ResizedImage::new(resized_buffer, image_buffer.height(),image_buffer.width(), start_time)
                    }
                }
            };

            sleep(Duration::from_millis(SLEEP_TIME_MS)).await;
        }
    }
}
