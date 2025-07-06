/// Configures a custom Rayon thread pool with specified size.
///
/// # Arguments
/// * `num_threads` - Desired number of threads.
///
/// # Returns
/// * `Result<ThreadPool>` - Created thread pool or error.
pub fn configure_thread_pool(num_threads: usize) -> anyhow::Result<rayon::ThreadPool> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build thread pool: {}", e))
}
