use std::fs::File;
use std::io::prelude::Read;

pub fn read_temp() -> u64 {
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
