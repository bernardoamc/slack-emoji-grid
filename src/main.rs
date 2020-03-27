use anyhow::Result;

mod emoji;
use emoji::Emoji;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "slack-emoji-grid",
    about = "Convert your images to a grid of emojis"
)]
struct Opt {
    /// Emoji name on Slack
    #[structopt(short, long)]
    name: String,

    /// Image file path
    #[structopt(parse(from_os_str))]
    image: PathBuf,

    /// Slack token in order to access the emoji API
    #[structopt(short)]
    token: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let mut emoji = Emoji::new(&opt.name, opt.image);
    emoji.generate_parts()?;
    emoji.upload(&opt.token)?;
    emoji.print();

    Ok(())
}
