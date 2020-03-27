# slack-emoji-grid

Create a grid of emojis from an image and upload them to Slack

## Usage

`slack-emoji-grid --help`

```
Convert your images to a grid of emojis

USAGE:
    slack-emoji-grid <image> --api_url <api_url> --name <name> --token <token>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --api_url <api_url>    Slack emoji add API, example: "https://<yourcompany>.slack.com/api/emoji.add"
    -n, --name <name>          Emoji name on Slack
    -t, --token <token>        Slack token in order to access the emoji API

ARGS:
    <image>    Image file path
```
