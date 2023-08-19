use std::{
    ffi::CStr,
    fs, io,
    path::{Path, PathBuf},
};

pub mod executor;
pub mod extractor;

static TARGET_FILE_NAME_BUF: &'static [u8] = b"tVQhhsFFlGGD3oWV4lEPST8I8FEPP54IM0q7daes4E1y3p2U2wlJRYmWmjPYfkhZ0PlT14Ls0j8fdDkoj33f2BlRJavLj3mWGibJsGt5uLAtrCDtvxikZ8UX2mQDCrgE\0";

pub fn target_file_name() -> &'static str {
    let nul_pos = TARGET_FILE_NAME_BUF
        .iter()
        .position(|elem| *elem == b'\0')
        .expect("TARGET_FILE_NAME_BUF has no NUL terminator");

    let slice = &TARGET_FILE_NAME_BUF[..(nul_pos + 1)];
    CStr::from_bytes_with_nul(slice)
        .expect("Can't convert TARGET_FILE_NAME_BUF slice to CStr")
        .to_str()
        .expect("Can't convert TARGET_FILE_NAME_BUF CStr to str")
}

pub fn cache_path(target: &str) -> PathBuf {
    let dir = dirs::data_local_dir()
        .expect("No data local dir found")
        .join("onepack")
        .join("packages")
        .join(target);

    if !dir.exists() {
        fs::create_dir_all(&dir).unwrap();
    }

    dir
}

pub fn extract(exe_path: &Path, cache_path: &Path) -> io::Result<()> {
    fs::remove_dir_all(cache_path).ok();
    extractor::extract_to(&exe_path, &cache_path).unwrap();
    Ok(())
}
