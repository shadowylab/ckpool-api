use std::str::FromStr;

use ckpool_api::prelude::*;

#[tokio::main]
async fn main() {
    let url = Url::parse("https://solo.ckpool.org").unwrap();
    let client = CKPoolClient::new(url);

    let addr = Address::from_str("bc1qz9vvexjmexe8pr2aueuz6x0v94ulkx2m2sp6lr")
        .unwrap()
        .assume_checked();
    let stats = client.user_stats(&addr).await.unwrap();
    println!("{stats:?}");
}
