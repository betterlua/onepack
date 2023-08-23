use std::collections::HashMap;
use std::io::{Read, Write};
use std::{fs::File, path::Path};
extern crate alloc;
use alloc::vec::Vec;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LuaLibHeader {
    pub magic: u32,
    pub lua_version: String,
    // target <os>-<arch>
    pub target: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LuaLibFile {
    pub file_name: String,
    pub byte_code: Vec<u8>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LuaLib {
    pub header: LuaLibHeader,
    pub data: Vec<LuaLibFile>,
}

impl LuaLib {
    pub fn new(lua_version: &str, target: &str) -> Self {
        Self {
            header: LuaLibHeader {
                magic: 0x182512EE,
                lua_version: lua_version.to_string(),
                target: target.to_string(),
            },
            data: Vec::new(),
        }
    }

    pub fn write(&self, file_name: &str) -> Result<(), String> {
        let mut file = match File::create(Path::new(file_name)) {
            Ok(file) => file,
            Err(e) => return Err(format!("Failed to create file becuase {e}")),
        };

        let bin = match bincode::serialize(self) {
            Ok(bin) => bin,
            Err(e) => return Err(format!("Failed to serialize lua lib becuase {e}")),
        };

        match file.write_all(&bin) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to write to file becuase {e}")),
        }
    }

    pub fn open(file_name: &str) -> Result<Self, String> {
        let mut file = match File::open(Path::new(file_name)) {
            Ok(file) => file,
            Err(e) => return Err(format!("Failed to open lua lib because: {e}")),
        };

        let mut bin = Vec::new();

        match file.read_to_end(&mut bin) {
            Ok(_) => (),
            Err(e) => return Err(format!("Failed to read lua lib because: {e}")),
        };

        let output = match bincode::deserialize::<LuaLib>(&bin) {
            Ok(lib) => lib,
            Err(e) => return Err(format!("Failed to deserialize lua lib because: {e}")),
        };

        if output.header.magic != 0x182512EE {
            return Err("Invalid magic number".to_string());
        }

        Ok(output)
    }
}

pub struct LibraryOptions {
    pub lua_version: String,
    pub target: String,
    pub output: String,
}

pub fn build(opts: LibraryOptions, files: HashMap<String, Vec<u8>>) -> Result<(), String> {
    let mut lib = LuaLib::new(&opts.lua_version, &opts.target);

    for (key, value) in files {
        let file = LuaLibFile {
            file_name: key,
            byte_code: value,
        };
        lib.data.push(file);
    }

    match lib.write(&opts.output) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
