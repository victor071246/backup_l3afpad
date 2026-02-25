use std::{path::Path, time::Duration};
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

fn ensure_ydotool_is_running() {
    if !ins_process_running("ydotoold") {
        println!("Iniciando ydotoold...");
        let status = Command::new("ydotoold").status().expect("Falha ao iniciar ydotoold");

        if !status.success() {
            println!("ydotoold falhou. Instale usando: sudo pacman -S ydotool");
        }
    }
}

fn load_config() -> Config {
    let home = std::env::var("HOME").unwrap_or_default();
    let config_path = PathBuf::from(&home).join(".config/leafguard/config.toml");

    //Cria diretório se não existe
    if let Some(parent) = config_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    if !config_path.exists() {
        println!("Nenhum arquivo de configuração encontrando, criando arquivo padrão em ~./config/leafguard/config.toml");
        let default_config = Config {
            watch_paths: vec!["/home/v/notas.txt".to_string()]
        };
        let toml_default = r#"watch_paths = ["/home/v/notas.txt]"#;
        
        let _ = fs::write(&config_path, toml_default);
        return default_config;
    }

    //Lê com o fallback
    match fs::read_to_string(&config_path) {
        Ok(content) => {
            match toml::from_str(&content) {
                Ok(config) => config,
                Err(_) => {
                    println!("Config corrompido, usando padrão");
                    Config { watch_paths: vec![]}
                }
            }
        }
        Err(_) => {
            println!("Erro ao ler config, usando padrão");
            Config { watch_paths: vec![]}
        }
    }



}

fn promp_add_path(config_path: &PathBuf) -> Config {
    println!("Nenhum arquivo configurado para monitorar.");
    println!("Digite o caminho completo do arquivo ou 'sair' para cancelar:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let path = input.trim();

    if path == "sair" {
        return Config { watch_paths: vec![] };
    }

    let mut config = load_config(); // Reusa a que já temos
    config.watch_paths.push(path.to_string());

    // Salva no disco
    let toml_content = toml::to_string(&config).unwrap();
    fs::write(config_path, toml_content).unwrap();

    println!("Adicionado: {}", path);
    config
}

fn backup_file(source_path: &str) {
    let backup_dir = get_backup_dir();
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let filename = PathBuf::from(source_path).file_name().unwrap_or_default().to_str().unwrap_or("backup").to_string();

    let backup_path = backup_dir.join(format!("{}_{}", filename, timestamp));

    if let Err(e) = fs::copy(source_path, &backup_path) {
        println!("Erro backup {}: {}", source_path, e);
    } else {
        println!("Backup salvo: {}", backup_path.display());
    }
}

#[tokio::main]
async fn main() {
    println!("Leafguard iniciando...");

    //Ydotool
    ensure_ydotool_is_running();


    // Config
    let home = std::env::var("HOME").expect("HOME não encontrada");
    let config_path = PathBuf::from(home).join(".config/leafguard/config.toml");
    let mut config = load_config();

    // Menu se vazio
    if config.watch_paths.is_empty() {
        config = promp_add_path (&config_path);
    }

    println!("Monitorando: {:?}", config.watch_paths);

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