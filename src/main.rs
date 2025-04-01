use battery_script;

use chrono::Local;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let benchmark_running:bool = match args[1].to_lowercase().as_str() {
        "true" => true,
        "false" => false,
        _ => panic!("Can't convert argument to bool."),
    };

    let battery = battery_script::BatteryInfo::build().unwrap();
    let system = battery_script::SystemInfo::build(Some(30)).unwrap();
    let query_moment = Local::now();

    let result = format!(
        "{},{},{},{},{},{},{},{},{}\n", 
        battery.current_energy,
        battery.energy_full,
        battery.energy_full_design,
        battery.energy_rate,
        system.uptime,
        system.cpu_usage,
        system.memory_usage,
        query_moment.format("%Y-%m-%d %H:%M:%S"),
        benchmark_running
    );

    battery_script::append_to_csv(
        "./results/battery_script.csv", 
        result
    );
}