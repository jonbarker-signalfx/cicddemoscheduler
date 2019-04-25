extern crate cronjob;
use cronjob::CronJob;
use reqwest;
use std::collections::HashMap;

// Our cronjob handler.

use std::time::Duration;

use clokwerk::{Scheduler, TimeUnits};
// Import week days and WeekDay
use clokwerk::Interval::*;
use std::thread;


fn main() {


// Create a new scheduler
let mut scheduler = Scheduler::new();
// Add some tasks to it
scheduler.every(1.minutes()).plus(30.seconds()).run(sendcurl);

}

fn sendcurl() {
    let mut map = HashMap::new();
    map.insert("user", "jbarker@signalfx.com");
    map.insert("body", "json");
    let client = reqwest::Client::new();
    let _res = client.post("https://us-central1-se-cicd-demo.cloudfunctions.net/deploy-canary")
    .json(&map)
    .send();
    println!("Curl request sent!");
}


// scheduler.every(1.day()).at("3:20 pm").run(|| println!("Daily task"));
// scheduler.every(Wednesday).at("14:20:17").run(|| println!("Weekly task"));
// scheduler.every(Weekday).run(|| println!("Every weekday at midnight"));

// Manually run the scheduler in an event loop
// for _ in 1..10 {
//     scheduler.run_pending();
//     thread::sleep(Duration::from_secs(10));
// }

// Or run it in a background thread
// let thread_handle = scheduler.watch_thread(Duration::from_millis(1000));
// // The scheduler stops when `thread_handle` is dropped, or `stop` is called
// thread_handle.stop();
// }






