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

    /// Slack emoji add API, example: "https://<yourcompany>.slack.com/api/emoji.add"
    #[structopt(short, long)]
    api_url: String,

    /// Slack token in order to access the emoji API
    #[structopt(short, long)]
    token: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let mut emoji = Emoji::new(&opt.name, opt.image);
    emoji.generate_parts()?;
    emoji.upload(&opt.api_url, &opt.token)?;
    emoji.print();
    emoji.cleanup()?;

    Ok(())
}
