extern crate cronjob;
use cronjob::CronJob;
use reqwest;
use std::collections::HashMap;
use std::time::Duration;
use clokwerk::{Scheduler, TimeUnits};
use clokwerk::Interval::*;
use std::thread;

fn main() {



let mut scheduler = Scheduler::new();

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






