extern crate chrono;
extern crate starship_battery as battery;

use chrono::Local;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::env;

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

    let args: Vec<String> = env::args().collect();
    let benchmark_running:bool = match args[1].to_lowercase().as_str() {
        "true" => true,
        "false" => false,
        _ => panic!("Can't convert argument to bool."),
    };

    let result = format!(
        "{},{},{},{},{},{}\n", 
        battery.energy().value, 
        battery.energy_full().value, 
        battery.energy_full_design().value,
        battery.energy_rate().value,
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        benchmark_running
    );

    OpenOptions::new()
        .write(true)
        .append(true)
        .open("C:\\Users\\Pedro Falcao\\Documents\\Rust\\battery-script\\results.csv")
        .expect("Failed to open file")
        .write_all(result.as_bytes())
        .expect("Failed to append to file");

    Ok(())

}