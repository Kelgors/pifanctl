# pifanctl

GPIO fan controller

## Install

```bash
git clone https://github.com/Kelgors/pifanctl.git
cd pifanctl
cargo build --release
sudo cp ./target/release/pifanctl /usr/local/bin/pifanctl
pifanctl --help
```

## Usage

```bash
# Check every 10 seconds if cpu fan if over 60
# if it's over, turn fan on. 
#   During fan on, if temperature is below 60, start timer to 900 seconds
#   if timer is over, stop fan
pifanctl --fanpin 14 watch --temperature 60 --seconds 900 --delay 10
```

```
USAGE:
    pifanctl [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -h, --help               Print help information
    -p, --fanpin <fanpin>    Set the fan pin [default: 14]
    -V, --version            Print version information

SUBCOMMANDS:
    disable              Turn fan off
    enable               Turn fan on
    help                 Print this message or the help of the given subcommand(s)
    watch                Watch temperature and start fan if needed
```

### watch

```
USAGE:
    pifanctl watch [OPTIONS]

OPTIONS:
    -d, --delay <delay>         Delay between each check [default: 10]
    -h, --help                  Print help information
    -s, --seconds <mintime>     Set the minimum time to fan after target temperature is reached
                                [default: 900]
    -t, --temperature <temp>    Target temperature to start fanning (in °C) [default: 55]
        --verbose               Show logs

```

