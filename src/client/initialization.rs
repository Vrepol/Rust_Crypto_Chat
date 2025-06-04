use std::io::{self, Write};
use fake::Fake;
use fake::locales::{FR_FR};
use fake::faker::name::raw::*;
use colored::*;
use std::io::IsTerminal;
use supports_color::{self,Stream as ColorStream};
pub fn init_color() {
    if std::env::var_os("NO_COLOR").is_some() {
        colored::control::set_override(false);
        return;
    }

    let ok = io::stdout().is_terminal()
        && supports_color::on(ColorStream::Stdout).is_some();

    colored::control::set_override(ok);
}
fn get_password_or_default()->String {
    let mut inp = String::new();
    let _ = io::stdin().read_line(&mut inp);
    if inp.trim().is_empty() {
        "Vrepol".to_string()
    } else {
        inp.trim().to_string()
    }
}
pub fn initial_name() -> io::Result<String> {
    // ---------- 询问昵称 ----------
    let username = loop {
        println!("{}","Continue with fake name".purple());
        print!("{}","      Or customize here: ".purple());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let name = input.trim();
        if !name.is_empty() {
            break name.to_owned();
        } else {
            let name: String = FirstName(FR_FR).fake();
            break name.to_owned()
        }
    };
    println!("{} {}","Enjoy youself, ".green(),username.clone().to_string().green());
    Ok(username)
}
pub fn initial_serveraddr() -> io::Result<String> {

    let servers = vec![
        ("Please config at /src/client/initialization::initial_serveraddr", "127.0.0.1:6655"),
    ];

    // 交互循环直到拿到合法输入
    let chosen = loop {
        println!("\nAvaliable Servers:");
        for (i, (name, _)) in servers.iter().enumerate() {
            println!("  {}. {}", i + 1, name);
        }
        print!("Choice / IP:Port / /INVITE:…  ➜ ");
        io::stdout().flush()?;

        let mut inp = String::new();
        io::stdin().read_line(&mut inp)?;
        let s = inp.trim();
        if s.is_empty() {
            println!("Default Choice : {}", servers[0].0);
            print!("Server Password: ");
            io::stdout().flush()?;
            let key = get_password_or_default();
            break format!("{}&{}",servers[0].1.to_string(),key);
        }
        // 1️⃣ 数字
        if let Ok(idx) = s.parse::<usize>() {
            if (1..=servers.len()).contains(&idx) {
                print!("Server Password: ");
                io::stdout().flush()?;
                let key = get_password_or_default();
                break format!("{}&{}",servers[idx - 1].1.to_string(),key.to_string());
            }
        }
        // 2️⃣ IP:Port  (简单正则校验 0-255.0-255.0-255.0-255:数字)
        if regex::Regex::new(r"^(?:\d{1,3}\.){3}\d{1,3}:\d+$")
            .unwrap()
            .is_match(s)
        {
            print!("Server Password: ");
            io::stdout().flush()?;
            let key = get_password_or_default();
            break format!("{}&{}",s.to_string(),key.to_string());
        }
        // 3️⃣ 邀请码
        if s.starts_with("/INVITE:") {
            break s.to_string();
        }
        println!("Enter an choice, IP or invite code!");
    };

    Ok(chosen)
}
