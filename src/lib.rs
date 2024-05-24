use napi_derive::napi;

mod clock;
mod ntp;

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use clock::Clock;
use ntp::check_time;

#[napi(object)]
pub struct Status {
  pub before_set_time: String,
  pub after_set_time: String,
  pub err_no: i32,
  pub err_msg: String,
}

#[napi]
pub fn check_and_set() -> Status {
  let before_set_time = Clock::get();
  let offset = check_time().unwrap() as i64;

  let adjust_ms = ChronoDuration::milliseconds(offset as i64);

  let now: DateTime<Utc> = Utc::now() + adjust_ms;

  Clock::set(now);

  let maybe_error = std::io::Error::last_os_error();

  let os_error_code = &maybe_error.raw_os_error();

  let mut err_no = 0;
  let err_msg = match os_error_code {
    Some(0) => "".to_owned(),
    Some(num) => {
      err_no = *num;
      maybe_error.to_string()
    }
    None => "".to_owned(),
  };

  let after_set_time = Clock::get();
  println!("before set : {}", before_set_time);
  println!("after set : {}", after_set_time);
  Status {
    before_set_time: format!("{}", before_set_time),
    after_set_time: format!("{}", after_set_time),
    err_no,
    err_msg,
  }
}
