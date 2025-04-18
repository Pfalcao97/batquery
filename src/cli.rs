use clap::Parser;

#[derive(Debug)]
pub enum QueryType {
    Benchmark,
    AdHoc,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    query_type: Option<String>,

    #[arg(short, long)]
    filename: Option<String>,

    #[arg(long)]
    benchmark: Option<String>,

    #[arg(long, action = clap::ArgAction::SetFalse)]
    no_system: Option<bool>,

    #[arg(short,long, action = clap::ArgAction::SetTrue)]
    verbose: Option<bool>,

    #[arg(long)]
    system_runs: Option<u8>,
}

#[derive(Debug)]
pub struct BSArguments {
    pub query_type: QueryType,
    pub filename: String,
    pub benchmark_running: bool,
    pub no_system: bool,
    pub verbose: bool,
    pub system_runs: u8,
}

pub fn parse_arguments() -> BSArguments {
    let parser = Cli::parse();

    let query_type = match parser
        .query_type
        .as_deref()
        .unwrap_or("adhoc")
        .to_lowercase()
        .as_str()
    {
        "benchmark" => QueryType::Benchmark,
        "adhoc" => QueryType::AdHoc,
        _ => panic!("Unsupported operation selected."),
    };

    let filename = match parser.filename {
        Some(path) => path,
        _ => format!(
            "./results/{}.csv",
            match query_type {
                QueryType::Benchmark => "benchmark",
                QueryType::AdHoc => "battery_script",
            }
        ),
    };

    let benchmark_running: bool = match parser.benchmark {
        Some(x) => match x.as_str() {
            "true" => true,
            "false" => false,
            _ => panic!("Can't convert argument to bool."),
        },
        _ => false,
    };

    let no_system = parser.no_system.unwrap_or(false);
    let verbose = parser.verbose.unwrap_or(false);
    let system_runs = parser.system_runs.unwrap_or(30);

    BSArguments {
        query_type,
        filename,
        benchmark_running,
        no_system,
        verbose,
        system_runs,
    }
}
