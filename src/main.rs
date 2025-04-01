use battery_script;
use chrono::Local;

fn main() {
    let mut result:String;

    let args = battery_script::parse_arguments();

    let query_moment = Local::now();
    let battery = battery_script::BatteryInfo::build().unwrap();

    if !args.no_system {
            let system = battery_script::SystemInfo::build(Some(30)).unwrap();
            result = battery_script::elements_to_csv_line!(
                battery.current_energy,
                battery.energy_full,
                battery.energy_full_design,
                battery.energy_rate,
                system.uptime,
                system.cpu_usage,
                system.memory_usage,
                query_moment.format("%Y-%m-%d %H:%M:%S"),
                args.benchmark_running
            );
    } else {
        result = battery_script::elements_to_csv_line!(
            battery.current_energy,
            battery.energy_full,
            battery.energy_full_design,
            battery.energy_rate,
            query_moment.format("%Y-%m-%d %H:%M:%S"),
            args.benchmark_running
        )
    };

    result.push_str("\n");

    battery_script::append_to_csv(
        args.filename.as_str(),
        result
    );
}