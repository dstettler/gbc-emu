use std::{collections::HashMap, fs::File, io::{Read, Seek, SeekFrom}, vec};

use crate::mappers::Mapper;

struct CpuBase {
    reg_af: u16,
    reg_bc: u16,
    reg_de: u16,
    reg_hl: u16,
    reg_sp: u16,
    reg_pc: u16,
    mapper: Box<dyn Mapper>
}

pub struct MemCpu {
    cpu_base: CpuBase,
    rom: Vec<u8>
}

pub struct FileCpu {
    cpu_base: CpuBase,
    file: File,
    rom_size: u32,
    jump_points: HashMap<u16, u16>
}

/// Reads and returns ROM size from 0x148 in ROM file
fn read_rom_size_from_file(file: &File) -> u32 {
    match (file.seek(SeekFrom::Start(0x148))) {
        Ok(_) => {
            let mut buf: u8;
            match (file.read_exact(&buf)) {
                Ok(_) => {
                    if (buf <= 8) {
                        1024 * (1 << buf)
                    } else {
                        panic!("Maximum ROM size exceeded. This emulator only supports ROM sizes up to 1024 * 256 bytes")
                    }
                },
                Err(err) => {
                    panic!("Error reading ROM, unable to read to buf: ", err)
                }
            }
        },
        Err(err) => {
            panic!("Error reading ROM, unable to seek to ROM size: ", err);
        }
    }
}

fn read_ram_size_from_file(file: &File) -> u32 {
    match (file.seek(SeekFrom::Start(0x149))) {
        Ok(_) => {
            let mut buf: u8;
            match (file.read_exact(&buf)) {
                Ok(_) => {
                    match buf {
                        0 => 0,
                        1 => 
                    }
                },
                Err(err) => {
                    panic!("Error reading ROM, unable to read to buf: ", err)
                }
            }
        },
        Err(err) => {
            panic!("Error reading ROM, unable to seek to ROM size: ", err);
        }
    }    
}

impl MemCpu {
    /// Create new CPU and reads all instructions and ROM content into memory
    /// - `file`: File of ROM to load into memory
    pub fn new(file: File) -> MemCpu {
        let rom_size = read_rom_size_from_file(&file);
        let ram_size = todo!();
        
        MemCpu {
            cpu_base: CpuBase {
                reg_af: 0,
                reg_bc: 0,
                reg_de: 0,
                reg_hl: 0,
                reg_sp: 0,
                reg_pc: 0,
                ram: vec![0, ram_size]
            },
            
            rom: vec![0, rom_size],
        }
    }

    fn read_instructions_from_file(&self, file: &File) {
        
    }
}