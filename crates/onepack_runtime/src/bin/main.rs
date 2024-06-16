use std::{env, fs, process};

use log::trace;
use log::Level;
use onepack_runtime::{cache_path, executor, extract, target_file_name};

fn main() {
    if env::var("OL_TRACE").is_ok() {
        simple_logger::init_with_level(Level::Trace).unwrap();
    }

    let self_path = env::current_exe().unwrap();
    let self_file_name = self_path.file_name().unwrap();
    let cache_path = cache_path(&self_file_name.to_string_lossy());

    trace!("self_path={:?}", self_path);
    trace!("self_file_name={:?}", self_file_name);
    trace!("cache_path={:?}", cache_path);

    let target_file_name = target_file_name();
    let target_path = cache_path.join(target_file_name);

    trace!("target_exec={:?}", target_file_name);
    trace!("target_path={:?}", target_path);

    extract(&self_path, &cache_path).unwrap();
    std::env::set_var("OP_HOME", &cache_path);
    let exit_code = executor::execute(&target_path).unwrap();
    process::exit(exit_code);
}
