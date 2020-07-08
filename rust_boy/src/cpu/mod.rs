//! This file contains the cpu and it's instructions
mod reg;

/// The CPU structure
pub struct Cpu{
  pub registers : reg::Registers,
}

impl Cpu
{
  /// Returns a new object of the type Cpu
  pub fn new() -> Cpu {
    Cpu{
      registers: reg::Registers::new(),
    }
  }

  pub fn decode(&mut self, opcode: u8, ram: &mut Vec<u8>) -> u8
  {
    let cicles = match opcode{
      0x00 => 1, // NOP

      0x02 => {ram[self.registers.get_bc() as usize] = self.registers.a; 2}, // LD (BC), A
      0x12 => {ram[self.registers.get_de() as usize] = self.registers.a; 2}, // LD A, (DE)
      0x22 => {let tmp = self.registers.get_hl(); ram[tmp as usize] = self.registers.a; self.registers.set_hl(tmp + 1); 2}, // LD (HL+), A
      0x32 => {let tmp = self.registers.get_hl(); ram[tmp as usize] = self.registers.a; self.registers.set_hl(tmp - 1); 2}, // LD (HL-), A
      
      0x06 => {self.registers.pc += 1; self.registers.b = ram[self.registers.pc as usize]; 2}, // LD B, u8
      0x16 => {self.registers.pc += 1; self.registers.d = ram[self.registers.pc as usize]; 2}, // LD D, u8
      0x26 => {self.registers.pc += 1; self.registers.h = ram[self.registers.pc as usize]; 2}, // LD H, u8
      0x36 => {self.registers.pc += 1; ram[self.registers.get_hl() as usize] = ram[self.registers.pc as usize]; 3}, // LD (HL), u8

      0x0E => {self.registers.pc += 1; self.registers.c = ram[self.registers.pc as usize]; 2}, // LD C, u8
      0x1E => {self.registers.pc += 1; self.registers.e = ram[self.registers.pc as usize]; 2}, // LD E, u8
      0x2E => {self.registers.pc += 1; self.registers.l = ram[self.registers.pc as usize]; 2}, // LD L, u8
      0x3E => {self.registers.pc += 1; self.registers.a = ram[self.registers.pc as usize]; 2}, // LD A, u8


      0x40 => {self.registers.b = self.registers.b; 1}, // LD B, B
      0x41 => {self.registers.b = self.registers.c; 1}, // LD B, C
      0x42 => {self.registers.b = self.registers.d; 1}, // LD B, D
      0x43 => {self.registers.b = self.registers.e; 1}, // LD B, E
      0x44 => {self.registers.b = self.registers.h; 1}, // LD B, H
      0x45 => {self.registers.b = self.registers.l; 1}, // LD B, L
      0x46 => {self.registers.b = ram[self.registers.get_hl() as usize]; 2}, // LD B, (HL)
      0x47 => {self.registers.b = self.registers.a; 1}, // LD B, A
      0x48 => {self.registers.c = self.registers.b; 1}, // LD C, B
      0x49 => {self.registers.c = self.registers.c; 1}, // LD C, C
      0x4A => {self.registers.c = self.registers.d; 1}, // LD C, D
      0x4B => {self.registers.c = self.registers.e; 1}, // LD C, E
      0x4C => {self.registers.c = self.registers.h; 1}, // LD C, H
      0x4D => {self.registers.c = self.registers.l; 1}, // LD C, L
      0x4E => {self.registers.c = ram[self.registers.get_hl() as usize]; 2}, // LD C, (HL)
      0x4F => {self.registers.c = self.registers.a; 1}, // LD C, A
      0x50 => {self.registers.d = self.registers.b; 1}, // LD D, B
      0x51 => {self.registers.d = self.registers.c; 1}, // LD D, C
      0x52 => {self.registers.d = self.registers.d; 1}, // LD D, D
      0x53 => {self.registers.d = self.registers.e; 1}, // LD D, E
      0x54 => {self.registers.d = self.registers.h; 1}, // LD D, H
      0x55 => {self.registers.d = self.registers.l; 1}, // LD D, L
      0x56 => {self.registers.d = ram[self.registers.get_hl() as usize]; 2}, // LD D, (HL)
      0x57 => {self.registers.d = self.registers.a; 1}, // LD D, A
      0x58 => {self.registers.e = self.registers.b; 1}, // LD E, B
      0x59 => {self.registers.e = self.registers.c; 1}, // LD E, C
      0x5A => {self.registers.e = self.registers.d; 1}, // LD E, D
      0x5B => {self.registers.e = self.registers.e; 1}, // LD E, E
      0x5C => {self.registers.e = self.registers.h; 1}, // LD E, H
      0x5D => {self.registers.e = self.registers.l; 1}, // LD E, L
      0x5E => {self.registers.e = ram[self.registers.get_hl() as usize]; 2}, // LD E, (HL)
      0x5F => {self.registers.e = self.registers.a; 1}, // LD E, A
      0x60 => {self.registers.h = self.registers.b; 1}, // LD H, B
      0x61 => {self.registers.h = self.registers.c; 1}, // LD H, C
      0x62 => {self.registers.h = self.registers.d; 1}, // LD H, D
      0x63 => {self.registers.h = self.registers.e; 1}, // LD H, E
      0x64 => {self.registers.h = self.registers.h; 1}, // LD H, H
      0x65 => {self.registers.h = self.registers.l; 1}, // LD H, L
      0x66 => {self.registers.h = ram[self.registers.get_hl() as usize]; 2}, // LD H, (HL)
      0x67 => {self.registers.h = self.registers.a; 1}, // LD H, A
      0x68 => {self.registers.l = self.registers.b; 1}, // LD L, B
      0x69 => {self.registers.l = self.registers.c; 1}, // LD L, C
      0x6A => {self.registers.l = self.registers.d; 1}, // LD L, D
      0x6B => {self.registers.l = self.registers.e; 1}, // LD L, E
      0x6C => {self.registers.l = self.registers.h; 1}, // LD L, H
      0x6D => {self.registers.l = self.registers.l; 1}, // LD L, L
      0x6E => {self.registers.l = ram[self.registers.get_hl() as usize]; 2}, // LD L, (HL)
      0x6F => {self.registers.l = self.registers.a; 1}, // LD L, A
      0x70 => {ram[self.registers.get_hl() as usize] = self.registers.b; 2}, // LD (HL), B
      0x71 => {ram[self.registers.get_hl() as usize] = self.registers.c; 2}, // LD (HL), C
      0x72 => {ram[self.registers.get_hl() as usize] = self.registers.d; 2}, // LD (HL), D
      0x73 => {ram[self.registers.get_hl() as usize] = self.registers.e; 2}, // LD (HL), E
      0x74 => {ram[self.registers.get_hl() as usize] = self.registers.h; 2}, // LD (HL), H
      0x75 => {ram[self.registers.get_hl() as usize] = self.registers.l; 2}, // LD (HL), L
      0x77 => {ram[self.registers.get_hl() as usize] = self.registers.a; 2}, // LD (HL), A
      0x78 => {self.registers.a = self.registers.b; 1}, // LD A, B
      0x79 => {self.registers.a = self.registers.c; 1}, // LD A, C
      0x7A => {self.registers.a = self.registers.d; 1}, // LD A, D
      0x7B => {self.registers.a = self.registers.e; 1}, // LD A, E
      0x7C => {self.registers.a = self.registers.h; 1}, // LD A, H
      0x7D => {self.registers.a = self.registers.l; 1}, // LD A, L
      0x7E => {self.registers.a = ram[self.registers.get_hl() as usize]; 2}, // LD A, (HL)
      0x7F => {self.registers.a = self.registers.a; 1}, // LD A, A


      0xF9 => {self.registers.sp = self.registers.get_hl(); 1}, // LD SP, HL
      
      0x0A => {self.registers.b = ram[self.registers.get_bc() as usize]; 2}, // LD A, (BC)
      0x1A => {self.registers.b = ram[self.registers.get_de() as usize]; 2}, // LD A, (DE)
      0x2A => {let tmp = self.registers.get_hl(); self.registers.a = ram[tmp as usize]; self.registers.set_hl(tmp + 1); 2}, // LD A, (HL+)
      0x3A => {let tmp = self.registers.get_hl(); self.registers.a = ram[tmp as usize]; self.registers.set_hl(tmp - 1); 2}, // LD A, (HL-)

      0xC1 => {let tmp = self.pop(ram); self.registers.set_bc(tmp); 3}, // POP BC
      0xD1 => {let tmp = self.pop(ram); self.registers.set_de(tmp); 3}, // POP DE
      0xE1 => {let tmp = self.pop(ram); self.registers.set_hl(tmp); 3}, // POP HL

      0xC5 => {let tmp = self.registers.get_bc(); self.push(tmp, ram); 4}, // PUSH BC
      0xD5 => {let tmp = self.registers.get_de(); self.push(tmp, ram); 4}, // PUSH DE
      0xE5 => {let tmp = self.registers.get_hl(); self.push(tmp, ram); 4}, // PUSH HL
      0xF5 => {let tmp = self.registers.get_af(); self.push(tmp, ram); 4}, // PUSH AF

      0xA0 => {self.registers.a &= self.registers.b; self.registers.f = 0b0010_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // AND A, B
      0xA1 => {self.registers.a &= self.registers.c; self.registers.f = 0b0010_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // AND A, C
      0xA2 => {self.registers.a &= self.registers.d; self.registers.f = 0b0010_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // AND A, D
      0xA3 => {self.registers.a &= self.registers.e; self.registers.f = 0b0010_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // AND A, E
      0xA4 => {self.registers.a &= self.registers.h; self.registers.f = 0b0010_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // AND A, H
      0xA5 => {self.registers.a &= self.registers.l; self.registers.f = 0b0010_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // AND A, L
      0xA6 => {self.registers.a &= ram[self.registers.get_hl() as usize]; self.registers.f = 0b0010_0000; self.registers.set_flag_zf(self.registers.a == 0); 2}, // AND A, (HL)
      0xA7 => {self.registers.a &= self.registers.a; self.registers.f = 0b0010_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // AND A, A      
      0xE6 => {self.registers.pc += 1; self.registers.a &= ram[self.registers.pc as usize]; self.registers.f = 0b0010_0000; self.registers.set_flag_zf(self.registers.a == 0); 2}, // AND A, u8

      0xA8 => {self.registers.a ^= self.registers.b; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // XOR A, B
      0xA9 => {self.registers.a ^= self.registers.c; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // XOR A, C
      0xAA => {self.registers.a ^= self.registers.d; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // XOR A, D
      0xAB => {self.registers.a ^= self.registers.e; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // XOR A, E
      0xAC => {self.registers.a ^= self.registers.h; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // XOR A, H
      0xAD => {self.registers.a ^= self.registers.l; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // XOR A, L
      0xAE => {self.registers.a ^= ram[self.registers.get_hl() as usize]; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 2}, // XOR A, (HL)
      0xAF => {self.registers.a ^= self.registers.a; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // XOR A, A      
      0xEE => {self.registers.pc += 1; self.registers.a ^= ram[self.registers.pc as usize]; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 2}, // XOR A, u8


      0xB0 => {self.registers.a |= self.registers.b; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // OR A, B
      0xB1 => {self.registers.a |= self.registers.c; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // OR A, C
      0xB2 => {self.registers.a |= self.registers.d; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // OR A, D
      0xB3 => {self.registers.a |= self.registers.e; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // OR A, E
      0xB4 => {self.registers.a |= self.registers.h; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // OR A, H
      0xB5 => {self.registers.a |= self.registers.l; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // OR A, L
      0xB6 => {self.registers.a |= ram[self.registers.get_hl() as usize]; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 2}, // OR A, (HL)
      0xB7 => {self.registers.a |= self.registers.a; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // OR A, A      
      0xF6 => {self.registers.pc += 1; self.registers.a |= ram[self.registers.pc as usize]; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 2}, // OR A, u8

      _      => panic!("Opcode {:#04x} not implemented!", opcode),
    };

    self.registers.pc += 1;

    cicles
  }

  fn pop(&mut self, ram: &mut Vec<u8>) -> u16
  {
    self.registers.sp += 0x01;
    let x = ram[self.registers.sp as usize]; 
    self.registers.sp += 0x01;
    let y = ram[self.registers.sp as usize]; 
    
    ((y as u16) << 8) | x as u16
  }

  fn push(&mut self, value: u16, ram: &mut Vec<u8>) -> u16
  {
    let x = (value & 0x00FF) as u8;
    let y = ((value & 0xFF00) >> 8) as u8;


    self.registers.sp -= 0x01;
    ram[self.registers.sp as usize] = y;
    self.registers.sp -= 0x01;
    ram[self.registers.sp as usize] = x;
    
    ((y as u16) << 8) | x as u16
  }

  /// Debugs the cpu information
  pub fn debug(&self)
  {
    self.registers.debug()
  }
}

