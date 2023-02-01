mod helper_methods;
mod mutex_vec;

#[tokio::main]
async fn main() {
    mutex_vec::mutex_vec().await;
}
