use clap::Arg;
use clap::Command;
use onepack_bundler::BundleOptions;
use onepack_bundler::RUNNER_BY_ARCH;
use onepack_bundler::RUNNER_MAGIC;
use std::fs;
use std::path::Path;
use std::process;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");

macro_rules! bail {
    () => (process::exit(1));
    ($($arg:tt)*) => ({
        eprint!("{}\n", format_args!($($arg)*));
        process::exit(1);
    })
}

fn main() {
    let args = Command::new(APP_NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about("Create self-contained single binary application")
        .arg(
            Arg::new("arch")
                .short('a')
                .long("arch")
                .value_name("arch")
                .help(&format!(
                    "Sets the architecture. Supported: {:?}",
                    RUNNER_BY_ARCH.keys()
                ))
                .display_order(1)
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("input_dir")
                .short('i')
                .long("input_dir")
                .value_name("input_dir")
                .help("Sets the input directory containing the application and dependencies")
                .display_order(2)
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("exec")
                .short('e')
                .long("exec")
                .value_name("exec")
                .help("Sets the application executable file name")
                .display_order(3)
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("output")
                .help("Sets the resulting self-contained application file name")
                .display_order(4)
                .num_args(1)
                .required(true),
        )
        .get_matches();

    let arch = args.get_one::<String>("arch").unwrap();
    let binding = arch.clone();
    let arch = binding.as_str();
    if !RUNNER_BY_ARCH.contains_key(arch) {
        bail!(
            "Unknown architecture specified: {}, supported: {:?}",
            arch,
            RUNNER_BY_ARCH.keys()
        );
    }
    let input_dir = args.get_one::<String>("input_dir").unwrap();
    let input_dir = Path::new(input_dir);
    if fs::metadata(input_dir).is_err() {
        bail!("Cannot access specified input directory {:?}", input_dir);
    }

    let exec_name = args.get_one::<String>("exec").unwrap();
    let binding = exec_name.clone();
    let exec_name = binding.as_str();
    if exec_name.len() >= RUNNER_MAGIC.len() {
        bail!("Executable name is too long, please consider using a shorter name");
    }

    let exec_path = Path::new(input_dir).join(exec_name);
    match fs::metadata(&exec_path) {
        Err(_) => {
            bail!("Cannot find file {:?}", exec_path);
        }
        Ok(metadata) => {
            if !metadata.is_file() {
                bail!("{:?} isn't a file", exec_path);
            }
        }
    }

    let bin_name = Path::new(args.get_one::<String>("output").unwrap());

    let opts = BundleOptions {
        arch: arch.to_string(),
        input_dir: input_dir.to_str().unwrap().to_string(),
        exec_name: exec_name.to_string(),
        output: bin_name.to_str().unwrap().to_string(),
    };

    onepack_bundler::bundle(opts).unwrap();
}
