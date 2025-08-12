use ckpool_api::prelude::*;

#[tokio::main]
async fn main() {
    let url = Url::parse("https://solo.ckpool.org").unwrap();
    let client = CKPoolClient::new(url);

    // Get block tip height
    let stats = client
        .user_stats("bc1qz9vvexjmexe8pr2aueuz6x0v94ulkx2m2sp6lr")
        .await
        .unwrap();
    println!("{stats:?}");
}
