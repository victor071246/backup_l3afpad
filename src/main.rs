use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() {
    println!("Leafguard iniciando...");

    let mut interval = time::interval(Duration::from_secs(5 * 60));

    loop {
        interval.tick().await;
        println!("Executando clico");
    }
}