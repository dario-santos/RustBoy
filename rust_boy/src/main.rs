mod cpu;
mod gpu;
mod memory;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::env;
use std::fs::File;
use std::io;
use std::path::Path;
use std::io::Read;


fn run(path: &Path) -> io::Result<()>
{
  // Reads rom file
  let mut rom_file = File::open(path)?;
  // Create the buffer
  let mut rom_buffer: Vec<u8> = Vec::new();
  rom_file.read_to_end(&mut rom_buffer)?;
  // Creates Memory
  let mut ram : memory::Memory = memory::Memory::new(rom_buffer);
  
  // Creates CPU
  let mut cpu = cpu::Cpu::new();
  let mut cpu_cicles = 0;

  // Create Display
  let sdl_context = sdl2::init().unwrap();
  let mut canvas = gpu::Display::new(&sdl_context, 160, 144);
  let mut event_pump = sdl_context.event_pump().unwrap();

  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          break 'running
        },
        // skip mouse motion intentionally because of the verbose it might cause.
        Event::MouseMotion {..} => {},
        e => {
          println!("{:?}", e);
        }
      }
    }

    canvas.clear();
    canvas.present();
    
    let opcode = ram.get(cpu.registers.pc);

    if cpu_cicles == 0 {
      //cpu_cicles += cpu.cicle(opcode, &mut ram);
    }
    
    println!("Opcode: {:#04x}", opcode);
    //cpu.debug();
    println!("CPU CICLES WAIT: {}", cpu_cicles);
    
    //cpu_cicles -= 1;
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
