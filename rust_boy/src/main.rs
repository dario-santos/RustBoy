use std::time::{Duration, SystemTime};
use std::thread::sleep;

mod cpu;

extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
 
pub fn main() 
{
  let mut cnt = 0;
  let mut resto = 0.0;
  let mut begin = SystemTime::now();
  
  let cpu = cpu::Cpu::new();
  


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("RustBoy - 0.1.0", 800, 600)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        cnt += 1;
    

        print!("{}", cnt);

        // Execute emulation loop
        if (cnt + (resto as i32)) >= cfg::INSTRUCTIONS_PER_SECOND
        {
          // Clear screen
          print!("\x1B[2J");
          cpu.debug();
    
          // The decimal part
          
          // Sleep the rest of the second
          
          if let Ok(n) = SystemTime::now().duration_since(begin) 
          {
            println!("Time to 1s: \x1b[0;33m{:?}\x1b[0m, delta: \x1b[0;33m{:?}\x1b[0m", Duration::from_secs(1) - n, n);
            sleep(Duration::from_secs(1) - n);
          }
          cnt = 0;
          resto = ((resto as i32) as f64 - resto) + cfg::INSTRUCTIONS_FRAC_PER_SECOND;
          
          begin = SystemTime::now();
        }
        
    canvas.present();
    }
}


mod cfg{
  pub const INSTRUCTIONS_PER_SECOND : i32 = 70221;
  pub const INSTRUCTIONS_FRAC_PER_SECOND : f64 = 0.0614432;
}

