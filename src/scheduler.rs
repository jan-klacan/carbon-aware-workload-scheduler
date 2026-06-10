use crate::model::{Job, ScheduledJob, Signal};
use anyhow::{anyhow, Result};
use chrono::Duration;

pub fn schedule_jobs(
    jobs: &[Job],
    signals: &[Signal],
    carbon_weight: f64,
) -> Result<Vec<ScheduledJob>> {
    let mut scheduled = Vec::new();

    for job in jobs {
        let best = find_best_start(job, signals, carbon_weight)?;
        scheduled.push(best);
    }

    Ok(scheduled)
}

fn find_best_start(
    job: &Job,
    signals: &[Signal],
    carbon_weight: f64,
) -> Result<ScheduledJob> {
    let mut best_option: Option<ScheduledJob> = None;
    let mut best_score = f64::INFINITY;

    for start_index in 0..signals.len() {
        let start_time = signals[start_index].timestamp;
        let end_time = start_time + Duration::hours(job.duration_hours as i64);

        if start_time < job.earliest_start || end_time > job.deadline {
            continue;
        }

        let end_index = start_index + job.duration_hours as usize;

        if end_index > signals.len() {
            continue;
        }

        let window = &signals[start_index..end_index];

        let energy_kwh = job.power_kw * job.duration_hours as f64;

        let cost_eur: f64 = window
            .iter()
            .map(|signal| job.power_kw * signal.electricity_price_eur_per_kwh)
            .sum();

        let carbon_kg: f64 = window
            .iter()
            .map(|signal| {
                let carbon_g = job.power_kw * signal.carbon_g_per_kwh;
                carbon_g / 1000.0
            })
            .sum();

        let priority_bonus = job.priority as f64 * 0.10;
        let score = cost_eur + carbon_weight * carbon_kg - priority_bonus;

        if score < best_score {
            best_score = score;
            best_option = Some(ScheduledJob {
                job_id: job.job_id,
                name: job.name.clone(),
                start_time,
                end_time,
                energy_kwh,
                cost_eur,
                carbon_kg,
            });
        }
    }

    best_option.ok_or_else(|| {
        anyhow!(
            "Could not find valid schedule for job {} ({})",
            job.job_id,
            job.name
        )
    })
}