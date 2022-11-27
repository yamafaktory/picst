use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

use crate::assets::DOTS;

/// Displays a spinner and provides a closure to be called when done.
pub(crate) fn display_spinner() -> impl Fn() {
    let spinner = ProgressBar::new_spinner();

    spinner.enable_steady_tick(Duration::from_millis(50));
    spinner.set_style(
        ProgressStyle::with_template("{spinner:.magenta} {msg}")
            .unwrap()
            .tick_strings(DOTS),
    );
    spinner.set_message("Processing...");

    move || spinner.finish_and_clear()
}
