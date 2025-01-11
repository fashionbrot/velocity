use env_logger;
use std::io::Write;

pub fn print_debug_log(){
    std::env::set_var("RUST_LOG", "info");
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} - {}] - {} ",
                record.target(),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}