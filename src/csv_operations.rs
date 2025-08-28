use std::error::Error;
use std::fs::OpenOptions;
use csv::Writer;

#[derive(serde::Serialize)]
pub struct Full {
    pub current_energy: f32,
    pub full_energy: f32,
    pub design_full_energy: f32,
    pub energy_loss_rate: f32,
    pub system_uptime: Option<u64>,
    pub cpu_usage: Option<f32>,
    pub memory_usage: Option<f32>,
    pub cpu_temperature: Option<f32>,
    pub query_moment: String,
    pub is_benchmark_running: bool,
}


pub fn append_row(file_path: &str, row:Full) -> Result<(), Box<dyn Error>> {

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)
        .unwrap();

    let mut wtr = Writer::from_writer(file);

    wtr.serialize(row)?;

    wtr.flush()?;
    Ok(())
}