use crate::model::{Job, Signal};
use anyhow::Result;

pub fn load_jobs(path: &str) -> Result<Vec<Job>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut jobs = Vec::new();

    for result in reader.deserialize() {
        let job: Job = result?;
        jobs.push(job);
    }

    Ok(jobs)
}

pub fn load_signals(path: &str) -> Result<Vec<Signal>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut signals = Vec::new();

    for result in reader.deserialize() {
        let signal: Signal = result?;
        signals.push(signal);
    }

    Ok(signals)
}