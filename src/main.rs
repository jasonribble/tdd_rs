use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ok", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8065")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> &'static str {
    "ok"
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handler() {
        let response = handler().await;
        assert_eq!("ok", response);
    }
}