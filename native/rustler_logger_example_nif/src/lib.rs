mod logger;

use crate::logger::logger_debug;

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    logger_debug("str");
    logger_debug(String::from("string"));
    logger_debug(format!("{}", a + b));
    a + b
}

rustler::init!("Elixir.RustlerLoggerExample.Nif");
