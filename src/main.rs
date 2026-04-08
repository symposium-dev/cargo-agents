use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    // Skip "agents" if invoked as `cargo agents ...`
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args
        .iter()
        .skip_while(|a| a.as_str() == "agents")
        .map(|s| s.as_str())
        .collect();

    // Check if symposium is available
    if let Some(path) = find_symposium() {
        let status = Command::new(path)
            .args(&args)
            .status()
            .expect("failed to execute symposium");
        return ExitCode::from(status.code().unwrap_or(1) as u8);
    }

    // symposium not found — offer to install it
    let method = if has_cargo_binstall() {
        "cargo binstall symposium"
    } else {
        "cargo install symposium"
    };

    eprintln!("symposium is not installed. Install it with `{method}`?");
    eprint!("[Y/n] ");

    let mut answer = String::new();
    if std::io::stdin().read_line(&mut answer).is_err() {
        eprintln!("failed to read input");
        return ExitCode::FAILURE;
    }

    let answer = answer.trim().to_lowercase();
    if !answer.is_empty() && answer != "y" && answer != "yes" {
        eprintln!("aborted");
        return ExitCode::FAILURE;
    }

    eprintln!("running: {method}");
    let status = if has_cargo_binstall() {
        Command::new("cargo")
            .args(["binstall", "symposium", "-y"])
            .status()
    } else {
        Command::new("cargo")
            .args(["install", "symposium"])
            .status()
    };

    match status {
        Ok(s) if s.success() => {
            eprintln!("symposium installed successfully, re-running...");
            if let Some(path) = find_symposium() {
                let status = Command::new(path)
                    .args(&args)
                    .status()
                    .expect("failed to execute symposium");
                ExitCode::from(status.code().unwrap_or(1) as u8)
            } else {
                eprintln!("symposium was installed but could not be found on PATH");
                ExitCode::FAILURE
            }
        }
        Ok(_) => {
            eprintln!("installation failed");
            ExitCode::FAILURE
        }
        Err(e) => {
            eprintln!("failed to run installer: {e}");
            ExitCode::FAILURE
        }
    }
}

fn find_symposium() -> Option<std::path::PathBuf> {
    which("symposium")
}

fn has_cargo_binstall() -> bool {
    which("cargo-binstall").is_some()
}

fn which(name: &str) -> Option<std::path::PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path).find_map(|dir| {
        let full = dir.join(name);
        if full.is_file() { Some(full) } else { None }
    })
}
