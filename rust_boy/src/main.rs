#![allow(warnings, unused)]

mod cartridge;
mod cpu;
mod gpu;
mod memory;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::sys::exit;

use std::thread::sleep;
use std::time::{Duration, SystemTime};

use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

fn run(path: &Path) -> io::Result<()> {
    // Loads cartridge
    let cart: cartridge::Cartridge = cartridge::Cartridge::load(path);
    cart.debug();

    // Creates Memory
    let mut ram: memory::Memory = memory::Memory::new(cart);

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
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {} // ignore all other events
            }
        }

        gpu.draw();
        //gpu.update(&mut ram);

        let opcode = ram.get(cpu.registers.pc);

        //println!{"Opcode: {:#04x}", opcode}
        //cpu.debug();

        if cpu_cicles == 0 {
            cpu_cicles += cpu.cicle(opcode, &mut ram);
        }

        cpu_cicles -= 1;
        sleep(Duration::from_secs(1));
    }

    Ok(())
}

fn main()-> Result<(), String> {
{

    let args: Vec<String> = env::args().collect();

    let path: &Path;

    if args.len() < 2 {
        let error_msg = "you need to pass the path to game as an argument.\n Example: ./rustboy.exe ./path/to/rom.gb";
        return Result::Err(error_msg.to_string());
    } else {
        path = Path::new(&args[1]);
    }

    run(path);

    return Result::Ok(());
}
