use battery_script::SystemInfo;
use chrono::Local;

fn main() {
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

    let mut system: Option<SystemInfo> = None;

    if args.no_system {
        if args.verbose {
            println!("Querying system info.");
        };

        system = Some(battery_script::SystemInfo::build(Some(args.system_runs)).unwrap());

        if args.verbose {
            println!("{:?}", system);   
        };
    };

    if args.verbose {
        println!("Saving data to file: {}.", args.filename);
    };

    let _ = battery_script::append_row(
        args.filename.as_str(), 
        battery_script::Full {
            current_energy: battery.current_energy,
            full_energy: battery.energy_full,
            design_full_energy: battery.energy_full_design,
            energy_loss_rate: battery.energy_rate,
            system_uptime: match &system {
                Some(sys) => Some(sys.uptime),
                None => None
            },
            cpu_usage: match &system {
                Some(sys) => Some(sys.cpu_usage),
                None => None
            },
            memory_usage: match &system {
                Some(sys) => Some(sys.memory_usage),
                None => None
            },
            cpu_temperature: match &system {
                Some(sys) => Some(sys.temperature),
                None => None
            },
            query_moment: query_moment.format("%Y-%m-%d %H:%M:%S").to_string(),
            is_benchmark_running: args.benchmark_running
        }
    );
}
