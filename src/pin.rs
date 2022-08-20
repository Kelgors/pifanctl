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
