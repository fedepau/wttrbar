<h1 align="center">
wttrbar
</h1>

<p align="center">
My fork of <a href="https://github.com/bjesus/wttrbar/">wttrbar</a> by bjesus, simple but detailed weather indicator for <a href="https://github.com/Alexays/Waybar/">Waybar</a> using <a href="https://wttr.in/">wttr.in</a>. This uses Nerd Fonts icons rather than Noto Fonts emojis.
</p>
<p align="center">
<img src="https://github.com/fedepau/wttrbar-nerd/assets/86348751/e06d68b4-4ace-48e4-b33d-e2e0db2b905d.png" height="400">
</p>
<hr />

## Installation

Compile yourself using `cargo build --release` and add the resulting `wttrbar` binary to your `$PATH`.

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
