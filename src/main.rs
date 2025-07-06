// Minimal imports required for the main logic
mod cli;
mod file_processing;
mod progress;
mod csv_to_parquet;
mod utils;

/// Main function: Entry point of the application.
///
/// Orchestrates parsing CLI arguments, validating paths,
/// and launching parallel CSV-to-Parquet conversion.
fn main() -> anyhow::Result<()> {
    println!("Start conversion...");

    let total_start = std::time::Instant::now();
    let args = cli::Args::parse();
    file_processing::check_path(&args.input)?;
    file_processing::ensure_parent_dir_exist(&args.output)?;

    let effective_threads = match args.threads {
        Some(n) if n > 0 => {
            let max_threads = num_cpus::get();
            if n > max_threads {
                println!("⚠️ Warning: Limiting thread count to {} (max available)", max_threads);
                max_threads
            } else {n}
        }
        Some(_) => return Err(anyhow::anyhow!("Number of threads must be a positive integer")),
        None => {
            let default_threads = rayon::current_num_threads();
            default_threads
        }
    };

    println!("🚀 Using {} thread(s)", effective_threads);

    if let Some(n) = args.threads {
        let local_pool = utils::configure_thread_pool(n)?;
        local_pool.install(|| progress::process_files(&args.input, &args.output))?;
    } else {
        progress::process_files(&args.input, &args.output)?;
    }

    let duration = total_start.elapsed();
    println!(
        "✅ Conversion completed in {:?} seconds",
        duration.as_secs_f64()
    );
    Ok(())
}
