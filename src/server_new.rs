use warp;

pub fn start() {
    use warp::Filter;

    // Match any request and return hello world!
    // If using something like `pretty_env_logger`,
    // view logs by setting `RUST_LOG=example::api`.
    let log = warp::log("openbanking::api");
    let routes = warp::any().map(|| "Hello, World!").with(log);

    warp::serve(routes).tls("src/tls/cert.pem", "src/tls/key.rsa").run(([127, 0, 0, 1], 8080));
}
