#![allow(dead_code, unused_imports)]

mod ch02_2_waker;
mod ch09_webserver;

use ch02_2_waker as waker;
use ch09_webserver as webserver;

fn main() {
    // waker::run();
    webserver::run();
}
