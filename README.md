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

## Output

After generating and uploading all the emojis to Slack the binary will output your new fancy emoji grid so you can just copy and paste it on Slack:

```
:coffee-battery_00_00::coffee-battery_01_00::coffee-battery_02_00::coffee-battery_03_00::coffee-battery_04_00:
:coffee-battery_00_01::coffee-battery_01_01::coffee-battery_02_01::coffee-battery_03_01::coffee-battery_04_01:
:coffee-battery_00_02::coffee-battery_01_02::coffee-battery_02_02::coffee-battery_03_02::coffee-battery_04_02:
:coffee-battery_00_03::coffee-battery_01_03::coffee-battery_02_03::coffee-battery_03_03::coffee-battery_04_03:
:coffee-battery_00_04::coffee-battery_01_04::coffee-battery_02_04::coffee-battery_03_04::coffee-battery_04_04:
```

