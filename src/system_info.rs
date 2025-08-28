use std::process::Command;
use sysinfo::System;

fn avg(float_vec: Vec<f32>) -> f32 {
    float_vec.iter().sum::<f32>() / float_vec.len() as f32
}

fn get_temp() -> f32 {
    let mut temp: f32 = 0.0;
    let output = Command::new("powershell")
        .args(["Get-WmiObject MSAcpi_ThermalZoneTemperature -Namespace 'root/wmi'"])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if line.contains("CurrentTemperature") {
                let _temp_aux: i16 = line
                    .split(":")
                    .last()
                    .expect("Couldn't find temperature.")
                    .trim()
                    .parse()
                    .unwrap();

                temp = (_temp_aux as f32 / 10.0) - 273.15;
            }
        }
    } else {
        println!(
            "Couldn't run the temperature command - try running this script with Adminstrator Access"
        );
    }

    temp
}

#[derive(Debug, Clone, Copy)]
pub struct SystemInfo {
    pub uptime: u64,
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub temperature: f32,
}

impl SystemInfo {
    pub fn build(maximum_loops: Option<u8>) -> Result<SystemInfo, &'static str> {
        let maximum_loops = maximum_loops.unwrap_or(30);

        let mut cpu_vector: Vec<f32> = Vec::new();
        let mut memory_vector: Vec<f32> = Vec::new();

        let mut system = System::new();

        let mut counter: u8 = 0;
        while counter <= maximum_loops {
            system.refresh_cpu_all(); // it's important to constantly refresh 
            system.refresh_memory(); // the System object.

            cpu_vector.push(system.global_cpu_usage());
            memory_vector.push(system.used_memory() as f32 / system.total_memory() as f32);

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
