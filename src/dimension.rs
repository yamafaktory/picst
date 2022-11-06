use dialoguer::{theme::ColorfulTheme, Input};

/// Enumeration for the dimension.
pub(crate) enum Dimension {
    Height,
    Width,
}

impl Dimension {
    fn get_name(self) -> &'static str {
        match self {
            Dimension::Height => "Height",
            Dimension::Width => "Width",
        }
    }
}

/// Returns the provided default dimension or prompt the user for a value.
pub(crate) fn get_dimension(dimension: Dimension, maybe_value: Option<u32>) -> u32 {
    // Skip prompt if a default is provided.
    if let Some(value) = maybe_value {
        return value;
    }

    // Use a prompt to get the desired value.
    let value: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(dimension.get_name())
        .validate_with({
            move |input: &String| -> Result<(), &str> {
                // Check if parsing the input to a u32 is fine.
                if input.parse::<u32>().is_ok() {
                    return Ok(());
                }

                Err("Please enter a valid number!")
            }
        })
        .interact_text()
        .unwrap();

    // We can safely unwrap here since this has already been checked above.
    value.parse::<u32>().unwrap()
}
