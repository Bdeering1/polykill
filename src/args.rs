use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct PolykillArgs {
    /// The directory to search for projects
    pub dir: String
}