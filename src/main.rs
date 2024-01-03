fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let status = vecquid::app::App::run();

    if let Err(e) = status {
        panic!("{}", e);
    }
}
