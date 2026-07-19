//! Authorized HTTP stress / load tester.
//! Only use against systems you own or have explicit written permission to test.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::{bail, Context, Result};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use tokio::sync::Semaphore;

#[derive(Parser, Debug)]
#[command(
    name = "stress",
    about = "Authorized HTTP load / stress tester (own systems only)",
    long_about = "Send concurrent HTTP requests to measure how YOUR service behaves under load.\n\
Never use against third-party targets without explicit authorization."
)]
struct Args {
    /// Target URL (must be a host you are authorized to test)
    #[arg(short, long)]
    url: String,

    /// Concurrent workers
    #[arg(short, long, default_value_t = 32)]
    concurrency: usize,

    /// Total number of requests
    #[arg(short, long, default_value_t = 1000)]
    requests: u64,

    /// HTTP method
    #[arg(short, long, default_value = "GET")]
    method: String,

    /// Confirm you are authorized to test this target
    #[arg(long)]
    i_am_authorized: bool,

    /// Request timeout in seconds
    #[arg(long, default_value_t = 10)]
    timeout: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if !args.i_am_authorized {
        bail!(
            "Refusing to run without --i-am-authorized.\n\
             Only stress-test systems you own or have written permission to test."
        );
    }

    if !(args.url.starts_with("http://") || args.url.starts_with("https://")) {
        bail!("URL must start with http:// or https://");
    }

    println!();
    println!("  stress-attack · authorized load tester");
    println!("  target      : {}", args.url);
    println!("  method      : {}", args.method.to_uppercase());
    println!("  concurrency : {}", args.concurrency);
    println!("  requests    : {}", args.requests);
    println!();

    let client = Client::builder()
        .timeout(Duration::from_secs(args.timeout))
        .pool_max_idle_per_host(args.concurrency)
        .build()
        .context("failed to build HTTP client")?;

    let ok = Arc::new(AtomicU64::new(0));
    let fail = Arc::new(AtomicU64::new(0));
    let bytes = Arc::new(AtomicU64::new(0));
    let sem = Arc::new(Semaphore::new(args.concurrency));

    let pb = ProgressBar::new(args.requests);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.red} [{bar:40.red/black}] {pos}/{len} · {per_sec} · ETA {eta}",
        )?
        .progress_chars("█▉▊▋▌▍ "),
    );

    let started = Instant::now();
    let mut handles = Vec::with_capacity(args.requests as usize);

    for _ in 0..args.requests {
        let permit = sem.clone().acquire_owned().await?;
        let client = client.clone();
        let url = args.url.clone();
        let method = args.method.clone();
        let ok = ok.clone();
        let fail = fail.clone();
        let bytes = bytes.clone();
        let pb = pb.clone();

        handles.push(tokio::spawn(async move {
            let _permit = permit;
            let req = match method.to_uppercase().as_str() {
                "POST" => client.post(&url),
                "PUT" => client.put(&url),
                "HEAD" => client.head(&url),
                _ => client.get(&url),
            };

            match req.send().await {
                Ok(resp) => {
                    let status = resp.status();
                    match resp.bytes().await {
                        Ok(body) => {
                            bytes.fetch_add(body.len() as u64, Ordering::Relaxed);
                            if status.is_success() {
                                ok.fetch_add(1, Ordering::Relaxed);
                            } else {
                                fail.fetch_add(1, Ordering::Relaxed);
                            }
                        }
                        Err(_) => {
                            fail.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                }
                Err(_) => {
                    fail.fetch_add(1, Ordering::Relaxed);
                }
            }
            pb.inc(1);
        }));
    }

    for h in handles {
        let _ = h.await;
    }
    pb.finish_and_clear();

    let elapsed = started.elapsed().as_secs_f64().max(0.001);
    let ok_n = ok.load(Ordering::Relaxed);
    let fail_n = fail.load(Ordering::Relaxed);
    let byte_n = bytes.load(Ordering::Relaxed);

    println!("── results ─────────────────────────────");
    println!("  ok          : {ok_n}");
    println!("  failed      : {fail_n}");
    println!("  bytes       : {byte_n}");
    println!("  duration    : {elapsed:.2}s");
    println!("  throughput  : {:.1} req/s", args.requests as f64 / elapsed);
    println!("────────────────────────────────────────");
    println!();

    Ok(())
}
