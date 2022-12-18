#![deny(unsafe_code, nonstandard_style)]
#![forbid(rust_2021_compatibility)]
#![warn(missing_debug_implementations, missing_docs)]
#![doc = include_str!("../README.md")]

use std::{borrow::Cow, io::Read};

use anyhow::Result;
use arboard::{Clipboard, ImageData};
use dialoguer::console::style;
use futures::{pin_mut, StreamExt};

use crate::{
    args::Args,
    assets::{BOOM, PICST},
    stream::get_stream,
};

mod args;
mod assets;
mod dimension;
mod resized_image;
mod spinner;
mod stream;
mod unit;
mod validation;
mod wizard;

#[tokio::main]
async fn main() -> Result<()> {
    // Display the banner.
    println!("{}", style(PICST).magenta());

    // Do the arguments parsing upfront to ensure to exit directly.
    let args = Args::custom_parse();

    // Get the stream.
    let stream = get_stream(args);

    // Pin it on the stack.
    pin_mut!(stream);

    // Get an instance of the clipboard to consume the stream.
    let mut clipboard = Clipboard::new().unwrap();

    while let Some(maybe_resized_image) = stream.next().await {
        let mut resized_image = maybe_resized_image?;
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

    Ok(())
}
