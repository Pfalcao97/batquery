extern crate chrono;
extern crate starship_battery as battery;

use chrono::Local;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;

fn main() -> battery::Result<()> {

    let manager = battery::Manager::new()?;
    let battery = match manager.batteries()?.next() {
        Some(Ok(battery)) => battery,
        Some(Err(e)) => {
            eprintln!("Unable to access battery information");
            return Err(e);
        }
        None => {
            eprintln!("Unable to find any batteries");
            return Err(io::Error::from(io::ErrorKind::NotFound).into());
        }
    };

    let result = format!(
        "{},{},{},{},{}\n", 
        battery.energy().value, 
        battery.energy_full().value, 
        battery.energy_full_design().value,
        battery.energy_rate().value,
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );

    OpenOptions::new()
        .write(true)
        .append(true)
        .open("results.csv")
        .expect("Failed to open file")
        .write_all(result.as_bytes())
        .expect("Failed to append to file");

    Ok(())

}

// get-process | Select-String -Pattern geekbench_avx2