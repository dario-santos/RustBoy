# RustBoy

A simple GameBoy, and possible GameBoy Color, emulator in Rust.


# Structure of the Program

- ```main.rs``` -> main file of the program;
- ```cpu/mod.rs``` -> The CPU file with the instructions;
- ```cpu/reg.rs``` -> The file with the registers, of the CPU.

# Cpu of GB

The CPU as a clock speed of 4.194304MHz and a vertycal sync of 59.73. This means that the CPU executes (4194304 / 59.73) per second.

4194304 / 59.73 = 70221.0614432 instructions

# Memory Map


+-------------+
| Inte. sw    |
+-------------+ 0xFFFF 
|             |
|  High Ram   |
|             |
+-------------+ 0xFF80
|             |
|     I/O     |
|             |
+-------------+ 0xFF00
|     ---     |
+-------------+ 0xFEA0
|             |
|     OAM     |
|             |
+-------------+ 0xFE00 
|             |
|  Work Ram   |
|             |
+-------------+ 0xE000 
|             |
|  Work Ram   |
|             |
+-------------+ 0xC000 
|             |
|  Exter. Ram |
| (Cartridge) |
+-------------+ 0xA000
|             |
|    VRAM     |
|             |
+-------------+ 0x8000
|             |
|             |
|     ROM     |
| (Cartridge) |
|             |
|             |
+-------------+ 0x0000


# References

 - [Collection of game boy test roms](https://github.com/retrio/gb-test-roms).
