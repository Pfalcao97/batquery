use std::env::consts::OS;
use std::process::Command;
use sysinfo::{Component, Components, System};

fn avg(float_vec: Vec<f32>) -> f32 {
    float_vec.iter().sum::<f32>() / float_vec.len() as f32
}

fn stddev(last_five_avgs: Vec<f32>) -> f32 {
    let mean = avg(last_five_avgs.clone());
    let mut sum: f32 = 0.0;

    for val in last_five_avgs.iter() {
        sum += (val - mean).powi(2);
    }

    (sum / (last_five_avgs.len() - 1) as f32).powf(0.5)
}

fn get_temp() -> f32 {
    match OS {
        "linux" => {
            let components = Components::new_with_refreshed_list();
            let cpu_component: Vec<&Component> = components
                .iter()
                .filter(|x| x.label().to_lowercase().contains("tctl"))
                .collect();
            cpu_component[0].temperature().unwrap()
        }
        "windows" => {
            let output = Command::new("powershell")
                .args(["Get-WmiObject MSAcpi_ThermalZoneTemperature -Namespace 'root/wmi'"])
                .output()
                .expect("Failed to execute command");

            if output.status.success() {
                let mut _temp_aux: i16 = 0;
                for line in String::from_utf8_lossy(&output.stdout).lines() {
                    if line.contains("CurrentTemperature") {
                        _temp_aux = line
                            .split(":")
                            .last()
                            .expect("Couldn't find temperature.")
                            .trim()
                            .parse()
                            .unwrap();
                    }
                }
                (_temp_aux as f32 / 10.0) - 273.15
            } else {
                println!(
                    "Couldn't run the temperature command - try running this script with Adminstrator Access"
                );
                0.0
            }
        }
        _ => 0.0,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SystemInfo {
    pub uptime: u64,
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub temperature: f32,
}

impl SystemInfo {
    pub fn build(maximum_loops: Option<u8>, verbose: bool) -> Result<SystemInfo, &'static str> {
        let maximum_loops = maximum_loops.unwrap_or(30);

        let mut cpu_vector: Vec<f32> = Vec::new();
        let mut memory_vector: Vec<f32> = Vec::new();

        let mut system = System::new();

        let mut counter: u8 = 0;
        let mut last_five: Vec<f32> = Vec::new();
        while counter <= maximum_loops {
            system.refresh_cpu_all(); // it's important to constantly refresh 
            system.refresh_memory(); // the System object.

            if counter != 0 {
                // CPU usage always starts out at 100%, so the first value can be discarded
                cpu_vector.push(system.global_cpu_usage());
            };
            memory_vector.push(system.used_memory() as f32 / system.total_memory() as f32);

            last_five.push(avg(cpu_vector.clone()));

            if last_five.len() == 5 {
                last_five.remove(0);

                // Once we have a big enough sample, we can may start checking for stability
                if stddev(last_five.clone()) < 0.1 {
                    if verbose {
                        println!("Early break after {:?} loops!", counter);
                    }

                    break;
                }
            }

            std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
            counter += 1;
        }

        Ok(SystemInfo {
            uptime: System::uptime(),
            cpu_usage: avg(cpu_vector),
            memory_usage: avg(memory_vector) * 100_f32,
            temperature: get_temp(),
        })
    }
}
