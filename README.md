# Battery Query CLI - `batquery`

A CLI tool created to query battery and system data from both windows and linux plataforms, using Rust, Bash and Powerscript. There are two different types of scripts under this repository:

1. **Benchmark Scripts:** scripts aimed at assessing battery draw under load (currently the load is produced by [Geekbench's 6](https://www.geekbench.com/download/) CPU benchmark).
2. **Daily Usage Scripts:** scripts created to query data at different points in time, under normal usage, and estimate how the battery draw is distributed within day-to-day use.

Each is suited for different analysis: while 'Daily Usage' is more useful in gaging the system's base load, with *your* configuration and usage, like a snapshot, the benchmark is aimed at understanding power consumption at full capacity - enabling you to test the power efficiency of the system, when compared with the Geekbench Score.

These scripts were developed as part of an experiment to optimize battery usage, and understand differences between windows and linux (both, stock and customized). The result of that understanding will lead to a blog post. Feel free to use the scripts, suggest issues or modifications and share your results!

## Downloading and Configuring

To start using this script, you'll need to first download the binary - check the releases section in the github for the latest version. You might also want to build it on your own, you can do that with `cargo build` [_(check documentation)_](https://doc.rust-lang.org/cargo/commands/cargo-build.html).

Once you have the binary, you may save it in a folder wih other CLI tools and add its address to the `PATH` and the CLI should be easily accessible from the terminal, with the name of the binary. 

## Basic usage

Once you've configured you system, you can now start using the tool. As said, currently there are two basic methods of using this script: `benchmark` and `adhoc`. By simply running the tool (with the `--verbose` flag so you can see what is happening), you'll be running an `adhoc` query, with system info:

```
batquery --verbose
```

will produce:

```
BSArguments { query_type: AdHoc, filename: "./results/battery_script.csv", benchmark_running: false, no_system: true, verbose: true, system_runs: 30 }
Querying battery info.
BatteryInfo { current_energy: 63612.0, energy_rate: 5.431, energy_full: 136296.0, energy_full_design: 205200.0, battery_state: "discharging" }
Querying system info.
SystemInfo { uptime: 2682847, cpu_usage: 10.361942, memory_usage: 45.46157 }
Saving data to file: ./results/battery_script.csv.
```

Querying system info, as you'll quickly see, is the part that takes the longest, that is because we have to perform multiple runs until the information has estabilized. You can customize the number of runs or disable the system querying completely.

The full list of option is available by using the `help` command:

```
batquery --help
```

If you want to perform the benchmark query, you must pass the following parameters:

```
batquery benchmark --benchmark true
```

The first argument tells the tools that you want to perform a `benchmark` query, the `--benchmark` flag tells the tool that the benchmark is currently running. This can be automated by looking at the processes that are running in the system (see `\windows\scripts\benchmark.ps1` for an example).