use std::time::{Duration, SystemTime};
use std::thread::sleep;

mod cpu;

fn main()
{
  // Execute 70221.0614432 per second
  let mut cnt = 0;
  let mut resto = 0.0;
  let mut begin = SystemTime::now();
  
  let cpu = cpu::Cpu::new();
  
  loop{
    cnt += 1;
    
    // Execute emulation loop
    
    if (cnt + (resto as i32)) >= 70221 
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
      resto = ((resto as i32) as f64 - resto) + 0.0614432;
      
      begin = SystemTime::now();
    }
  }
}
