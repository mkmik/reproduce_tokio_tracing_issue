use observability_deps::{
    tracing::{self, dispatcher::DefaultGuard, error},
    tracing_subscriber::{self, fmt, layer::SubscriberExt, EnvFilter},
};
use tokio::runtime::Runtime;

pub fn init_simple_logs(log_verbose_count: u8) -> DefaultGuard {
    let log_layer_filter = match log_verbose_count {
        0 => EnvFilter::try_new("warn").unwrap(),
        1 => EnvFilter::try_new("info").unwrap(),
        2 => EnvFilter::try_new("debug").unwrap(),
        _ => EnvFilter::try_new("trace").unwrap(),
    };
    let subscriber = tracing_subscriber::Registry::default()
        .with(log_layer_filter)
        .with(fmt::layer());

    let tracing_guard = tracing::subscriber::set_default(subscriber);

    tracing_guard
}

async fn foo(n: &str) {
    println!("in foo({}) async: a println log", n);
    error!("in foo({}) async: an error log", n);
    println!("in foo({}) async: done", n);
}

fn main() {
    let _tracing_guard = init_simple_logs(2);

    println!("Hello, world!");
    error!("an error log");

    let tokio_runtime = Runtime::new().unwrap();
    tokio_runtime.block_on(async {
        println!("in main async: a println log");
        error!("in main async: an error log");

        foo("from main").await;

        tokio::spawn(async {
            println!("in task async: a println log");
            error!("in task async: an error log");

            foo("from spawn").await;

            println!("in task async: done");
        })
        .await
        .unwrap();

        println!("in async: done");
    });
}
