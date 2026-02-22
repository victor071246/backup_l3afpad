use std::time::Duration;
use tokio::time;
use sysinfo::System;
use std::process::Command;

fn ins_process_running(name: &str) -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys.processes().values().any(|p| p.name().to_str().unwrap_or("").contains(name))
}

fn force_save() {
    match Command::new("ydotool")
        .args(["key", "ctrl+s"])
        .spawn()
        {
            Ok(_) => println!("Control + S efetuado"),
            Err(e) => println!("Erro ao salvar control + S: {}", e)
        }
}

#[tokio::main]
async fn main() {
    println!("Leafguard iniciando...");

    let mut interval = time::interval(Duration::from_secs(5 * 60));

    loop {
        interval.tick().await;
        println!("Executando ciclo");

        if !ins_process_running("l3afpad") {
            println!("O l3afpad não foi encontrado, pulando...");
            continue;
        }

        println!("l3afpad detectado");
        force_save();
    }
}

// ydotool # insalado via pacman