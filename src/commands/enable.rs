use crate::pin;

pub struct EnableOptions {
    pub fan_pin_num: u64,
}

pub fn run(options: EnableOptions) {
    pin::set_pin_state(options.fan_pin_num, 1);
}
