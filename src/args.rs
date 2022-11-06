use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// Height of the resized image.
    #[arg(short = 'H', long)]
    pub(crate) height: Option<u32>,

    /// Width of the resized image.
    #[arg(short, long)]
    pub(crate) width: Option<u32>,
}

#[test]
fn check_args() {
    use clap::CommandFactory;

    Args::command().debug_assert()
}
