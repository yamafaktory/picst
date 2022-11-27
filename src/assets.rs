use dialoguer::console::Emoji;

/// Banner.
pub(crate) static PICST: &str = r#"
██████╗ ██╗ ██████╗███████╗████████╗
██╔══██╗██║██╔════╝██╔════╝╚══██╔══╝
██████╔╝██║██║     ███████╗   ██║   
██╔═══╝ ██║██║     ╚════██║   ██║   
██║     ██║╚██████╗███████║   ██║   
╚═╝     ╚═╝ ╚═════╝╚══════╝   ╚═╝  
"#;

/// Spinner animation.
pub(crate) static DOTS: &[&str; 14] = &[
    "⠄", "⠆", "⠇", "⠋", "⠙", "⠸", "⠰", "⠠", "⠰", "⠸", "⠙", "⠋", "⠇", "⠆",
];

pub(crate) static BOOM: Emoji = Emoji("💥 ", "");
pub(crate) static CLIPBOARD: Emoji = Emoji("📋 ", "");
pub(crate) static HEIGHT: Emoji = Emoji("↕️ ", "");
pub(crate) static STATS: Emoji = Emoji("📊 ", "");
pub(crate) static WIDTH: Emoji = Emoji("↔️ ", "");
pub(crate) static ZAP: Emoji = Emoji("⚡", "");
