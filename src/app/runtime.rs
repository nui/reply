use std::io;
use tokio::runtime::{Builder, Runtime};

pub fn build() -> io::Result<Runtime> {
    Builder::new_current_thread().enable_all().build()
}
