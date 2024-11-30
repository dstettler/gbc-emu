use std::error::Error;

pub trait Mapper {
    /// Set mapped address value
    fn set_address(&mut self, target: u16, value: u16) -> Result<(), Box<dyn Error>>;
    fn get_address(&self, target: u16) -> Result<u16, Box<dyn Error>>;
    /// Sets currently selected bank depending on the MBC used
    fn set_bank(&mut self, target: u16, bank_type: MapperBankType) -> Result<(), Box<dyn Error>>;
}

trait MapperFields {

}

struct Bank {
    id: u8,
    read_only: bool,
    mapped_range_min: u16,
    mapped_range_max: u16,
    bytes: Vec<u8>
}

#[derive(PartialEq)]
pub enum MapperBankType {
    ROM,
    RAM
}

struct MBCCommon {
    banks: Vec<Bank>,
    selected_rom_bank: usize,
    selected_ram_bank: usize,
}

pub struct MBC1 {
    mbc: MBCCommon
}

impl MBC1 {
    pub fn new(rom_size: u16, ram_size: u16) -> MBC1 {
        let mut banks: Vec<Bank> = Vec::new();
        banks.push(Bank { id: 0, read_only: true, mapped_range_min: 0, mapped_range_max: 0x3FFF, bytes: Vec::new() });
        if (rom_size > 0)


        for i in 1..rom_size {
            banks.push(Bank { id: i, read_only: true, mapped_range_min: (), mapped_range_max: (), bytes: () });
        }

        MBC1 { mbc: MBCCommon {
            banks: banks,
            selected_rom_bank: todo!(),
            selected_ram_bank: todo!(),
        } }
    }
}

impl Mapper for MBC1 {
    fn set_address(&mut self, target: u16, value: u16) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn get_address(&self, target: u16) -> Result<u16, Box<dyn Error>> {
        todo!()
    }

    fn set_bank(&mut self, target: u16, bank_type: MapperBankType) -> Result<(), Box<dyn Error>> {
        for (idx, bank) in self.mbc.banks.iter().enumerate() {
            if bank.read_only && bank_type == MapperBankType::ROM && u16::from(bank.id) == target {
                self.mbc.selected_rom_bank = idx;
                return Ok(());
            } else if !bank.read_only && bank_type == MapperBankType::RAM && u16::from(bank.id) == target {
                self.mbc.selected_ram_bank = idx;
                return Ok(());
            }
        }
        
        Err(format!("Mapper error: Invalid bank [{}] selected", target))?
    }
}