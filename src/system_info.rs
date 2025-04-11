use sysinfo::System;

fn avg(float_vec: Vec<f32>) -> f32 {
    float_vec.iter().sum::<f32>() / float_vec.len() as f32
}

#[derive(Debug)]
pub struct SystemInfo {
    pub uptime: u64,
    pub cpu_usage: f32,
    pub memory_usage: f32,
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
        })
    }
}
