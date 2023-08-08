#![allow(clippy::match_single_binding)]

use std::net::SocketAddr;
use tokio::runtime::Runtime;

use tracing::info;

use crate::app::config::Config;
use crate::app::context::{AppContext, RefContext};

async fn run(context: RefContext) {
    let Config { http_server, .. } = context.config.as_ref();

    let (tx, mut rx) = tokio::sync::mpsc::channel(1);

    ctrlc::set_handler(move || {
        let _ = tx.try_send(());
    })
    .expect("Error setting Ctrl-C handler");

    let main_server = async {
        let route = crate::routes::create_router(context).with_state(context);
        let address = http_server.to_server_address();
        display_socket_address("Main server", address);
        axum::Server::bind(&address)
            .serve(route.into_make_service())
            .with_graceful_shutdown(async move {
                if rx.recv().await.is_some() {
                    info!("Shutting down server");
                }
            })
            .await
            .unwrap()
    };

    main_server.await
}

fn display_socket_address(name: &str, socket_address: SocketAddr) {
    match format_args!("{name} is listening on http://{socket_address}") {
        socket_info => {
            info!("{socket_info}");
        }
    }
}

pub fn run_server(runtime: &Runtime, config: Config) {
    let main_task = async move {
        let context = AppContext::create(config).await.into_ref();
        run(context).await
    };
    runtime.block_on(main_task);
}
