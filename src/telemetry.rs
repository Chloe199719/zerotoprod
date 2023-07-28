use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{ BunyanFormattingLayer, JsonStorageLayer };
use tracing_subscriber::{ EnvFilter, Registry, layer::SubscriberExt };
use tracing_log::LogTracer;
use tracing::Subscriber;

pub fn get_subscriber(name: String, env_filter: String) -> impl Subscriber + Send + Sync {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_|
        EnvFilter::new(env_filter)
    );
    let formating_layer = BunyanFormattingLayer::new(name.into(), std::io::stdout);
    Registry::default().with(env_filter).with(JsonStorageLayer).with(formating_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger.");
    set_global_default(subscriber).expect("Failed to set subscriber.");
}
