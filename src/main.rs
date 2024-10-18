use ztrs::FromEnv;

#[tokio::main]
async fn main() {
    let central_client = zerotier_central::Client::from_env();
    let one_client = zerotier_one::Client::from_env();
    ztrs::cli(&central_client, &one_client).await;
}
