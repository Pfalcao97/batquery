use chrono::Local;

fn main() {
    let mut result: String;

    let args = battery_script::parse_arguments();

    if args.verbose {
        println!("{:?}", args);
    };

    let query_moment = Local::now();

    if args.verbose {
        println!("Querying battery info.");
    };

    let battery = battery_script::BatteryInfo::build().unwrap();

    if args.verbose {
        println!("{:?}", battery);
    };

    result = if args.no_system {
        if args.verbose {
            println!("Querying system info.");
        };

        let system = battery_script::SystemInfo::build(Some(args.system_runs)).unwrap();

        if args.verbose {
            println!("{:?}", system);
        };

        battery_script::elements_to_csv_line!(
            battery.current_energy,
            battery.energy_full,
            battery.energy_full_design,
            battery.energy_rate,
            system.uptime,
            system.cpu_usage,
            system.memory_usage,
            query_moment.format("%Y-%m-%d %H:%M:%S"),
            args.benchmark_running
        )
    } else {
        battery_script::elements_to_csv_line!(
            battery.current_energy,
            battery.energy_full,
            battery.energy_full_design,
            battery.energy_rate,
            query_moment.format("%Y-%m-%d %H:%M:%S"),
            args.benchmark_running
        )
    };

    if args.verbose {
        println!("Saving data to file: {}.", args.filename);
    };

    result.push_str("\n");
    battery_script::append_to_csv(args.filename.as_str(), result);
}
