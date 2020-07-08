extern crate sdl2;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::{io, fmt};
use std::path::Path;
use std::io::Read;

mod cpu;
mod gpu;

fn run()
{
  let sdl_context = sdl2::init().unwrap();
  
  let mut canvas = gpu::Display::new(&sdl_context, 160, 144);
  let mut canvas_debug = gpu::Display::new(&sdl_context, 800, 600);
  
  loop {
    canvas.clear();
    canvas_debug.clear();
    
    canvas_debug.present();
    canvas_debug.present();
  }
}

fn main() -> io::Result<()>
{

  let args: Vec<_> = env::args().collect();

  let path: &Path;
  
  if args.len() < 2 {
      panic!("Usage: ./demo rom.gb")
    } else {
      path = Path::new(&args[1]);
  }

  let mut rom_file = File::open(path)?;

  // Create the buffer
  let mut rom_buffer: Vec<u8> = Vec::new();
  
  rom_file.read_to_end(&mut rom_buffer)?;
  

  let mut cpu = cpu::Cpu::new();
  
  let mut ram : Vec<u8> = vec![0x0; 0xFFFF];
  println!("Opcode: 0xC1");
  cpu.debug();

  let mut cpu_cicles = 0;
  loop{
    if cpu_cicles == 0 {
      cpu_cicles += cpu.decode(rom_buffer[cpu.registers.pc as usize], &mut ram);
    }
    cpu.debug();
    println!("CPU CICLES WAIT: {}", cpu_cicles);
    
    cpu_cicles -= 1;
  }

  Ok(())
}
