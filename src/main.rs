mod clock;
mod ntp;

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use clap::{Arg, Command};

use crate::{clock::Clock, ntp::check_time};

fn main() {
  let command = Command::new("clock")
    .version("0.1.2")
    .about("Gets and (aspirationally) sets the time.")
    .after_help(
      "Note: UNIX timestamps are parsed as whole \
               seconds since 1st January 1970 0:00:00 UTC. \
               For more accuracy, use another format.",
    )
    .arg(
      Arg::new("action")
        .short('a')
        .value_parser(["get", "set", "check-ntp"])
        .default_value("get"),
    )
    .arg(
      Arg::new("std")
        .short('s')
        .long("use-standard")
        .value_parser(["rfc2822", "rfc3339", "timestamp"])
        .default_value("rfc3339"),
    )
    .arg(Arg::new("datetime").help("When <action> is 'set', apply <datetime>. Otherwise, ignore."));

  let args = command.get_matches();

  let action = args.get_one::<String>("action").unwrap();
  let std = args.get_one::<String>("std").unwrap();
  let before_set_now = Clock::get();
  if action == "set" {
    let t_ = args.get_one::<String>("datetime").unwrap();

    let parser = match &std[..] {
      "rfc2822" => DateTime::parse_from_rfc2822,
      "rfc3339" => DateTime::parse_from_rfc3339,
      _ => unreachable!(),
    };

    let err_msg = format!("Unable to parse {} according to {}", t_, std);

    let t = parser(t_).expect(&err_msg);
    println!("before set {:?}", t);
    Clock::set(t);

    let maybe_error = std::io::Error::last_os_error();
    let os_error_code = &maybe_error.raw_os_error();

    match os_error_code {
      // Some(0) => (),
      Some(code) => eprintln!(
        "Error Code :{:?} \n Unable to set the time: {:?}",
        code, maybe_error
      ),
      None => (),
    }
  } else if action == "check-ntp" {
    let offset = check_time().unwrap() as isize;

    let adjust_ms = ChronoDuration::milliseconds(offset as i64);

    let now: DateTime<Utc> = Utc::now() + adjust_ms;

    Clock::set(now);
  }

  let after_set_now = Clock::get();
  println!("before set : {}", before_set_now);
  println!("after set : {}", after_set_now);
  match &std[..] {
    "timestamp" => println!("{}", after_set_now.timestamp()),
    "rfc2822" => println!("{}", after_set_now.to_rfc2822()),
    "rfc3339" => println!("{}", after_set_now.to_rfc3339()),
    _ => unreachable!(),
  };
}
