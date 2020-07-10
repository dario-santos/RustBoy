use crate::memory;

// Creates displays
use sdl2::Sdl;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color as SdlColor;
use sdl2::rect::Point;


pub struct Display{
  screen: Canvas<Window>,
  buffer: Vec<Vec<u8>>,
  
}

impl Display
{

  pub fn new(sdl2: &Sdl, witdth: u32, height: u32) -> Display
  {
    // Get video system of sdl
    let video_subsystem = sdl2.video().unwrap();
    
    // Build window
    let window = video_subsystem.window("Rustboy v 0.1.0", witdth, height).position_centered().build().unwrap();

    // Create a canvas that will be used to draw in the Window
    let canvas = window.into_canvas().software().build().unwrap();

    Display{
      screen: canvas,
      buffer: vec![vec![0x0; witdth as usize]; height as usize]
    }
  }
  
  pub fn draw(&mut self) {
    self.screen.clear();
    
    for y in 0..144 {
      let mut index = y;
      for x in 0..160 {
        
        let color = match index{
          0 => SdlColor::RGB(0xff, 0xff, 0xff),
          1 => SdlColor::RGB(0x00, 0x00, 0x00),
          2 => SdlColor::RGB(0xab, 0xab, 0xab),
          _ => SdlColor::RGB(0x55, 0x55, 0x55),
        };

        index = (index + 1) % 4;

        self.screen.set_draw_color(color);
        self.screen.draw_point(Point::new(x as i32, y as i32)).unwrap();
      }

    }

    self.screen.present();
  }

  pub fn update(&mut self, ram: &mut memory::Memory)
  {
  
    if (ram.get(0xFF40) & 0b1000_0000) >> 7 == 1
    {
      println!("Display ON");
    }
    else
    {
      println!("Display OFF");
    
      return;
    }

    //if (m_ScalineCounter <= 0)
    {
      // Move to next scanline
      let result = match ram.get(0xFF44).checked_add(1){
        Some(v) => v, // NO OVERFLOW
        None    => 0, // OVERFLOW
      };
      
      ram.set(0xFF44, result);

    let current_line: u8 = ram.get(0xFF44);
      //m_ScalineCounter = 456 ;

      
    println!("current scanline: {}", current_line);
    
     
    if current_line > 153 
    {
      ram.set(0xFF44, 0);
    } 
    // draw the current scanline
    else if current_line < 144
    {
      //draw_scanline();
    }
     // we have entered vertical blank period
     //if (currentline == 144)
     //  RequestInterupt(0) ;

   }
}
}
