use clap::{Arg, ArgAction, Command};

// pifanctl -p 14 watch -s 900 -t 55 -d 10
// pifanctl -p 14 enable
// pifanctl -p 14 disable
fn cli() -> Command<'static> {
    Command::new("pifanctl")
        .about("Fan controller")
        .version("1.0.0")
        .author("Matthieu BOHEAS")
        .arg(
            Arg::new("fanpin")
                .short('p')
                .long("fanpin")
                .help("Set the fan pin")
                .action(ArgAction::Set)
                .default_value("14"),
        )
        .subcommand(
            Command::new("watch")
                .about("Watch temperature and start fan if needed")
                .arg(
                    Arg::new("mintime")
                        .short('s')
                        .long("seconds")
                        .help("Set the minimum time to fan after target temperature is reached")
                        .action(ArgAction::Set)
                        .default_value("900"),
                )
                .arg(
                    Arg::new("temp")
                        .short('t')
                        .long("temperature")
                        .help("Target temperature to start fanning (in °C)")
                        .action(ArgAction::Set)
                        .default_value("55"),
                )
                .arg(
                    Arg::new("delay")
                        .short('d')
                        .long("delay")
                        .help("Delay between each check")
                        .action(ArgAction::Set)
                        .default_value("10"),
                )
                .arg(
                    Arg::new("verbose")
                        .long("verbose")
                        .help("Show logs")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(Command::new("enable").about("Turn fan on"))
        .subcommand(Command::new("disable").about("Turn fan off"))
}

fn main() {
    let matches = cli().get_matches();
    let fan_pin_num = matches
        .get_one::<String>("fanpin")
        .unwrap()
        .parse::<u64>()
        .unwrap();
    match matches.subcommand() {
        Some(("watch", matches)) => {
            let min_time = matches
                .get_one::<String>("mintime")
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let cpu_temp = matches
                .get_one::<String>("temp")
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let frame_delay = matches
                .get_one::<String>("delay")
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let verbose = matches.get_one::<bool>("verbose").unwrap().to_owned();

            pifanctl::watch(pifanctl::WatchOptions {
                fan_pin_num,
                min_time,
                hot_cpu_temp: cpu_temp * 1000,
                frame_delay,
                verbose,
            });
        }
        Some(("enable", _)) => pifanctl::set_pin_state(fan_pin_num, 1),
        Some(("disable", _)) => pifanctl::set_pin_state(fan_pin_num, 0),
        _ => unreachable!(),
    }
}
