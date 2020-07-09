mod cpu;
mod gpu;
mod memory;
mod cartridge;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::{Duration, SystemTime};
use std::thread::sleep;

use std::env;
use std::fs::File;
use std::io;
use std::path::Path;
use std::io::Read;

fn run(path: &Path) -> io::Result<()>
{  
  // Loads cartridge
  let cart: cartridge::Cartridge = cartridge::Cartridge::load(path);
  cart.debug();
  
  // Creates Memory
  let mut ram : memory::Memory = memory::Memory::new(cart);
  
  // Creates CPU
  let mut cpu = cpu::Cpu::new();
  let mut cpu_cicles = 0;

  // Create Display
  let sdl_context = sdl2::init().unwrap();
  let mut gpu = gpu::Display::new(&sdl_context, 160, 144);
  let mut event_pump = sdl_context.event_pump().unwrap();

  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          break 'running
        },
        _ => {}, // ignore all other events
      }
    }

    gpu.draw();
    
    let opcode = ram.get(cpu.registers.pc);

    if cpu_cicles == 0 {
      cpu_cicles += cpu.cicle(0x80, &mut ram);
    }
    
    println!{"Opcode: {:#04x}", opcode}
    cpu.debug();
    
    cpu_cicles -= 1;
    //sleep(Duration::from_secs(1));
};

  Ok(())
}

fn main()
{
  let args: Vec<_> = env::args().collect();

  let path: &Path;
  
  if args.len() < 2 {
      panic!("Usage: ./demo rom.gb")
    } else {
      path = Path::new(&args[1]);
  }
  
  run(path);
}
