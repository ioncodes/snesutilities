mod snesutilities;
use snesutilities::SnesUtils;

fn main() {
    let suz = SnesUtils::new("./zelda.sfc".to_string());
    println!("Internal Name: {}", suz.internal_name);
    println!("Rom Type: {:?}", suz.rom_type);
    println!("Rom Makeup Type: {:?}", suz.rom_makeup_type);
    println!("ROM Size: {:?}", suz.rom_size);
    println!("SRAM Size: {:?}", suz.sram_size);
    println!("Video Mode: {:#?}", suz.video_mode);
    println!("License: {:#?}\n", suz.license);
    let sus = SnesUtils::new("./sengoku.sfc".to_string());
    println!("Internal Name: {}", sus.internal_name);
    println!("Rom Type: {:?}", sus.rom_type);
    println!("Rom Makeup Type: {:?}", sus.rom_makeup_type);
    println!("ROM Size: {:?}", sus.rom_size);
    println!("SRAM Size: {:?}", sus.sram_size);
    println!("Video Mode: {:#?}", sus.video_mode);
    println!("License: {:#?}\n", sus.license);
    let suf = SnesUtils::new("./ffv.sfc".to_string());
    println!("Internal Name: {}", suf.internal_name);
    println!("Rom Type: {:?}", suf.rom_type);
    println!("Rom Makeup Type: {:?}", suf.rom_makeup_type);
    println!("ROM Size: {:?}", suf.rom_size);
    println!("SRAM Size: {:?}", suf.sram_size);
    println!("Video Mode: {:#?}", suf.video_mode);
    println!("License: {:#?}\n", suf.license);
}
