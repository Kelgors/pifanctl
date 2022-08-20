use crate::pin;

pub struct DisableOptions {
    pub fan_pin_num: u64,
}

pub fn run(options: DisableOptions) {
    pin::set_pin_state(options.fan_pin_num, 0);
}
