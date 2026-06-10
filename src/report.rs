use crate::model::ScheduledJob;
use anyhow::Result;

pub fn write_schedule(path: &str, schedule: &[ScheduledJob]) -> Result<()> {
    let mut writer = csv::Writer::from_path(path)?;

    for job in schedule {
        writer.serialize(job)?;
    }

    writer.flush()?;
    Ok(())
}

pub fn print_summary(schedule: &[ScheduledJob]) {
    let total_energy: f64 = schedule.iter().map(|job| job.energy_kwh).sum();
    let total_cost: f64 = schedule.iter().map(|job| job.cost_eur).sum();
    let total_carbon: f64 = schedule.iter().map(|job| job.carbon_kg).sum();

    println!();
    println!("Schedule summary");
    println!("----------------");
    println!("Jobs scheduled: {}", schedule.len());
    println!("Total energy:   {:.2} kWh", total_energy);
    println!("Total cost:     €{:.2}", total_cost);
    println!("Total carbon:   {:.2} kg CO₂", total_carbon);
}