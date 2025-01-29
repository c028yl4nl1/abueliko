type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;
use core::hash;
use native_dialog::FileDialog;
use native_dialog::MessageDialog;
use native_dialog::MessageType;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::mem::replace;
use ABUELIKO::clear_screen;

use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use ABUELIKO::ascii_art;
const FILENAME_SALVE: &str = "user_pass.txt";
const FILENAME_SALVE_MAIL_PASS: &str = "mail_pass.txt";
use std::fs::{ File};
use std::io::{self, BufReader, Read};
use std::path::Path;


fn main() {
    ascii_art();
    println!("\n\nSelecciona el archivo:\n\n\n");
    let file_path = openfile();

    // Configurar tamanho do chunk (100 MB)
    const CHUNK_SIZE: usize = 50 * 1024 * 1024;

    // Abrir o arquivo para leitura
    let file = File::open(&file_path).expect("Não foi possível abrir o arquivo");
    let mut reader = BufReader::new(file);

    let mut buffer = vec![0; CHUNK_SIZE];

    println!("Escribe la clave, la URL o cualquier otro tipo, y realizaré la búsqueda por similitud\n\n");
    let key = input_keySearch();

    println!("Buscando logins ...");
    let mut total_logins_encontrados = 0;

    let mut folder = filesave();

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        println!("Searching: {} Found", total_logins_encontrados );
        // Parar quando atingir o final do arquivo
        if bytes_read == 0 {
            break;
        }

        // Converte os bytes lidos para string
        let chunk_content = String::from_utf8_lossy(&buffer[..bytes_read]);

        let mut string = String::new();
        for line in chunk_content.lines() {
            if line.contains(&key) {
                string.push_str(&format!("{}\n", line));
            }
        }

        let format_user_pass = format_url_user_pass(string.clone());
        total_logins_encontrados += format_user_pass.len();

        for login in format_user_pass {
            let folder = folder.join(format!("{}/", login.tldsuffix));
            fs::create_dir_all(&folder);
            let buffer = login.buffer;
            if contains_email(&buffer) {
                let mail = folder.join("mail_pass.txt");
                salve_file(mail, buffer);
            } else {
                let mail = folder.join("user_pass.txt");
                salve_file(mail, buffer);
            }
        }
    }

    println!("Total de logins encontrados: {}", total_logins_encontrados);
}


#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct login {
    tldsuffix: String,
    buffer: String,
}

fn format_url_user_pass(buffer: String) -> HashSet<login> {
    let mut hash = HashSet::new();

    for line in buffer.lines() {
        let v = capture_tld(line);
        let mut tld = String::new();
        if let Some(capture) = v {
            tld = capture;
        } else {
            continue;
        }
        if line.contains("[NOT_SAVED]") {
            continue;
        }
        let line = line.replace(" ", "").to_owned();

        let line: Vec<&str> = line.split(":").collect();
        let len = line.len();

        if len > 2 {
            let last_two = &line[line.len() - 2..];

            let user = last_two[0];
            let pass = last_two[1];
            let user_pass = format!("{}:{}\n", user, pass);
            if user_pass.contains("/") {
                continue;
            }

            hash.insert(login {
                tldsuffix: tld,
                buffer: user_pass,
            });
        } else {
            continue;
        }
    }

    hash
}

fn openfile() -> PathBuf {
    use native_dialog::FileDialog;

    if let Ok(Some(filename)) = FileDialog::new()
        .add_filter("Select File login", &["txt"])
        .show_open_single_file()
    {
        return filename;
    } else {
        let _ = MessageDialog::new()
            .set_title("Error")
            .set_type(MessageType::Error)
            .set_text("Error opening the file")
            .show_alert();
    }
    eprintln!("I need to file");
    exit(1);
}

fn filesave() -> PathBuf {
    if let Ok(Some(folder)) = FileDialog::new()
        .set_title("Select Folder to Save Login")
        .show_open_single_dir()
    {
        return folder;
    } else {
        let _ = MessageDialog::new()
            .set_title("Error")
            .set_type(MessageType::Error)
            .set_text("Error selecting a folder")
            .show_alert();
    }

    eprintln!("A folder is required.");
    exit(1);
}

use lolcrab::*;

fn read_input() -> String {
    let mut input = String::new();

    let mut lol = Lolcrab::new(None, None);
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let text = r#"
    [1] URLP - Format User:Pass
    
    "#;
    lol.colorize_str(&text, &mut stdout);
    stdout.flush();

    println!();
    print!("Op: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn input_keySearch() -> String {
    let mut input = String::new();
    print!("Search Key : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
use regex::Regex;

fn contains_email(input: &str) -> bool {
    let re = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
    re.is_match(input)
}

fn capture_tld(string: &str) -> Option<String> {
    let domain_regex = Regex::new(r"(?:(?:https?://)?(?:www\.)?)?([^/:]+)").unwrap();

    // Aplicar a regex na string
    if let Some(captures) = domain_regex.captures(string) {
        if let Some(domain) = captures.get(1) {
            let extract = domain.as_str();

            let tld = extract_tld(extract)?;

            let suffix = extract_tld_suffix(&tld);

            return suffix;
        }
    } else {
        return None;
    }
    None
}

use tldextract::*;
pub fn extract_tld(host: &str) -> Option<String> {
    let option = TldOption::default();
    let v = TldExtractor::new(option).extract(&host.replace(" ", ""));
    let es = "es".to_string();
    match v {
        Ok(url) => {
            let format_website = format!(
                "{}.{}",
                url.domain.unwrap_or(es.clone()),
                url.suffix.unwrap_or(es.clone())
            );
            Some(format_website)
        }

        _ => None,
    }
}

pub fn extract_tld_suffix(host: &str) -> Option<String> {
    let option = TldOption::default();
    let v = TldExtractor::new(option).extract(&host.replace(" ", ""));
    match v {
        Ok(url) => {
            let format_website = url.suffix.unwrap_or("com".to_string());
            Some(format_website)
        }

        _ => None,
    }
}

fn salve_file(path: PathBuf, buffer: String) {
    let mut mail_file = fs::OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap();

    mail_file.write(format!("{}", buffer).as_bytes());
}
