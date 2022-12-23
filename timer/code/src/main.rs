use anyhow::{Ok, Result};
use chrono::NaiveTime;
use clap::Parser;
// use indicatif::ProgressBar;
use std::fmt::Display;
use std::io::{self, Write};
use std::ops::Deref;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use std::alloc::System;

#[global_allocator]
static A: System = System;

#[derive(Parser, Debug)]
/// A simple timer
///
/// Example : timer -t "11:45:14" -c "ls -l" -f "%H:%M:%S"
struct Cli {
    /// The time before task is executed
    #[arg(short, long)]
    time: String,

    /// The command to execute
    #[arg(long, short)]
    command: String,

    /// Time format
    #[arg(long, short, default_value = "%H:%M:%S")]
    format: String,

    /// Display remaining time ?
    #[arg(long, short, default_value = "True")]
    display: String,
}



trait TryFromAnyhow<T>: Sized {
    fn try_from_(value: T) -> Result<Self>;
}
struct Time {
    hours: u8,
    minutes: u8,
    seconds: u8,
}

impl Time {
    fn new(hours: u8, minutes: u8, seconds: u8) -> Time {
        Time {
            hours,
            minutes,
            seconds,
        }
    }
    fn to_seconds(&self) -> u64 {
        let hours_u64 = u64::from(self.hours);
        let minutes_u64 = u64::from(self.minutes);
        let seconds_u64 = u64::from(self.seconds);
        let result_seconds: u64 = hours_u64 * 3600 + minutes_u64 * 60 + seconds_u64;
        result_seconds
    }
}
impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}h{}m{}s", self.hours, self.minutes, self.seconds)
    }
}
impl TryFromAnyhow<NaiveTime> for Time {
    fn try_from_(value: NaiveTime) -> Result<Self> {
        let value_string = value.to_string();
        let mut str_iter = value_string.split(":");
        let hour = str_iter.next().unwrap().to_string().parse::<u8>()?;
        let minutes = str_iter.next().unwrap().to_string().parse::<u8>()?;
        let seconds = str_iter.next().unwrap().to_string().parse::<u8>()?;
        let result_time = Time::new(hour, minutes, seconds);
        Ok(result_time)
    }
}
impl TryFromAnyhow<&Time> for Duration {
    fn try_from_(value: &Time) -> Result<Self> {
        let result_seconds = value.to_seconds();
        Ok(Duration::from_secs(result_seconds))
    }
}
//
fn main() -> Result<()> {
    let args = Cli::parse();
    let format_time = NaiveTime::parse_from_str(args.time.deref(), args.format.deref())?;
    let duration_time: Time = Time::try_from_(format_time)?;
    let time_format_second: u64 = duration_time.to_seconds();

    // let pb = ProgressBar::new(time_format_second);
    if args.display == "True" {
        for i in 0..time_format_second {
            let time_left = time_format_second - i;
            print!(
                "\r{}h {}m {}s left",
                time_left / 3600,
                time_left / 60,
                time_left % 60
            );
            io::stdout().flush().unwrap();
            sleep(Duration::from_secs(1))
        }
    }
    else {
        for _ in 0..time_format_second{
            sleep(Duration::from_secs(1));
        }
    }
    // pb.finish_with_message("done");

    let mut iter_of_command = args.command.split_whitespace();
    let cmd = iter_of_command.next().unwrap();

    let mut args_of_command: Vec<String> = Vec::new();
    for i in iter_of_command {
        args_of_command.push(i.to_string());
    }
    let output = Command::new(cmd).args(args_of_command).output()?;

    let ls_list = String::from_utf8(output.stdout)?;
    println!("{}", ls_list);

    Ok(())
}
