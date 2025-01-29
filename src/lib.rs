use lolcrab::Lolcrab;
use std::io;
use std::io::Write;
use std::process::Command;
use std::time::Duration;

const ANIMATION_TIME: u64 = 100;
pub fn ascii_art() {
    let mut lol = Lolcrab::new(None, None);
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    for c in 0..10 {
        clear_screen();
        lol.colorize_str(&asci(), &mut stdout);
        stdout.flush();
        sleep();
    }
    clear_screen();
    lol.colorize_str(&asci(), &mut stdout);
}
fn asci() -> &'static str {
    r#"

    _    ____  _   _ _____ _     ___ _  _____    _____                          _   
    / \  | __ )| | | | ____| |   |_ _| |/ / _ \  |  ___|__  _ __ _ __ ___   __ _| |_ 
   / _ \ |  _ \| | | |  _| | |    | || ' / | | | | |_ / _ \| '__| '_ ` _ \ / _` | __| 
  / ___ \| |_) | |_| | |___| |___ | || . \ |_| | |  _| (_) | |  | | | | | | (_| | |_ 
 /_/   \_\____/ \___/|_____|_____|___|_|\_\___/  |_|  \___/|_|  |_| |_| |_|\__,_|\__|

 
██████╗░░░░░█████╗░
╚════██╗░░░██╔══██╗
░░███╔═╝░░░██║░░██║
██╔══╝░░░░░██║░░██║
███████╗██╗╚█████╔╝
╚══════╝╚═╝░╚════╝░
                                                                                     
-> Tienda : http://abuelikopro.mysellix.io

-> Referencias: https://t.me/+Ml6i1s-LBIRiMmRk 


    "#
}

pub fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Error");
    } else {
        Command::new("clear").status().expect("Errro");
    }
}

fn sleep() {
    std::thread::sleep(Duration::from_millis(ANIMATION_TIME));
}
