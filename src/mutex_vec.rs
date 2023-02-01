use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use rand::Rng;
use tokio::task;

pub async fn mutex_vec() {
    let numbers = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for _ in 0..10 {
        let numbers = Arc::clone(&numbers);

        let handle = task::spawn(async move {
            let num = get_random_number().await;
            numbers.lock().unwrap().push(num);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("Numbers: {:?}", *numbers.lock().unwrap());
}

pub async fn get_random_number() -> u32 {
    thread::sleep(Duration::from_millis(2000));
    rand::thread_rng().gen_range(0..100)
}
