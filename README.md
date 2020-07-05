# RustBoy

A simple GameBoy, and possible GameBoy Color, emulator in Rust.


# Structure of the Program

- ```main.rs``` -> main file of the program;
- ```cpu/mod.rs``` -> The CPU file with the instructions;
- ```cpu/reg.rs``` -> The file with the registers, of the CPU.

# Cpu of GB

The CPU as a clock speed of 4.194304MHz and a vertycal sync of 59.73. This means that the CPU executes (4194304 / 59.73) per second.

4194304 / 59.73 = 70221.0614432 instructions

# References

 - [Collection of game boy test roms](https://github.com/retrio/gb-test-roms).
