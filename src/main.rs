use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::process::Command;

const CACHE_FILE: &str = "hs_cache.txt";

fn read_cache() -> io::Result<HashMap<String, String>> {
    let mut cache = HashMap::new();
    if let Ok(file) = File::open(CACHE_FILE) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(entry) = line {
                if let Some((key, value)) = entry.split_once("=>") {
                    cache.insert(key.trim().to_string(), value.trim().to_string());
                }
            }
        }
    }
    Ok(cache)
}

fn write_to_cache(target: &str, result: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(CACHE_FILE)?;
    writeln!(file, "{} => {}", target, result)
}

fn run_nmap(args: &[String]) -> io::Result<String> {
    let output = Command::new("nmap").args(args).output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}

fn extract_target(args: &[String]) -> Option<String> {
    args.iter()
        .find(|arg| arg.contains('.') || arg.parse::<std::net::Ipv4Addr>().is_ok())
        .cloned()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 || args[1] != "nmap" {
        eprintln!("Usage: hs nmap <nmap-arguments>");
        std::process::exit(1);
    }

    let nmap_args = &args[2..];
    let target = match extract_target(nmap_args) {
        Some(t) => t,
        None => {
            eprintln!("Error: Could not find a valid DNS or IP target in the nmap command.");
            std::process::exit(1);
        }
    };

    let mut cache = read_cache().unwrap_or_else(|err| {
        eprintln!("Failed to read cache: {}", err);
        std::process::exit(1);
    });

    if let Some(cached_result) = cache.get(&target) {
        println!("Cached result for {}:\n{}", target, cached_result);
    } else {
        match run_nmap(nmap_args) {
            Ok(result) => {
                println!("Nmap result for {}:\n{}", target, result);
                if let Err(err) = write_to_cache(&target, &result) {
                    eprintln!("Failed to write to cache: {}", err);
                }
            }
            Err(err) => {
                eprintln!("Failed to run nmap: {}", err);
                std::process::exit(1);
            }
        }
    }
}
