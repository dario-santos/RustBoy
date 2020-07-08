// Creates displays
pub mod Display
{
  use sdl2::Sdl;
  use sdl2::video::Window;
  use sdl2::render::Canvas;

  pub fn new(sdl2: &Sdl, witdth: u32, height: u32) -> Canvas<Window> 
  {
    // Get video system of sdl
    let video_subsystem = sdl2.video().unwrap();
    
    // Build window
    let window = video_subsystem.window("Rustboy v 0.1.0", witdth, height).position_centered().build().unwrap();

    // Create a canvas that will be used to draw in the Window
    window.into_canvas().software().build().unwrap()
  }
}
