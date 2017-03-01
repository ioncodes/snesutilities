# snesutilities
Utilities to gather data out of roms. Written in Rust. It (should) support all types.

# How
Have a look at main.rs:
```rust
use snesutilities::SnesUtils;
let suz = SnesUtils::new("./zelda.sfc".to_string());
println!("Internal Name: {}", suz.internal_name);
println!("Rom Type: {:?}", suz.rom_type);
println!("Rom Makeup Type: {:?}", suz.rom_makeup_type);
println!("ROM Size: {:?}", suz.rom_size);
println!("SRAM Size: {:?}", suz.sram_size);
println!("Video Mode: {:#?}", suz.video_mode);
println!("License: {:#?}\n", suz.license);
```

# Features
These informations can be parsed:
* Internal Name
* Rom Type
* Rom Makeup Type
* ROM Size
* SRAM Size
* Video Mode
* License (Owner)

# Specifications
Rom Type:
```rust
pub enum RomType {
    ROM = 0,
    ROMRAM = 1,
    ROMSRAM = 2,
    ROMDSP1 = 3,
    ROMDSP1RAM = 4,
    ROMDSP1SRAM = 5,
    FX = 6,
    Unknown,
}
```

Rom Makeup Type:
```rust
pub enum RomMakupType {
    LoROM = 32,
    HiROM = 33,
    LoROMFastROM = 48,
    HiROMFastROM = 49,
    ExLoROM = 50,
    ExHiROM = 53,
    Unknown,
}
```

Video Mode:
```rust
pub struct VideoMode {
    pub country: String,
    pub mode: String,
}
```
