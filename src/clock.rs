use chrono::{DateTime, Local, TimeZone};
pub(crate) struct Clock;

impl Clock {
  #[allow(dead_code)]
  pub fn get() -> DateTime<Local> {
    Local::now()
  }
  #[cfg(windows)]
  pub fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
    extern crate winapi;
    use std::mem::zeroed;

    use chrono::Datelike;
    use chrono::Timelike;
    use chrono::Weekday;
    use winapi::shared::minwindef::WORD;
    use winapi::um::minwinbase::SYSTEMTIME;
    use winapi::um::sysinfoapi::SetLocalTime;

    let t = t.with_timezone(&Local);

    println!("after with_timezone {:?}", t);

    let mut systime: SYSTEMTIME = unsafe { zeroed() };

    let dow = match t.weekday() {
      Weekday::Mon => 1,
      Weekday::Tue => 2,
      Weekday::Wed => 3,
      Weekday::Thu => 4,
      Weekday::Fri => 5,
      Weekday::Sat => 6,
      Weekday::Sun => 0,
    };

    let mut ns = t.nanosecond();
    let is_leap_second = ns > 1_000_000_000;

    if is_leap_second {
      ns -= 1_000_000_000;
    }

    systime.wYear = t.year() as WORD;
    systime.wMonth = t.month() as WORD;
    systime.wDayOfWeek = dow as WORD;
    systime.wDay = t.day() as WORD;
    systime.wHour = t.hour() as WORD;
    systime.wMinute = t.minute() as WORD;
    systime.wSecond = t.second() as WORD;
    systime.wMilliseconds = (ns / 1_000_000) as WORD;

    let systime_ptr = &systime as *const SYSTEMTIME;
    unsafe {
      SetLocalTime(systime_ptr);
    }
  }

  #[cfg(not(windows))]
  pub fn set<Tz: TimeZone>(t: DateTime<Tz>) {
    use std::mem::zeroed;

    use libc::{settimeofday, timezone};
    use libc::{suseconds_t, time_t, timeval};

    let t = t.with_timezone(&Local);
    let mut u: timeval = unsafe { zeroed() };

    u.tv_sec = t.timestamp() as time_t;
    u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

    unsafe {
      let mock_tz: *const timezone = std::ptr::null();
      settimeofday(&u as *const timeval, mock_tz)
    };
  }
}
