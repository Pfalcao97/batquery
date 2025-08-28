use std::error::Error;
use std::fs::OpenOptions;
use csv::{Writer, WriterBuilder};
use std::path::Path;

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

    let fpath = Path::new(file_path);

    let mut wtr = match fpath.exists() {
        true => WriterBuilder::new()
                                .has_headers(false)
                                .from_writer(OpenOptions::new()
                                                            .write(true)
                                                            .create(true)
                                                            .append(true)
                                                            .open(fpath)?),
        false => Writer::from_path(fpath)?
    };

    wtr.serialize(row)?;

    wtr.flush()?;
    Ok(())
}