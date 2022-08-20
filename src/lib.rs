use chrono::Utc;
use std::fs::File;
use std::io::prelude::Read;
use std::{thread, time};
use sysfs_gpio::{Direction, Pin};

pub fn set_pin_state(pin_num: u64, value: u8) {
    let pin = Pin::new(pin_num);
    pin.with_exported(|| {
        pin.set_direction(Direction::Out)?;
        pin.set_value(value)?;
        Ok(())
    })
    .unwrap();
}

pub fn read_cpu_temp() -> u64 {
    let cpu_temp_path = "/sys/class/thermal/thermal_zone0/temp";
    let mut file = File::open(&cpu_temp_path).expect("Cannot open cpu_temp file");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Cannot read cpu_temp file");
    return content
        .trim()
        .parse::<u64>()
        .expect("Cannot parse cpu_temp file content");
}

pub struct WatchOptions {
    pub fan_pin_num: u64,
    pub min_time: i64,
    pub hot_cpu_temp: u64,
    pub frame_delay: u64,
    pub verbose: bool,
}

pub fn watch(options: WatchOptions) {
    let WatchOptions {
        min_time,
        fan_pin_num,
        frame_delay,
        hot_cpu_temp,
        verbose,
    } = options;
    let frame_delay = time::Duration::from_secs(frame_delay);
    let mut fan_started_at: i64 = -1;
    let mut below_at: i64 = -1;

    set_pin_state(fan_pin_num, 0);
    thread::sleep(time::Duration::from_secs(2));

    loop {
        let cpu_temp = read_cpu_temp();
        if verbose {
            println!("CPU: {}°C", cpu_temp / 1000);
        }
        if fan_started_at > -1 {
            if cpu_temp < hot_cpu_temp && below_at == -1 {
                below_at = Utc::now().timestamp();
            } else if cpu_temp >= hot_cpu_temp && below_at > -1 {
                below_at = -1;
            }
            let now = Utc::now().timestamp();
            let fanning_since = now - fan_started_at;
            if verbose {
                println!("Fan started since {}s", fanning_since);
            }
            if below_at > -1 {
                if now - below_at >= min_time {
                    println!("Stopping fan after {}s of fanning...", now - fan_started_at);
                    set_pin_state(fan_pin_num, 0);
                    fan_started_at = -1;
                    below_at = -1;
                } else if verbose {
                    println!("Below since {}s", now - below_at);
                }
            }
        } else if cpu_temp >= hot_cpu_temp {
            println!("Starting fan (cpu: {}°C)...", cpu_temp / 1000);
            set_pin_state(fan_pin_num, 1);
            fan_started_at = Utc::now().timestamp();
        }
        thread::sleep(frame_delay);
    }
}
