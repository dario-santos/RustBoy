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
  a: u8,
  f: u8,
  b: u8,
  c: u8,
  d: u8,
  e: u8,
  h: u8,
  l: u8,
  sp: u16,
  pc: u16,
}

impl Registers {
  /// Returns a new object of the type Registers
  pub fn new() -> Registers {
    Registers {
      a: 0x00,
      f: 0x00,
      b: 0x00,
      c: 0x00,
      d: 0x00,
      e: 0x00,
      h: 0x00,
      l: 0x00,
      sp: 0x0000,
      pc: 0x0000,
    }
  }

  /// Prints the registers information
  pub fn debug(&self) {
    println!("Registers Debug: \n\n");

    println!("|Register | Value  | High | Low  | ");
    println!("| \x1b[0;36mAF\x1b[0m      | {:#06x} | {:#04x} | {:#04x} |", self.get_af(), self.a, self.f);
    println!("| \x1b[0;36mBC\x1b[0m      | {:#06x} | {:#04x} | {:#04x} |", self.get_af(), self.b, self.c);
    println!("| \x1b[0;36mDE\x1b[0m      | {:#06x} | {:#04x} | {:#04x} |", self.get_af(), self.d, self.e);
    println!("| \x1b[0;36mHL\x1b[0m      | {:#06x} | {:#04x} | {:#04x} |", self.get_af(), self.h, self.l);
    println!("| \x1b[0;36mPC\x1b[0m      | {:#06x} | ---- | ---- |", self.pc);
    println!("| \x1b[0;36mSP\x1b[0m      | {:#06x} | ---- | ---- |", self.sp);
  }
  

  /// Returns the ```AF``` register
  pub fn get_af(&self) -> u16 {
    // AF = (A << 8) | F
    ((self.a as u16) << 8) | (self.f as u16)
  }

  /// Returns the ```BC``` register
  pub fn get_bc(&self) -> u8 {
    unimplemented!();
  }
  /// Returns the ```DE``` register
  pub fn get_de(&self) -> u8 {
    unimplemented!();
  }

  /// Returns the ```HL``` register
  pub fn get_hl(&self) -> u8 {
    unimplemented!();
  }

  /// Sets the ```AF``` register
  pub fn set_af(&self) -> u16 {
    unimplemented!();
  }

  /// Sets the ```BC``` register
  pub fn set_bc(&self) -> u8 {
    unimplemented!();
  }

  /// Sets the ```DE``` register
  pub fn set_de(&self) -> u8 {
    unimplemented!();
  }

  /// Sets the ```HL``` register
  pub fn set_hl(&self) -> u8 {
    unimplemented!();
  }
}
