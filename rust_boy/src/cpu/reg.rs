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
pub struct Registers{
  af: u16,
  bc: u16,
  de: u16,
  hl: u16,
  sp: u16,
  pc: u16,
}

impl Registers
{    
  /// Returns a new object of the type Registers
  pub fn new() -> Registers {
    Registers {
      af: 0x0000,
      bc: 0x0000,
      de: 0x0000,
      hl: 0x0000,
      sp: 0x0000,
      pc: 0x0000,
    }
  }

  /// Prints the registers information
  pub fn debug(&self)
  {
    println!("Registers Debug: \n\n");

    println!("|Register | Value  | Lower | ");
    println!("| AF      | {:#06x} | {:#04x}  |", self.af, self.get_a());
    println!("| BC      | {:#06x} |", self.bc);
    println!("| DE      | {:#06x} |", self.de);
    println!("| HL      | {:#06x} |", self.hl);
    println!("| PC      | {:#06x} |", self.pc);
    println!("| SP      | {:#06x} |", self.sp);
  }

  /// Returns the high byte of the ```AF``` register
  pub fn get_a(&self) -> u8 {
    return ((self.af & 0xFF00) >> 8) as u8;
  }
}
