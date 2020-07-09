use std::path::Path;

use std::fs::File;
use std::io::Read;

pub struct Cartridge{
  pub rom: Vec<u8>,
  pub title: String,
  pub licence_code: String,
  pub cgb: u8,
  pub sgb: u8,
  pub cart_type: u8,
  pub rom_size: u8,  
  pub ram_size: u8,
  pub destination_code: u8,
  pub version_number: u8,
  // pub checksum: u8,
  // pub Global checksum: u16,

}

impl Cartridge{
  pub fn load(path: &Path) -> Cartridge{
    // Reads rom file
    let mut rom_file = File::open(path).expect("Error loading file, rom does not exist in path.");
    // Create the buffer
    let mut rom_buffer: Vec<u8> = Vec::new();
    rom_file.read_to_end(&mut rom_buffer).expect("Error loading file, rom does not exist in path.");

    let mut rom_title: String = String::new();
    for d in 0x0134 .. 0x0140 {
      rom_title.push(rom_buffer[d] as char);
    }

    let cgb_flag: u8 = rom_buffer[0x0143];
    
    let sgb_flag: u8 = rom_buffer[0x0146];

    let mut licence : String = String::new();
    if rom_buffer[0x014B] != 0x33 {
      licence.push(rom_buffer[0x014B] as char);
    } else {
      for d in 0x0144 .. 0x146 {
        licence.push(rom_buffer[d] as char);
      }
    }

    let _cart_type: u8 = rom_buffer[0x0147];
    let _rom_size: u8 = rom_buffer[0x0148];
    let _ram_size: u8 = rom_buffer[0x0149];
    let _destination_code: u8 = rom_buffer[0x014A];
    let _version_number: u8 = rom_buffer[0x014C];

    Cartridge{
      rom: rom_buffer,
      licence_code: licence,
      title: rom_title,
      cgb: cgb_flag,
      sgb: sgb_flag,
      cart_type: _cart_type,
      rom_size: _rom_size,
      ram_size: _ram_size,
      destination_code: _destination_code,
      version_number: _version_number,
    }
  }

  pub fn debug(&self) {
    println!("\nHeader Information:\n");

    println!("Title: {}", self.title);
    println!("Licence Code: {}", self.licence_code);
    println!("CGB Flag: {:#04x}", self.cgb);
    println!("SGB Flag: {:#04x}", self.sgb);
    println!("Cartridge Type: {:#04x}", self.cart_type);    
    println!("Rom Size: {:#04x}", self.rom_size);
    println!("Ram Size: {:#04x}", self.ram_size);
    println!("Destination Code: {:#04x}", self.destination_code);
  }
}
