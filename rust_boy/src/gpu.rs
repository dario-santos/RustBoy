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
}
