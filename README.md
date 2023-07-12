<h1 align="center">
wttrbar
</h1>

<p align="center">
My fork of <a href="https://github.com/bjesus/wttrbar/">wttrbar</a> by bjesus, simple but detailed weather indicator for <a href="https://github.com/Alexays/Waybar/">Waybar</a> using <a href="https://wttr.in/">wttr.in</a>. This uses Nerd Fonts icons rather than Noto Fonts.
</p>
<p align="center">
<img src="https://user-images.githubusercontent.com/55081/232401699-b8345fe0-ffce-4353-b51b-615389153448.png" height="400">
</p>
<hr />

## Installation

Compile yourself using `cargo build --release` and add the `wttrbar` binary to your `$PATH`.


## Usage

- `--ampm` - display time in AM/PM format
- `--location` - pass a specific location to wttr.in
- `--main-indicator` - decide which [`current_conditions` key](https://wttr.in/?format=j1) will be shown on waybar. defaults to `temp_C`
- `--date-format` - defaults to `%Y-%m-%d`, formats the date next to the days. see [reference](https://docs.rs/chrono/latest/chrono/format/strftime/index.html)
- `--hide-conditions` - show a shorter descrpition next to each hour, like `7° Mist` instead of `7° Mist, Overcast 81%, Sunshine 17%, Frost 15%`
- `--fahrenheit` - use fahrenheit instead of celsius

e.g. `wttrbar --date-format "%m/%d" --location Paris --hide-conditions`

## Waybar configuration

Assuming `wttrbar` is in your path, it can be used like:
```json
"custom/weather": {
    "format": "{} °",
    "tooltip": true,
    "interval": 3600,
    "exec": "wttrbar",
    "return-type": "json"
},
```

## Old version

This code is based on my [old Python gist](https://gist.github.com/bjesus/f8db49e1434433f78e5200dc403d58a3) that was used for the same purpose.
