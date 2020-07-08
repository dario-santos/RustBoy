//! This file contains the registers and their functions

// The Flag Registor (The lower 8 bits of AF)
//
// | Bit | Name | Set | Clr | Explanation |
// +-----+------+-----+-----+-------------+
// |  7  |  zf  |  Z  |     |
// |  6  |  n   |  -  |     |
// |  5  |  h   |  -  |     |
// |  4  |  cy  |  C  |     |
// | 0-3 |  -   |  -  |     |

/// The registers of the LR35902 processor
///
/// | Register | High | Low | Name/Functionality |
/// |:--------:|:----:|:---:|:-------------------|
/// |    AF    |  A   |  F  | Accumulator/ Flags |
/// |    BC    |  B   |  C  | BC                 |
/// |    DE    |  D   |  E  | DE                 |
/// |    HL    |  H   |  L  | HL                 |
/// |    SP    |  -   |  -  | Stack Pointer      |
/// |    PC    |  -   |  -  | Program Counter    |
///
pub struct Registers {
  pub a: u8,
  pub f: u8,
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub h: u8,
  pub l: u8,
  pub sp: u16,
  pub stack: Vec<u16>,
  pub pc: u16,
}

impl Registers {
  /// Returns a new object of the type Registers
  pub fn new() -> Registers {
    Registers {
      a: 0x01,
      f: 0xB0,
      b: 0x00,
      c: 0x13,
      d: 0x00,
      e: 0xD8,
      h: 0x01,
      l: 0x4D,
      sp: 0xFFFE,
      stack: vec![0xFFFE],
      pc: 0x0000,
    }
  }

  /// Prints the registers information
  pub fn debug(&self)
  {    
    println!("|Register | Value  | High | Low  |");
    println!("| \x1b[0;36mBC\x1b[0m      | {:#06x} | {:#04x} | {:#04x} |", self.get_af(), self.a, self.f);
    println!("| \x1b[0;36mBC\x1b[0m      | {:#06x} | {:#04x} | {:#04x} |", self.get_bc(), self.b, self.c);
    println!("| \x1b[0;36mDE\x1b[0m      | {:#06x} | {:#04x} | {:#04x} |", self.get_de(), self.d, self.e);
    println!("| \x1b[0;36mHL\x1b[0m      | {:#06x} | {:#04x} | {:#04x} |", self.get_hl(), self.h, self.l);
    println!("| \x1b[0;36mPC\x1b[0m      | {:#06x} | ---- | ---- |", self.pc);
    println!("| \x1b[0;36mSP\x1b[0m      | {:#06x} | ---- | ---- |", self.sp);
  }
  

  /// Returns the ```AF``` register
  pub fn get_af(&self) -> u16 {
    // AF = (A << 8) | F
    ((self.a as u16) << 8) | (self.f as u16)
  }

  /// Returns the ```BC``` register
  pub fn get_bc(&self) -> u16 {
    ((self.b as u16) << 8) | (self.c as u16)
  }

  /// Returns the ```DE``` register
  pub fn get_de(&self) -> u16 {
    ((self.d as u16) << 8) | (self.e as u16)
  }

  /// Returns the ```HL``` register
  pub fn get_hl(&self) -> u16 {
    ((self.h as u16) << 8) | (self.l as u16)
  }

  /// Sets the ```AF``` register
  pub fn set_af(&mut self, value: u16) {
    self.f = (value & 0x00FF) as u8;
    self.a = ((value & 0xFF00) >> 8) as u8;
  }

  /// Sets the ```BC``` register
  pub fn set_bc(&mut self, value: u16) {
    self.c = (value & 0x00FF) as u8;
    self.b = ((value & 0xFF00) >> 8) as u8;
  }

  /// Sets the ```DE``` register
  pub fn set_de(&mut self, value: u16) {
    self.e = (value & 0x00FF) as u8;
    self.d = ((value & 0xFF00) >> 8) as u8;
  }

  /// Sets the ```HL``` register
  pub fn set_hl(&mut self, value: u16) {
    self.l = (value & 0x00FF) as u8;
    self.h = ((value & 0xFF00) >> 8) as u8;
  }

  
  /// Sets the ```zero``` flag
  pub fn get_flag_zf(&mut self) -> u8 {
    let tmp = self.f & 0b1000_0000;
    tmp >> 7
  }
  
  /// Sets the ```half carry``` flag
  pub fn set_flag_h(&mut self, value: bool) {
    self.f |= if value {0b0010_0000} else {0b0000_0000}
  }

  /// Sets the ```zero``` flag
  pub fn set_flag_zf(&mut self, value: bool) {
    self.f |= if value {0b1000_0000} else {0b0000_0000}
  }
}


// | Bit | Name | Set | Clr | Explanation |
// +-----+------+-----+-----+-------------+
// |  7  |  zf  |  Z  |     |
// |  6  |  n   |  -  |     |
// |  5  |  h   |  -  |     |
// |  4  |  cy  |  C  |     |
// | 0-3 |  -   |  -  |     |