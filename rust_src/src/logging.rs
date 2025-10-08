use simplelog::*;
use std::{fs::{self, File}, path::Path};

use crate::prelude::{PoiseResult};


pub fn setup_logger(console: bool) -> PoiseResult<()> {
    let now = chrono::Utc::now().to_rfc3339().replace(':', "-");
    
    if !Path::new("logs").exists() {
        fs::create_dir("logs")?;
    }
    let path = format!("logs/{}.log", now);

    drop(now);

    let log_file: File = File::create(path).unwrap();

    if console {
        CombinedLogger::init(vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Always),
            WriteLogger::new(LevelFilter::Info, Config::default(), log_file),
        ])?;
    } else {
        CombinedLogger::init(vec![
            WriteLogger::new(LevelFilter::Debug, Config::default(), log_file)
            ]
        )?;
    }

    Ok(())
}

// Non-async-safe logging helper
pub fn log_sync(level: Level, msg: String) {
    match level {
        log::Level::Error => log::error!("{}", msg),
        log::Level::Warn => log::warn!("{}", msg),
        log::Level::Info => log::info!("{}", msg),
        log::Level::Debug => log::debug!("{}", msg),
        log::Level::Trace => log::trace!("{}", msg)
    }
}

// Async-safe logging helper
pub async fn log_async(level: Level, msg: String) {
    // Spawn a separate task so it doesn't block async code
    tokio::task::spawn_blocking(move || {
        match level {
            log::Level::Error => log::error!("{}", msg),
            log::Level::Warn => log::warn!("{}", msg),
            log::Level::Info => log::info!("{}", msg),
            log::Level::Debug => log::debug!("{}", msg),
            log::Level::Trace => log::trace!("{}", msg)
        }
    }).await.ok();
}