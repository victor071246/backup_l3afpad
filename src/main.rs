use std::time::Duration;
use tokio::time;
use sysinfo::System;
use std::process::Command;
use std::fs;
use std::path::PathBuf;
use chrono::Local;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Write};

#[derive(Deserialize, Serialize)]
struct Config {
    watch_paths: Vec<String>,
}

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

fn get_backup_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    let dir = PathBuf::from(home).join(".leafguard-backups");
    fs::create_dir_all(&dir).ok();
    dir
}

fn load_config() -> Config {
    let home = std::env::var("HOME").unwrap_or_default();
    let config_path = PathBuf::from(&home).join(".config/leafguard/config.toml");

    if !config_path.exists() {
        println!("Nenhum arquivo de configuração encontrando, criando arquivo padrão em ~./config/leafguard/config.toml");
        let dir = config_path.parent().unwrap();
        fs::create_dir_all(dir).ok();
        let default = "watch_paths = [\"/home/v/notas.txt\"]\n";
        fs::write(&config_path, default).ok();
    }

    let content = fs::read_to_string(&config_path).expect("Erro ao ler config.toml");

    toml::from_str(&content).expect("Erro ao parsear config.toml")


}

fn promp_add_path(config: &mut Config, config_path: &PathBuf) {
    println!("Nenhum arquivo configurado para monitorar.");
    println!("Digite o caminho completo do arquivo ou 'sair' para cancelar:");
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