use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// root directory
    #[arg(short, long, default_value = "dist")]
    pub dir: String,
    /// server address
    #[arg(short, long, default_value = "localhost:3000")]
    pub addr: String,
    /// auto open browser
    #[arg(short, long, default_value_t = true)]
    pub open: bool,
}
