#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use dotenv::dotenv;
use pg_todos::{
    api::{Api, ApiCtx},
    config::Config,
};
use std::{sync::Arc, thread};
use tokio::runtime::Builder;

fn main() {
    // Init from env
    dotenv().ok();
    tracing_subscriber::fmt::init();

    // Load config
    let config = Arc::new(Config::default());
    tracing::debug!("Loaded config = {:?}", config);

    // Create a runtime on the main thread
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    // Spin up a series of runtimes in background threads
    for _ in 1..num_cpus::get() {
        let config = Arc::clone(&config);
        thread::spawn(move || {
            Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(serve(config));
        });
    }

    // Run a server on the main thread
    tracing::info!("Server listening on {}", config.listen_addr);
    rt.block_on(serve(config));
}

async fn serve(config: Arc<Config>) {
    let ctx = ApiCtx::new(Arc::clone(&config)).await;
    let api = Api::new(Arc::new(ctx));
    let listener = config.tcp_listener();
    axum::serve(listener, api.routes()).await.unwrap();
}
