#![deny(unsafe_code, nonstandard_style)]
#![forbid(rust_2021_compatibility)]
#![warn(missing_debug_implementations, missing_docs)]
#![doc = include_str!("../README.md")]

use arboard::{Clipboard, ImageData};
use dialoguer::console::{style, Emoji};
use futures::{pin_mut, StreamExt};
use std::{borrow::Cow, io::Read};

use crate::stream::get_stream;

mod args;
mod dimension;
mod resized_image;
mod spinner;
mod stream;

static BOOM: Emoji<'_, '_> = Emoji("💥 ", "");

#[tokio::main]
async fn main() {
    // Get the stream.
    let stream = get_stream();

    // Pin it on the stack.
    pin_mut!(stream);

    // Get an instance of the clipboard to consume the stream.
    let mut clipboard = Clipboard::new().unwrap();

    while let Some(mut resized_image) = stream.next().await {
        let image_buffer = resized_image.get_buffer();
        let maybe_bytes: Result<Vec<u8>, _> = image_buffer.bytes().collect();

        if let Ok(bytes) = maybe_bytes {
            // Prepare a new image for the clipboard.
            let image = ImageData {
                bytes: Cow::from(bytes),
                height: image_buffer.height() as usize,
                width: image_buffer.width() as usize,
            };

            if clipboard.set_image(image).is_ok() {
                resized_image.get_stats();

                continue;
            }

            eprintln!(
                "{}{}",
                style("Moving the image to the clipboard failed!").red(),
                BOOM
            );
            // New line for readability.
            println!();
        }
    }
}
