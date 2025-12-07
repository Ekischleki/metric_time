use std::{thread::sleep, time::Duration};

use chrono::{Local, Timelike};

fn main() {
    loop {
        let time = Local::now();

        //Convert time into percentage of day complete
        let day_perc = (time.num_seconds_from_midnight() as f64
            + time.timestamp_subsec_millis() as f64 / 1000.0)
            / (24 * 3600) as f64;

        //Extract metric hours minutes and seconds
        let met_hour = (day_perc * 10.0).floor() as u32;
        let met_min = (day_perc * 10_00.0).floor() as u32 % 100;
        let met_sec = (day_perc * 10_00_00.0).floor() as u32 % 100;

        _ = clearscreen::clear();

        println!(
            "Current metric time:\n{met_hour}:{:02}:{:02}",
            met_min, met_sec
        );

        sleep(Duration::new(0, 860_000_000));
    }
}
