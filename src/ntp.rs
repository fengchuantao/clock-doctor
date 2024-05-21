use std::{net::UdpSocket, time::Duration};

use sntpc::NtpResult;

fn weighted_mean(values: &[f64], weights: &[f64]) -> f64 {
  let mut result = 0.0;
  let mut sum_of_weights = 0.0;

  for (v, w) in values.iter().zip(weights) {
    result += v * w;
    sum_of_weights += w;
  }

  result / sum_of_weights
}

fn ntp_roundtrip(server: &str, port: u16) -> Result<NtpResult, sntpc::Error> {
  let socket = UdpSocket::bind("0.0.0.0:0").expect("Unable to create UDP socket");
  socket
    .set_read_timeout(Some(Duration::from_secs(2)))
    .expect("Unable to set UDP socket read timeout");
  sntpc::simple_get_time(&format!("{}:{}", server, port), socket)
}

pub fn check_time() -> Result<f64, std::io::Error> {
  const NTP_PORT: u16 = 123;

  let servers = [
    "ntp.aliyun.com",
    "ntp1.aliyun.com",
    "ntp2.aliyun.com",
    // "ntp3.aliyun.com",
    // "ntp4.aliyun.com",
    "ntp.sjtu.edu.cn",
    "time1.cloud.tencent.com",
    "time2.cloud.tencent.com",
    // "time3.cloud.tencent.com",
    // "time4.cloud.tencent.com",
    "ntp.tuna.tsinghua.edu.cn",
  ];

  let mut times = Vec::with_capacity(servers.len());

  for &server in servers.iter() {
    print!("{} =>", server);
    let calc = ntp_roundtrip(server, NTP_PORT);
    match calc {
      Ok(time) => {
        let microseconds = sntpc::fraction_to_microseconds(time.sec_fraction());
        print!("Got time: {}.{} ;", time.sec(), microseconds);
        println!("{}ms away from local system time", time.offset() / 1000);
        times.push(time);
      }
      Err(err) => {
        println!("{:?}", err);
      }
    };
  }
  let mut offsets = Vec::with_capacity(servers.len());
  let mut offsets_weights = Vec::with_capacity(servers.len());

  for time in &times {
    let offset = (time.offset() / 1000) as f64;
    let delay = (time.roundtrip() / 1000) as f64;

    let weight = 1_000_000.0 / (delay * delay);
    // let weight: f64 = 1.0;
    println!(
      "offset origin {}  delay origin {} ",
      time.offset(),
      time.roundtrip()
    );
    println!("offset {}  delay {} weight {} ", offset, delay, weight);
    if weight.is_finite() {
      offsets.push(offset);
      offsets_weights.push(weight);
    }
  }

  println!("{:?}\n{:?}", offsets, offsets_weights);

  let avg_offser = weighted_mean(&offsets, &offsets_weights);

  Ok(avg_offser)
}
