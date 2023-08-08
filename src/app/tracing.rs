use time::format_description::FormatItem;
use time::UtcOffset;
use tracing_subscriber::filter::ParseError;
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

pub fn init() {
    const LOG_DATE_FORMAT: &[FormatItem<'_>] = time::macros::format_description!(
        "[[[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3][offset_hour sign:mandatory]:[offset_minute]]"
    );
    let timer = OffsetTime::new(
        UtcOffset::current_local_offset().expect("local offset"),
        LOG_DATE_FORMAT,
    );

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .with_timer(timer);

    let env_filter = build_env_filter().expect("Failed to build EnvFilter");

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(env_filter)
        .init();
}

fn build_env_filter() -> Result<EnvFilter, ParseError> {
    let env_name = EnvFilter::DEFAULT_ENV;
    if let Ok(directives) = std::env::var(env_name) {
        EnvFilter::try_new(directives)
    } else {
        EnvFilter::try_new("info")
    }
}
