use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

const CONFIG_FILE: &str = "ARCHIVIST.conf";

pub fn get_archivist() -> String {
    if let Some(arch) = read_archivist() {
        if !is_safe_string(&arch) {
            panic!("ARCHIVIST contains illegal characters");
        }
        arch
    } else {
        let arch = new_archivist();
        if !is_safe_string(&arch) {
            panic!("ARCHIVIST contains illegal characters");
        }
        arch
    }
}

fn new_archivist() -> String {
    println!("zh: 初次运行，请输入可以唯一标识您节点的字符串，例如 neko-stwp-114。（合法字符：字母、数字、-、_）");
    println!("en: This is your first time running this program. Please enter a string that uniquely identifies your node, e.g. neko-stwp-114. (Legal characters: letters, numbers, -, _)");
    print!("ARCHIVIST: ");
    io::stdout().flush().unwrap();

    let mut arch = String::new();
    io::stdin()
        .read_line(&mut arch)
        .expect("Failed to read input");
    let arch = arch.trim().to_string();

    let mut file = File::create(CONFIG_FILE).expect("Failed to create file");
    writeln!(file, "{}", arch).expect("Failed to write to file");

    read_archivist().expect("Failed to get archivist")
}

fn read_archivist() -> Option<String> {
    if let Ok(arch) = env::var("ARCHIVIST") {
        return Some(arch);
    }

    if Path::new(CONFIG_FILE).exists() {
        let file = File::open(CONFIG_FILE);
        if let Ok(file) = file {
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                return line.ok();
            }
        }
    }
    None
}

fn is_safe_string(s: &str) -> bool {
    s.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe_string() {
        assert!(is_safe_string("neko-stwp-114"));
        assert!(is_safe_string("neko_stwp_114"));
        assert!(!is_safe_string("牢鼠"));
        assert!(!is_safe_string("laoshu stwp 514"));
        assert!(!is_safe_string("neko@stwp-114"));
    }

    #[test]
    fn test_create_config_file() {
        let arch = "neko-stwp-114";
        if Path::new(CONFIG_FILE).exists() {
            std::fs::rename(CONFIG_FILE, "ARCHIVIST.conf.bak").expect("Failed to rename file");
        }
        let mut file = File::create(CONFIG_FILE).expect("Failed to create file");
        writeln!(file, "{}", arch).expect("Failed to write to file");

        let arch = read_archivist().expect("Failed to get archivist");
        assert_eq!(arch, "neko-stwp-114");
        std::fs::remove_file(CONFIG_FILE).expect("Failed to remove file");
        std::fs::rename("ARCHIVIST.conf.bak", CONFIG_FILE).expect("Failed to rename file");
    }
}
