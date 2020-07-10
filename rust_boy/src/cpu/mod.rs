mod reg;
use crate::memory;

/// The CPU structure
pub struct Cpu{
  pub registers : reg::Registers,
  pub cb_prefix : bool,
}

impl Cpu
{
  /// Returns a new object of the type Cpu
  pub fn new() -> Cpu {
    Cpu{
      registers: reg::Registers::new(),
      cb_prefix: false,
    }
  }

  fn decode_cb(&mut self, opcode: u8, ram: &mut memory::Memory) -> u8 {
    panic!("Opcode {:#04x} not implemented!", opcode)
  }
  
  fn decode(&mut self, opcode: u8, ram: &mut memory::Memory) -> u8
  {
    match opcode{
      0x00 => 1, // NOP

      0x01 => {self.registers.pc += 1; self.registers.b = ram.get(self.registers.pc); self.registers.pc += 1; self.registers.c = ram.get(self.registers.pc); 3}, // LD BC, (u16)
      0x11 => {self.registers.pc += 1; self.registers.d = ram.get(self.registers.pc); self.registers.pc += 1; self.registers.e = ram.get(self.registers.pc); 3}, // LD DE, (u16)
      0x21 => {self.registers.pc += 1; self.registers.h = ram.get(self.registers.pc); self.registers.pc += 1; self.registers.l = ram.get(self.registers.pc); 3}, // LD HL, (u16)
      0x31 => {self.registers.pc += 1; let x = ram.get(self.registers.pc); self.registers.pc += 1; let y = ram.get(self.registers.pc); self.registers.sp = ((y as u16) << 8) | x as u16; 3}, // LD SP, (u16)

      0x08 => {self.registers.pc += 1; ram.set(self.registers.pc, (self.registers.sp & 0x00FF) as u8); self.registers.pc += 1; ram.set(self.registers.pc, ((self.registers.sp & 0xFF00) >> 8) as u8); 5}, // LD (u16), SP

      0xE0 => {self.registers.pc += 1; let x = (ram.get(self.registers.pc) as u16) & 0xFF00; ram.set(x, self.registers.a); 3}, // LD (FF00+u8), A 
      0xF0 => {self.registers.pc += 1; let x = (ram.get(self.registers.pc) as u16) & 0xFF00; self.registers.a = ram.get(x); 3}, // LD A, (FF00+u8)
      
      0xE2 => {ram.set((self.registers.c as u16) & 0xFF00, self.registers.a); 2}, // LD (FF00+C), A
      0xF2 => {self.registers.a = ram.get((self.registers.c as u16) & 0xFF00); 2}, // LD A, (FF00+C)

      0xEA => {self.registers.pc += 1; let x = ram.get(self.registers.pc); self.registers.pc += 1; let y = ram.get(self.registers.pc); ram.set(((y as u16) << 8) | x as u16, self.registers.a); 4}, // LD (u16), A
      0xFA => {self.registers.pc += 1; let x = ram.get(self.registers.pc); self.registers.pc += 1; let y = ram.get(self.registers.pc); self.registers.a = ram.get(((y as u16) << 8) | x as u16); 4}, // LD A, (u16)

      0x02 => {ram.set(self.registers.get_bc(), self.registers.a); 2}, // LD (BC), A
      0x12 => {ram.set(self.registers.get_de(), self.registers.a); 2}, // LD A, (DE)
      0x22 => {let tmp = self.registers.get_hl(); ram.set(tmp, self.registers.a); self.registers.set_hl(tmp + 1); 2}, // LD (HL+), A
      0x32 => {let tmp = self.registers.get_hl(); ram.set(tmp, self.registers.a); self.registers.set_hl(tmp - 1); 2}, // LD (HL-), A
      
      0x06 => {self.registers.pc += 1; self.registers.b = ram.get(self.registers.pc); 2}, // LD B, u8
      0x16 => {self.registers.pc += 1; self.registers.d = ram.get(self.registers.pc); 2}, // LD D, u8
      0x26 => {self.registers.pc += 1; self.registers.h = ram.get(self.registers.pc); 2}, // LD H, u8
      0x36 => {self.registers.pc += 1; ram.set(self.registers.get_hl(), ram.get(self.registers.pc)); 3}, // LD (HL), u8

      0x0E => {self.registers.pc += 1; self.registers.c = ram.get(self.registers.pc); 2}, // LD C, u8
      0x1E => {self.registers.pc += 1; self.registers.e = ram.get(self.registers.pc); 2}, // LD E, u8
      0x2E => {self.registers.pc += 1; self.registers.l = ram.get(self.registers.pc); 2}, // LD L, u8
      0x3E => {self.registers.pc += 1; self.registers.a = ram.get(self.registers.pc); 2}, // LD A, u8

      0x40 => {self.registers.b = self.registers.b; 1}, // LD B, B
      0x41 => {self.registers.b = self.registers.c; 1}, // LD B, C
      0x42 => {self.registers.b = self.registers.d; 1}, // LD B, D
      0x43 => {self.registers.b = self.registers.e; 1}, // LD B, E
      0x44 => {self.registers.b = self.registers.h; 1}, // LD B, H
      0x45 => {self.registers.b = self.registers.l; 1}, // LD B, L
      0x46 => {self.registers.b = ram.get(self.registers.get_hl()); 2}, // LD B, (HL)
      0x47 => {self.registers.b = self.registers.a; 1}, // LD B, A
      0x48 => {self.registers.c = self.registers.b; 1}, // LD C, B
      0x49 => {self.registers.c = self.registers.c; 1}, // LD C, C
      0x4A => {self.registers.c = self.registers.d; 1}, // LD C, D
      0x4B => {self.registers.c = self.registers.e; 1}, // LD C, E
      0x4C => {self.registers.c = self.registers.h; 1}, // LD C, H
      0x4D => {self.registers.c = self.registers.l; 1}, // LD C, L
      0x4E => {self.registers.c = ram.get(self.registers.get_hl()); 2}, // LD C, (HL)
      0x4F => {self.registers.c = self.registers.a; 1}, // LD C, A
      0x50 => {self.registers.d = self.registers.b; 1}, // LD D, B
      0x51 => {self.registers.d = self.registers.c; 1}, // LD D, C
      0x52 => {self.registers.d = self.registers.d; 1}, // LD D, D
      0x53 => {self.registers.d = self.registers.e; 1}, // LD D, E
      0x54 => {self.registers.d = self.registers.h; 1}, // LD D, H
      0x55 => {self.registers.d = self.registers.l; 1}, // LD D, L
      0x56 => {self.registers.d = ram.get(self.registers.get_hl()); 2}, // LD D, (HL)
      0x57 => {self.registers.d = self.registers.a; 1}, // LD D, A
      0x58 => {self.registers.e = self.registers.b; 1}, // LD E, B
      0x59 => {self.registers.e = self.registers.c; 1}, // LD E, C
      0x5A => {self.registers.e = self.registers.d; 1}, // LD E, D
      0x5B => {self.registers.e = self.registers.e; 1}, // LD E, E
      0x5C => {self.registers.e = self.registers.h; 1}, // LD E, H
      0x5D => {self.registers.e = self.registers.l; 1}, // LD E, L
      0x5E => {self.registers.e = ram.get(self.registers.get_hl()); 2}, // LD E, (HL)
      0x5F => {self.registers.e = self.registers.a; 1}, // LD E, A
      0x60 => {self.registers.h = self.registers.b; 1}, // LD H, B
      0x61 => {self.registers.h = self.registers.c; 1}, // LD H, C
      0x62 => {self.registers.h = self.registers.d; 1}, // LD H, D
      0x63 => {self.registers.h = self.registers.e; 1}, // LD H, E
      0x64 => {self.registers.h = self.registers.h; 1}, // LD H, H
      0x65 => {self.registers.h = self.registers.l; 1}, // LD H, L
      0x66 => {self.registers.h = ram.get(self.registers.get_hl()); 2}, // LD H, (HL)
      0x67 => {self.registers.h = self.registers.a; 1}, // LD H, A
      0x68 => {self.registers.l = self.registers.b; 1}, // LD L, B
      0x69 => {self.registers.l = self.registers.c; 1}, // LD L, C
      0x6A => {self.registers.l = self.registers.d; 1}, // LD L, D
      0x6B => {self.registers.l = self.registers.e; 1}, // LD L, E
      0x6C => {self.registers.l = self.registers.h; 1}, // LD L, H
      0x6D => {self.registers.l = self.registers.l; 1}, // LD L, L
      0x6E => {self.registers.l = ram.get(self.registers.get_hl()); 2}, // LD L, (HL)
      0x6F => {self.registers.l = self.registers.a; 1}, // LD L, A
      0x70 => {ram.set(self.registers.get_hl(), self.registers.b); 2}, // LD (HL), B
      0x71 => {ram.set(self.registers.get_hl(), self.registers.c); 2}, // LD (HL), C
      0x72 => {ram.set(self.registers.get_hl(), self.registers.d); 2}, // LD (HL), D
      0x73 => {ram.set(self.registers.get_hl(), self.registers.e); 2}, // LD (HL), E
      0x74 => {ram.set(self.registers.get_hl(), self.registers.h); 2}, // LD (HL), H
      0x75 => {ram.set(self.registers.get_hl(), self.registers.l); 2}, // LD (HL), L
      0x77 => {ram.set(self.registers.get_hl(), self.registers.a); 2}, // LD (HL), A
      0x78 => {self.registers.a = self.registers.b; 1}, // LD A, B
      0x79 => {self.registers.a = self.registers.c; 1}, // LD A, C
      0x7A => {self.registers.a = self.registers.d; 1}, // LD A, D
      0x7B => {self.registers.a = self.registers.e; 1}, // LD A, E
      0x7C => {self.registers.a = self.registers.h; 1}, // LD A, H
      0x7D => {self.registers.a = self.registers.l; 1}, // LD A, L
      0x7E => {self.registers.a = ram.get(self.registers.get_hl()); 2}, // LD A, (HL)
      0x7F => {self.registers.a = self.registers.a; 1}, // LD A, A

      0xF9 => {self.registers.sp = self.registers.get_hl(); 2}, // LD SP, HL
      
      0x0A => {self.registers.b = ram.get(self.registers.get_bc()); 2}, // LD A, (BC)
      0x1A => {self.registers.b = ram.get(self.registers.get_de()); 2}, // LD A, (DE)
      0x2A => {let tmp = self.registers.get_hl(); self.registers.a = ram.get(tmp); self.registers.set_hl(tmp + 1); 2}, // LD A, (HL+)
      0x3A => {let tmp = self.registers.get_hl(); self.registers.a = ram.get(tmp); self.registers.set_hl(tmp - 1); 2}, // LD A, (HL-)

      0xC1 => {let tmp = self.pop(ram); self.registers.set_bc(tmp); 3}, // POP BC
      0xD1 => {let tmp = self.pop(ram); self.registers.set_de(tmp); 3}, // POP DE
      0xE1 => {let tmp = self.pop(ram); self.registers.set_hl(tmp); 3}, // POP HL
      0xF1 => {let tmp = self.pop(ram); self.registers.set_af(tmp); 3}, // POP AF

      0xC5 => {let tmp = self.registers.get_bc(); self.push(tmp, ram); 4}, // PUSH BC
      0xD5 => {let tmp = self.registers.get_de(); self.push(tmp, ram); 4}, // PUSH DE
      0xE5 => {let tmp = self.registers.get_hl(); self.push(tmp, ram); 4}, // PUSH HL
      0xF5 => {let tmp = self.registers.get_af(); self.push(tmp, ram); 4}, // PUSH AF

      0x80 => {self.add_u8(self.registers.b); 1}, // ADD A, B
      0x81 => {self.add_u8(self.registers.c); 1}, // ADD A, C
      0x82 => {self.add_u8(self.registers.d); 1}, // ADD A, D
      0x83 => {self.add_u8(self.registers.e); 1}, // ADD A, E
      0x84 => {self.add_u8(self.registers.h); 1}, // ADD A, H
      0x85 => {self.add_u8(self.registers.l); 1}, // ADD A, L
      0x86 => {self.add_u8(ram.get(self.registers.get_hl())); 2}, // ADD A, (HL)
      0x87 => {self.add_u8(self.registers.a); 1}, // ADD A, A
      0xC6 => {self.registers.pc += 1; self.add_u8(ram.get(self.registers.pc)); 2}, // ADD A, u8

      0xA0 => {self.registers.a &= self.registers.b; self.registers.f = 0b0010_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // AND A, B
      0xA1 => {self.registers.a &= self.registers.c; self.registers.f = 0b0010_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // AND A, C
      0xA2 => {self.registers.a &= self.registers.d; self.registers.f = 0b0010_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // AND A, D
      0xA3 => {self.registers.a &= self.registers.e; self.registers.f = 0b0010_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // AND A, E
      0xA4 => {self.registers.a &= self.registers.h; self.registers.f = 0b0010_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // AND A, H
      0xA5 => {self.registers.a &= self.registers.l; self.registers.f = 0b0010_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // AND A, L
      0xA6 => {self.registers.a &= ram.get(self.registers.get_hl()); self.registers.f = 0b0010_0000; self.registers.set_flag_zf(self.registers.a == 0); 2}, // AND A, (HL)
      0xA7 => {self.registers.a &= self.registers.a; self.registers.f = 0b0010_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // AND A, A      
      0xE6 => {self.registers.pc += 1; self.registers.a &= ram.get(self.registers.pc); self.registers.f = 0b0010_0000; self.registers.set_flag_zf(self.registers.a == 0); 2}, // AND A, u8

      0xA8 => {self.registers.a ^= self.registers.b; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // XOR A, B
      0xA9 => {self.registers.a ^= self.registers.c; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // XOR A, C
      0xAA => {self.registers.a ^= self.registers.d; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // XOR A, D
      0xAB => {self.registers.a ^= self.registers.e; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // XOR A, E
      0xAC => {self.registers.a ^= self.registers.h; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // XOR A, H
      0xAD => {self.registers.a ^= self.registers.l; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // XOR A, L
      0xAE => {self.registers.a ^= ram.get(self.registers.get_hl()); self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 2}, // XOR A, (HL)
      0xAF => {self.registers.a ^= self.registers.a; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // XOR A, A      
      0xEE => {self.registers.pc += 1; self.registers.a ^= ram.get(self.registers.pc); self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 2}, // XOR A, u8

      0xB0 => {self.registers.a |= self.registers.b; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // OR A, B
      0xB1 => {self.registers.a |= self.registers.c; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // OR A, C
      0xB2 => {self.registers.a |= self.registers.d; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // OR A, D
      0xB3 => {self.registers.a |= self.registers.e; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // OR A, E
      0xB4 => {self.registers.a |= self.registers.h; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // OR A, H
      0xB5 => {self.registers.a |= self.registers.l; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // OR A, L
      0xB6 => {self.registers.a |= ram.get(self.registers.get_hl()); self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 2}, // OR A, (HL)
      0xB7 => {self.registers.a |= self.registers.a; self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 1}, // OR A, A      
      0xF6 => {self.registers.pc += 1; self.registers.a |= ram.get(self.registers.pc); self.registers.f = 0b0000_0000; self.registers.set_flag_zf(self.registers.a == 0); 2}, // OR A, u8

      0xC3 => {self.registers.pc += 1; let l = ram.get(self.registers.pc); self.registers.pc += 1; let h = ram.get(self.registers.pc); self.registers.pc = ((h as u16) << 8) | l as u16; 4}, // JP u16
      0xE9 => {self.registers.pc = self.registers.get_hl(); 1}, // JP HL

      0x20 => {self.registers.pc += 1; let e = ram.get(self.registers.pc); let cicles = if self.registers.get_flag_zf() == 0 {self.registers.pc = if (e & 0b1000_0000) >> 7 == 1 {self.registers.pc - ((e & 0b0111_1111) as u16)} else {self.registers.pc + ((e & 0b0111_1111) as u16)}; 3} else {2}; cicles}, // JR NZ, i8
      0x30 => {self.registers.pc += 1; let e = ram.get(self.registers.pc); let cicles = if self.registers.get_flag_c() == 0 {self.registers.pc = if (e & 0b1000_0000) >> 7 == 1 {self.registers.pc - ((e & 0b0111_1111) as u16)} else {self.registers.pc + ((e & 0b0111_1111) as u16)}; 3} else {2}; cicles},  // JR NC, i8

      0x18 => {self.registers.pc += 1; let e = ram.get(self.registers.pc); self.registers.pc = if (e & 0b1000_0000) >> 7 == 1 {self.registers.pc - ((e & 0b0111_1111) as u16)} else {self.registers.pc + ((e & 0b0111_1111) as u16)}; 3}, // JR Z, i8
      0x28 => {self.registers.pc += 1; let e = ram.get(self.registers.pc); let cicles = if self.registers.get_flag_zf() == 1 {self.registers.pc = if (e & 0b1000_0000) >> 7 == 1 {self.registers.pc - ((e & 0b0111_1111) as u16)} else {self.registers.pc + ((e & 0b0111_1111) as u16)}; 3} else {2}; cicles}, // JR Z, i8
      0x38 => {self.registers.pc += 1; let e = ram.get(self.registers.pc); let cicles = if self.registers.get_flag_c() == 1 {self.registers.pc = if (e & 0b1000_0000) >> 7 == 1 {self.registers.pc - ((e & 0b0111_1111) as u16)} else {self.registers.pc + ((e & 0b0111_1111) as u16)}; 3} else {2}; cicles},  // JR C, i8

      0xC2 => {self.registers.pc += 1; let l = ram.get(self.registers.pc); self.registers.pc += 1; let h = ram.get(self.registers.pc); let cicles = if self.registers.get_flag_zf() == 0 {self.registers.pc = ((h as u16) << 8) | l as u16; 4} else {3}; cicles}, // JP NZ, u16
      0xD2 => {self.registers.pc += 1; let l = ram.get(self.registers.pc); self.registers.pc += 1; let h = ram.get(self.registers.pc); let cicles = if self.registers.get_flag_c() == 0 {self.registers.pc = ((h as u16) << 8) | l as u16; 4} else {3}; cicles},  // JP NC, u16
     
      0xCA => {self.registers.pc += 1; let l = ram.get(self.registers.pc); self.registers.pc += 1; let h = ram.get(self.registers.pc); let cicles = if self.registers.get_flag_zf() == 1 {self.registers.pc = ((h as u16) << 8) | l as u16; 4} else {3}; cicles}, // JP Z, u16
      0xDA => {self.registers.pc += 1; let l = ram.get(self.registers.pc); self.registers.pc += 1; let h = ram.get(self.registers.pc); let cicles = if self.registers.get_flag_c() == 1 {self.registers.pc = ((h as u16) << 8) | l as u16; 4} else {3}; cicles},  // JP C, u16
     
      0xCD => {self.registers.pc += 1; let l = ram.get(self.registers.pc); self.registers.pc += 1; let h = ram.get(self.registers.pc); self.push(self.registers.pc, ram); self.registers.pc = ((h as u16) << 8) | l as u16; 6},  // CALL u16
            
      0xC9 => {self.registers.pc = self.pop(ram); 4},  // RET
      
      0xC7 => {self.push(self.registers.pc, ram); self.registers.pc = 0x00; 4}, // RST 0x00
      0xD7 => {self.push(self.registers.pc, ram); self.registers.pc = 0x10; 4}, // RST 0x10
      0xE7 => {self.push(self.registers.pc, ram); self.registers.pc = 0x20; 4}, // RST 0x20
      0xF7 => {self.push(self.registers.pc, ram); self.registers.pc = 0x30; 4}, // RST 0x30
      0xCF => {self.push(self.registers.pc, ram); self.registers.pc = 0x08; 4}, // RST 0x08
      0xCF => {self.push(self.registers.pc, ram); self.registers.pc = 0x18; 4}, // RST 0x18
      0xCF => {self.push(self.registers.pc, ram); self.registers.pc = 0x28; 4}, // RST 0x28
      0xCF => {self.push(self.registers.pc, ram); self.registers.pc = 0x38; 4}, // RST 0x38
          
      0x03 => {self.registers.set_bc(self.inc_u16(self.registers.get_bc())); 2}, // INC BC
      0x13 => {self.registers.set_de(self.inc_u16(self.registers.get_de())); 2}, // INC DE
      0x23 => {self.registers.set_hl(self.inc_u16(self.registers.get_hl())); 2}, // INC HL
      0x33 => {self.registers.sp = self.inc_u16(self.registers.sp); 2}, // INC SP
      
      0x04 => {self.registers.b = self.inc_u8(self.registers.b); 1}, // INC B
      0x14 => {self.registers.d = self.inc_u8(self.registers.d); 1}, // INC D
      0x24 => {self.registers.h = self.inc_u8(self.registers.h); 1}, // INC H
      0x34 => {let tmp = ram.get(self.registers.get_hl()); ram.set(self.registers.get_hl(), self.inc_u8(tmp)); 3}, // INC (HL)
      0x0C => {self.registers.c = self.inc_u8(self.registers.c); 1}, // INC C
      0x1C => {self.registers.e = self.inc_u8(self.registers.e); 1}, // INC E
      0x2C => {self.registers.l = self.inc_u8(self.registers.l); 1}, // INC L
      0x3C => {self.registers.a = self.inc_u8(self.registers.a); 1}, // INC A
      
      0x0B => {self.registers.set_bc(self.dec_u16(self.registers.get_bc())); 2}, // DEC BC
      0x1B => {self.registers.set_de(self.dec_u16(self.registers.get_de())); 2}, // DEC DE
      0x2B => {self.registers.set_hl(self.dec_u16(self.registers.get_hl())); 2}, // DEC HL
      0x3B => {self.registers.sp = self.dec_u16(self.registers.sp); 2}, // DEC SP
      
      0x05 => {self.registers.b = self.dec_u8(self.registers.b); 1}, // DEC B
      0x15 => {self.registers.d = self.dec_u8(self.registers.d); 1}, // DEC D
      0x25 => {self.registers.h = self.dec_u8(self.registers.h); 1}, // DEC H
      0x35 => {let tmp = ram.get(self.registers.get_hl()); ram.set(self.registers.get_hl(), self.dec_u8(tmp)); 3}, // DEC (HL)
      0x0D => {self.registers.c = self.dec_u8(self.registers.c); 1}, // DEC C
      0x1D => {self.registers.e = self.dec_u8(self.registers.e); 1}, // DEC E
      0x2D => {self.registers.l = self.dec_u8(self.registers.l); 1}, // DEC L
      0x3D => {self.registers.a = self.dec_u8(self.registers.a); 1}, // DEC A
      
      0x2F => {self.registers.f |= 0b0110_0000; self.registers.a = !self.registers.a; 1} // CPL
      0x3F => {let c = (!self.registers.get_flag_c()) & 1; self.registers.f = ((self.registers.f | 0b0000_0000) & 0b1000_0000) | (c << 4); 1} // CCF
      0x37 => {self.registers.f = (self.registers.f | 0b0001_0000) & 0b1001_0000; 1} // SCF

      _      => panic!("Opcode {:#04x} not implemented!", opcode),
    }
  }

  /// Adds an u8 value to the A register
  pub fn add_u8(&mut self, value: u8){
    // New flag values: Z0HC ----
  
    // Clears the F register
    self.registers.f &= 0b0;

    // save old a value
    let a = self.registers.a;
  
    self.registers.a = match self.registers.a.checked_add(value){
      Some(v) => v, // NO OVERFLOW
      None    => 0, // OVERFLOW
    };
  
    // Sets the Zero flag
    self.registers.set_flag_zf(value == 0x0);
  
    // Half Carry flag    
    self.registers.set_flag_h((((value & 0xF) + (a & 0xF)) & 0x10) == 0x10);

    // Carry Flag
    self.registers.set_flag_c(((((value as u16 & 0xFF) + (a as u16 & 0xFF)) as u16) & 0x0100) == 0x0100);
  }

  /// Decrements an u16 value
  pub fn dec_u16(&self, value: u16) -> u16{
    match value.checked_sub(1){
      Some(v) => v, // NO OVERFLOW
      None    => 0, // OVERFLOW
    }
  }

  /// Increments an u16 value
  pub fn inc_u16(&self, value: u16) -> u16{
    match value.checked_add(1){
      Some(v) => v, // NO OVERFLOW
      None    => 0, // OVERFLOW
    }
  }

  /// Increments an u8 value
  pub fn inc_u8(&mut self, value: u8) -> u8{
    // New flag values: Z0H- ----

    // Clears the F register but keeps the carry value
    self.registers.f &= 0b0001_0000;

    let result = match value.checked_add(1){
      Some(v) => v, // NO OVERFLOW
      None    => 0, // OVERFLOW
    };

    // Sets the Zero flag
    self.registers.set_flag_zf(value == 0x0);

    // Half Carry flag    
    self.registers.set_flag_h((((1 & 0xf) + (value & 0xf)) & 0x10) == 0x10);

    result
  }

    /// Decrements an u8 value
    pub fn dec_u8(&mut self, value: u8) -> u8{
      // New flag values: Z1H- ----
  
      // Clears the F register but keeps the carry value
      self.registers.f &= 0b0001_0000;
      // Sets the sub flag as 1
      self.registers.f |= 0b0100_0000;
  
      let result = match value.checked_sub(1){
        Some(v) => v, // NO OVERFLOW
        None    => 0, // OVERFLOW
      };
  
      // Sets the Zero flag
      self.registers.set_flag_zf(value == 0x0);
  
      // Half Carry flag    
      self.registers.set_flag_h((((1 & 0xf) + (value & 0xf)) & 0x10) == 0x10);
  
      result
    }

  pub fn cicle(&mut self, opcode: u8, ram: &mut memory::Memory) -> u8
  {
    let cicles = if self.cb_prefix {self.decode_cb(opcode, ram)} else {self.decode(opcode, ram)};

    self.registers.pc += 1;

    cicles
  }

  fn pop(&mut self, ram: &mut memory::Memory) -> u16
  {
    self.registers.sp += 0x01;
    let x = ram.get(self.registers.sp); 
    self.registers.sp += 0x01;
    let y = ram.get(self.registers.sp);
    
    ((y as u16) << 8) | x as u16
  }

  fn push(&mut self, value: u16, ram: &mut memory::Memory)
  {
    let x = (value & 0x00FF) as u8;
    let y = ((value & 0xFF00) >> 8) as u8;

    self.registers.sp -= 0x01;
    ram.set(self.registers.sp, y);
    self.registers.sp -= 0x01;
    ram.set(self.registers.sp, x);
  }

  /// Debugs the cpu information
  pub fn debug(&self)
  {
    self.registers.debug()
  }
}
