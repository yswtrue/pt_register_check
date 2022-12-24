use adapters::Scraper;
use anyhow::Result;
mod adapters;
mod utils;
use clokwerk::{AsyncScheduler, TimeUnits};
use env_logger::{Builder, Target};
use log;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut scheduler = AsyncScheduler::new();
    scheduler.every(1.hours()).run(|| async {
        log::info!("开始检测PT是否有开放注册");
        let pt_time = adapters::pttime::PTTime {};
        let _ = pt_time.check().await;
        let hd_sky = adapters::hdsky::HDSky {};
        let _ = hd_sky.check().await;
    });
    log::info!("程序启动");
    while running.load(Ordering::SeqCst) {
        scheduler.run_pending().await;
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    log::info!("程序结束");
    Ok(())
}
