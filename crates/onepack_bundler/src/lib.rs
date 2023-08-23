use flate2::write::GzEncoder;
use flate2::Compression;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::copy;
use std::io::Write;
use std::path::Path;
use tempdir::TempDir;

#[derive(Debug)]
pub struct BundleOptions {
    pub arch: String,
    pub input_dir: String,
    pub exec_name: String,
    pub output: String,
}

const APP_NAME: &str = env!("CARGO_PKG_NAME");

pub const RUNNER_MAGIC: &[u8] = b"tVQhhsFFlGGD3oWV4lEPST8I8FEPP54IM0q7daes4E1y3p2U2wlJRYmWmjPYfkhZ0PlT14Ls0j8fdDkoj33f2BlRJavLj3mWGibJsGt5uLAtrCDtvxikZ8UX2mQDCrgE\0";

const RUNNER_LINUX_X64: &[u8] =
    include_bytes!("runtimes/onepack_runtime_linux");
const RUNNER_WINDOWS_X64: &[u8] =
    include_bytes!("runtimes/onepack_runtime_windows.exe");

lazy_static! {
    pub static ref RUNNER_BY_ARCH: HashMap<&'static str, &'static [u8]> = {
        let mut m = HashMap::new();
        m.insert("linux-x64", RUNNER_LINUX_X64);
        m.insert("windows-x64", RUNNER_WINDOWS_X64);
        m
    };
}

pub fn patch_runner(arch: &str, exec_name: &str) -> io::Result<Vec<u8>> {
    // Read runner executable in memory
    let runner_contents = RUNNER_BY_ARCH.get(arch).unwrap();
    let mut buf = runner_contents.to_vec();

    // Set the correct target executable name into the local magic buffer
    let magic_len = RUNNER_MAGIC.len();
    let mut new_magic = vec![0; magic_len];
    new_magic[..exec_name.len()].clone_from_slice(exec_name.as_bytes());

    // Find the magic buffer offset inside the runner executable
    let mut offs_opt = None;
    for (i, chunk) in buf.windows(magic_len).enumerate() {
        if chunk == RUNNER_MAGIC {
            offs_opt = Some(i);
            break;
        }
    }

    if offs_opt.is_none() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "no magic found inside runner",
        ));
    }

    // Replace the magic with the new one that points to the target executable
    let offs = offs_opt.unwrap();
    buf[offs..offs + magic_len].clone_from_slice(&new_magic);

    Ok(buf)
}

pub fn create_tgz(dir: &Path, out: &Path) -> io::Result<()> {
    let f = fs::File::create(out)?;
    let gz = GzEncoder::new(f, Compression::best());
    let mut tar = tar::Builder::new(gz);
    tar.follow_symlinks(false);
    tar.append_dir_all(".", dir)?;
    Ok(())
}

#[cfg(target_family = "unix")]
pub fn create_app_file(out: &Path) -> io::Result<File> {
    use std::os::unix::fs::OpenOptionsExt;

    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o755)
        .open(out)
}

#[cfg(target_family = "windows")]
pub fn create_app_file(out: &Path) -> io::Result<File> {
    fs::OpenOptions::new().create(true).write(true).open(out)
}

pub fn create_app(runner_buf: &Vec<u8>, tgz_path: &Path, out: &Path) -> io::Result<()> {
    let mut outf = create_app_file(out)?;
    let mut tgzf = fs::File::open(tgz_path)?;
    outf.write_all(runner_buf)?;
    copy(&mut tgzf, &mut outf)?;
    Ok(())
}

pub fn bundle(opts: BundleOptions) -> Result<(), String> {
    let runner_buf: Vec<u8> = patch_runner(&opts.arch, &opts.exec_name).unwrap();

    println!("Compressing input directory {:?}...", opts.input_dir);
    let tmp_dir = TempDir::new(APP_NAME).unwrap();
    let tgz_path = tmp_dir.path().join("input.tgz");
    create_tgz(Path::new(&opts.input_dir), &tgz_path).unwrap();

    let exec_name = Path::new(opts.output.as_str());
    println!(
        "Creating self-contained application binary {:?}...",
        exec_name
    );
    create_app(&runner_buf, &tgz_path, &exec_name).unwrap();

    println!("All done");
    Ok(())
}
