use std::{thread, time::Duration};

use rand::Rng;

pub async fn get_random_number() -> u32 {
    thread::sleep(Duration::from_millis(2000));
    rand::thread_rng().gen_range(0..100)
}
