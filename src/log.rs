use tracing::Level;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry;

pub fn setup(level: Level) {
    let subscriber = registry().with(
        tracing_subscriber::fmt::layer()
            .json()
            .with_filter(LevelFilter::from_level(level)),
    );
    tracing::subscriber::set_global_default(subscriber).expect("Could not setup tracing/logging");
}
