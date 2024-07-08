use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::env;

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
    println!("zh: 初次运行，请输入可以唯一标识您节点的字符串，例如 alice-aws-114。（合法字符：字母、数字、-、_）");
    println!("en: This is your first time running this program. Please enter a string that uniquely identifies your node, e.g. alice-aws-114. (Legal characters: letters, numbers, -, _)");
    print!("ARCHIVIST: ");
    io::stdout().flush().unwrap();

    let mut arch = String::new();
    io::stdin().read_line(&mut arch).expect("Failed to read input");
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
        let file = File::open(CONFIG_FILE).expect("Failed to open file");
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            return line.ok();
        }
    }
    None
}

fn is_safe_string(s: &str) -> bool {
    s.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}
