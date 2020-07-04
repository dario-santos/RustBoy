mod cpu;

fn main()
{
    // Creates the cpu
    let cpu = cpu::Cpu::new();
    cpu.debug();
}
