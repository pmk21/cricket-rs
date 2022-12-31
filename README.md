# cricket-rs

![CI](https://github.com/pmk21/cricket-rs/workflows/CI/badge.svg)

A terminal based fast and optimized live cricket score viewer.

![Terminal UI for cricket-rs](assets/cricket-rs-screenshot.png)

## Supported Operating Systems

* Should support any Linux distro(if not, please open an issue).
* Windows.
* MacOS(Not tested).

## Installation

* Download the binary for your OS from [releases](https://github.com/pmk21/cricket-rs/releases).
* Install directly from the repository -
   1. **Prerequisites:** `rust` and `cargo`
   2. `git clone` this repository.
   3. `cargo install --path .` should install the binary. Make sure `$HOME/.cargo/bin` is in your `$PATH` variable.

## Usage

```output
Fast and optimized live cricket score viewer in the terminal

Usage: cricket-rs [OPTIONS]

Options:
  -t, --tick-rate <MILLISECONDS>  Sets match details refresh rate [default: 40000]
  -m, --match-id <ID>             ID of the match to follow live [default: 0]
  -h, --help                      Print help information
  -V, --version                   Print version information
```

* **For Windows Users -** I suggest using [Windows Terminal](https://github.com/Microsoft/Terminal) and Powershell.
* How to get the match ID -
  * Go to the cricbuzz page showing the match live.
  * From the URL of the page extract the match ID.
  * Example URL - `https://www.cricbuzz.com/live-cricket-scores/<match-id>/series-name...`.
  * Then run the CLI using the match ID - `cricket-rs -m <match-id>`.

## Keyboard Bindings

| Key                                            | Description           |
| ---------------------------------------------- | --------------------- |
| <kbd>↑</kbd>                                   | Scroll scorecard up   |
| <kbd>↓</kbd>                                   | Scroll scorecard down |
| <kbd>←</kbd> & <kbd>→</kbd>                    | Switch tabs/matches   |
| <kbd>Ctrl</kbd> + <kbd>C</kbd> or <kbd>q</kbd> | Quit                  |


## Contributing

Take a look at the [guide](CONTRIBUTING.md).

## License

MIT License
