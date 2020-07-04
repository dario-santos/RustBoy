//! This file contains the cpu and it's instructions
mod reg;

/// The CPU structure
pub struct Cpu{
  registers : reg::Registers,
}

impl Cpu
{    
  /// Returns a new object of the type Cpu
  pub fn new() -> Cpu {
    Cpu{
      registers : reg::Registers::new(),
    }
  }

  /// Debugs the cpu information
  pub fn debug(&self)
  {
    self.registers.debug();
  }
}
