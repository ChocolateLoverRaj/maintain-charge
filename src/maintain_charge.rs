use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
    thread::sleep,
    time::Duration,
};

use anyhow::{anyhow, Context};

/// Keeps the laptop's battery within these percentages
/// Currently only works on Chromebooks, and is made for Chromebooks without sustainer
/// Sets charge control to idle, but the battery will over time still lose energy.
/// When the energy reaches the min threshold, it will start charging and charge until it reaches max_percent.
/// If for some reason the energy is above the max percent, it will force discharge
/// Even though "min" and "max" percent are specified, the battery will go slightly above and below those values for some time
pub fn maintain_charge(
    min_percent: u8,
    max_percent: u8,
    check_frequency: Duration,
) -> anyhow::Result<()> {
    if min_percent > 100 {
        Err(anyhow!("Min percent is invalid"))?;
    }
    if max_percent > 100 {
        Err(anyhow!("Max percent is invalid"))?;
    }
    if max_percent < min_percent {
        Err(anyhow!("Max percent is less than min percent"))?;
    }
    let battery_path = PathBuf::from("/sys/class/power_supply/BAT0");
    let capacity_path = battery_path.join("capacity");
    let charge_behavior_path = battery_path.join("charge_behaviour");
    let mut charge_up_to_max_percent = false;
    loop {
        let current_battery_percent = {
            let mut string = String::default();
            File::open(&capacity_path)
                .context("Failed to read battery capacity")?
                .read_to_string(&mut string)?;
            string.trim().parse::<u8>()?
        };
        let charge_behavior = if current_battery_percent < min_percent {
            // Don't just charge up to min percent, charge all the way up to max percent
            charge_up_to_max_percent = true;
            "auto"
        } else if current_battery_percent <= max_percent {
            if charge_up_to_max_percent {
                "auto"
            } else {
                "inhibit-charge"
            }
        } else {
            // Don't start charging again until it reaches min percent
            charge_up_to_max_percent = false;
            "force-discharge"
        };
        println!(
            "Battery is at {}%. Setting charge behavior to {:?}",
            current_battery_percent, charge_behavior
        );
        OpenOptions::new()
            .write(true)
            .read(false)
            .open(&charge_behavior_path)
            .context("Failed to set charge behavior")?
            .write_all(charge_behavior.as_bytes())?;
        sleep(check_frequency);
    }
}
