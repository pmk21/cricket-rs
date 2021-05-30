# cricket-rs

![CI](https://github.com/pmk21/cricket-rs/workflows/CI/badge.svg)

A terminal based fast and optimized live cricket score viewer.

![Terminal UI for cricket-rs](examples/cricket-rs-screenshot.png)

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
USAGE:
    cricket-rs [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --tick-rate <milliseconds>    Sets match details refresh rate [default: 10000]
```

## Keyboard Bindings

| Key                         | Description           |
| --------------------------- | --------------------- |
| <kbd>↑</kbd>                | Scroll scorecard up   |
| <kbd>↓</kbd>                | Scroll scorecard down |
| <kbd>←</kbd> & <kbd>→</kbd> | Switch tabs/matches   |

## License

MIT License
