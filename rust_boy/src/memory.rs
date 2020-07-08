pub struct Memory
{
  rom:          Vec<u8>,
  vram:         Vec<u8>,
  external_ram: Vec<u8>,
  wram_bank0:   Vec<u8>,
  wram_bank1:   Vec<u8>,
  echo:         Vec<u8>,
  oam:          Vec<u8>,
  io:           Vec<u8>,
  hram:         Vec<u8>,
  interrupt:    u8,
}

impl Memory{
  pub fn new(cartridge: Vec<u8>) -> Memory{
    Memory{
      rom:          cartridge,
      vram:         vec![0x0; 0x2000],
      external_ram: vec![0x0; 0x2000],
      wram_bank0:   vec![0x0; 0x1000],
      wram_bank1:   vec![0x0; 0x1000],
      echo:         vec![0x0; 0x1E00],
      oam:          vec![0x0; 0x00A0],
      io:           vec![0x0; 0x0080],
      hram:         vec![0x0; 0x007F],
      interrupt:    0x0,
    }
  }

  /// Gets the correct position in memory
  pub fn get(&self, position: u16) -> u8{
    match position{
      0x0000 ..= 0x7FFF => self.rom[position as usize],
      0x8000 ..= 0x9FFF => self.vram[(position - 0x8000) as usize],
      0xA000 ..= 0xBFFF => self.external_ram[(position - 0xA000) as usize],
      0xC000 ..= 0xCFFF => self.wram_bank0[(position - 0xC000) as usize],
      0xD000 ..= 0xDFFF => self.wram_bank1[(position - 0xD000) as usize],
      0xE000 ..= 0xFDFF => self.echo[(position - 0xE000) as usize],
      0xFE00 ..= 0xFE9F => self.oam[(position - 0xFE00) as usize],      
      0xFEA0 ..= 0xFEFF => panic!("INVALID MEMORY POSITION - NOT USED") /* Does nothing */,
      0xFF00 ..= 0xFF7F => self.io[(position - 0xFF00) as usize],
      0xFF80 ..= 0xFFFE => self.hram[(position - 0xFF80) as usize],
      0xFFFF            => self.interrupt,
      _                 => panic!("INVALID MEMORY POSITION - OUT OF RANGE"),
    }
  }

  /// Sets the correct position in memory
  pub fn set(&mut self, position: u16, value: u8){
    match position{
      0x0000 ..= 0x7FFF => self.rom[position as usize] = value,
      0x8000 ..= 0x9FFF => self.vram[(position - 0x8000) as usize] = value,
      0xA000 ..= 0xBFFF => self.external_ram[(position - 0xA000) as usize] = value,
      0xC000 ..= 0xCFFF => self.wram_bank0[(position - 0xC000) as usize] = value,
      0xD000 ..= 0xDFFF => self.wram_bank1[(position - 0xD000) as usize] = value,
      0xE000 ..= 0xFDFF => self.echo[(position - 0xE000) as usize] = value,
      0xFE00 ..= 0xFE9F => self.oam[(position - 0xFE00) as usize] = value,      
      0xFEA0 ..= 0xFEFF => panic!("INVALID MEMORY POSITION - NOT USED") /* Does nothing */,
      0xFF00 ..= 0xFF7F => self.io[(position - 0xFF00) as usize] = value,
      0xFF80 ..= 0xFFFE => self.hram[(position - 0xFF80) as usize] = value,
      0xFFFF            => self.interrupt = value,
      _                 => panic!("INVALID MEMORY POSITION - OUT OF RANGE"),
    }
  }
}
