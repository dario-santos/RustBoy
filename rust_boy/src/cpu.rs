// Todo: CPU
//  - Flags
//  - Registers
//  - Instructions

// The Flag Registor (The lower 8 bits of AF)
//
// | Bit | Name | Set | Clr | Explanation |
// +-----+------+-----+-----+-------------+
// |  7  |  zf  |  Z  |     |
// |  6  |  n   |  -  |     |
// |  5  |  h   |  -  |     |
// |  4  |  cy  |  C  |     |
// | 0-3 |  -   |  -  |     |

// The registers of the LR35902 processor
// 
// | Register | High | Low | Name/Functionality |
// +:--------:+------+-----+--------------------+
// |    AF    |  A   |  -  | Accumulator, Flags |
// |    BC    |  B   |  C  | BC                 |
// |    DE    |  D   |  E  | DE                 |
// |    HL    |  H   |  L  | HL                 |
// |    SP    |  -   |  -  | Stack Pointer      |
// |    PC    |  -   |  -  | Program Counter    |
struct Registers{
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
  pub const fn new() -> Registers {
    Registers {
      af: 0x0000,
      bc: 0x0000,
      de: 0x0000,
      hl: 0x0000,
      sp: 0x0000,
      pc: 0x0000,
    }
  }
  /// Returns the high byte of the ```AF``` register
  pub const fn get_a(&self) -> u8 {
    return ((self.af & 0xFF00) >> 8) as u8;
  }
}

static REGISTER : Registers = Registers::new();

pub fn debug() 
{
  println!("Registers Debug: \n\n");

  println!("|Register | Value  | Lower | ");
  println!("| AF      | {:#06x} | {:#04x}  |", REGISTER.af, REGISTER.get_a());
  println!("| BC      | {:#06x} |", REGISTER.bc);
  println!("| DE      | {:#06x} |", REGISTER.de);
  println!("| HL      | {:#06x} |", REGISTER.hl);
  println!("| PC      | {:#06x} |", REGISTER.pc);
  println!("| SP      | {:#06x} |", REGISTER.sp);
}