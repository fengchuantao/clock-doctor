#![deny(clippy::all)]

use napi_derive::napi;

mod clock;
mod ntp;

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use clock::Clock;
use ntp::check_time;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[napi]
pub fn check_and_set() {
  let offset = check_time().unwrap() as isize;

  let adjust_ms = ChronoDuration::milliseconds(offset as i64);

  let now: DateTime<Utc> = Utc::now() + adjust_ms;

  Clock::set(now);
}
