extern crate starship_battery as battery;

pub struct BatteryInfo {
    pub current_energy: f32,
    pub energy_rate: f32,
    pub energy_full: f32,
    pub energy_full_design: f32,
    pub battery_state: String,
}

impl BatteryInfo {
    pub fn build() -> Result<BatteryInfo, &'static str> {

        let battery = battery::Manager::new()         // Since this script is only valid
                                                .unwrap()                     // when at least one battey is found
                                                .batteries() // and able to be queryed, i'm 
                                                .unwrap()                   // overusing `unwrap`. It's not
                                                .next()// always the best way to handle errors,
                                                .unwrap()      // but for this case, it works well.
                                                .unwrap();

    Ok(BatteryInfo {
        current_energy: battery.energy().value,
        energy_rate: battery.energy_rate().value,
        energy_full: battery.energy_full().value,
        energy_full_design: battery.energy_full_design().value,
        battery_state: battery.state().to_string()
    })

    }
}