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