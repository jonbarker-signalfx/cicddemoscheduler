use reqwest;
use std::collections::HashMap;
use clokwerk::{Scheduler, TimeUnits};
use std::thread;
use std::time::Duration;

fn main() {

let mut scheduler = Scheduler::new();
scheduler.every(5.minutes()).plus(30.seconds()).run(||{
    let mut map = HashMap::new();
    map.insert("user", "jbarker@signalfx.com");
    let client = reqwest::Client::new();
    client.post("https://us-central1-se-cicd-demo.cloudfunctions.net/deploy-canary")
    .json(&map)
    .send();
    println!("Curl request sent!");
}
);

loop {
    scheduler.run_pending();
    thread::sleep(Duration::from_millis(100));
}

}






