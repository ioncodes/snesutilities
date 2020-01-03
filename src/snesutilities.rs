use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::str;
use snesutilities::SnesBufferIndex::{SRowSizeIndex, RowTypeIndex, RomMarkupTypeIndex, VideoModeIndex, RowSizeIndex, LicensesIndex};

const SNES_BUFFER_SIZE: usize = 6;

enum SnesBufferIndex {
    RomMarkupTypeIndex = 0,
    RowTypeIndex = 1,
    RowSizeIndex = 2,
    SRowSizeIndex = 3,
    VideoModeIndex = 4,
    LicensesIndex = 5
}

macro_rules! number_to_enum {
    ($number:expr => $enum:ident<$type:ty>{ $($field:ident),+}; $error:expr) => {
        match $number{
            $(_ if $number == $enum::$field as $type => {
                $enum::$field
            })+
            _ => $error
        }
    };
}

#[repr(u8)]
#[derive(Debug)]
pub enum RomMarkupType {
    LoROM = 32,        // 32 // 32704
    HiROM = 33,        // 33 // 65472
    LoROMFastROM = 48, // 48
    HiROMFastROM = 49, // 49
    ExLoROM = 50,      // 50
    ExHiROM = 53,      // 53
    Unknown,
}
#[repr(u8)]
#[derive(Debug, PartialEq)]
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
#[derive(Debug)]
pub struct VideoMode {
    pub country: String,
    pub mode: String,
}
pub struct SnesUtils {
    pub internal_name: String,
    pub rom_makeup_type: RomMarkupType,
    pub rom_type: RomType,
    pub rom_size: u8,
    pub sram_size: u8,
    pub video_mode: VideoMode,
    pub license: String,
}

impl SnesUtils {
    pub fn new(file_name: String) -> SnesUtils {

        let mut buffer = [0; SNES_BUFFER_SIZE]; // create initial buffer
        let mut file = &mut File::open(file_name).unwrap(); // load the file

        read_buffer(&mut file, &mut buffer); // read rom makeup byte

        SnesUtils {
            internal_name:  read_file(file),
            rom_makeup_type: get_rom_makeup_type(buffer),
            rom_type: get_rom_type(buffer),
            rom_size:  buffer[RowSizeIndex],
            sram_size: buffer[SRowSizeIndex],
            video_mode: get_video_mode(buffer),
            license: LICENSES[buffer[LicensesIndex] as usize].to_string(),
        }
    }
}

#[allow(unused_must_use)]
fn read_file(file: &mut File) -> String {
    let mut vec = vec![0u8; 21];
    file.seek(SeekFrom::Start(32704));
    file.read(vec.as_mut_slice()).unwrap();
    let mut is_lo_rom = true;
    for byte in vec.iter() {
        if *byte <= 31 || *byte > 127 {
            is_lo_rom = false;
            break;
        }
    }
    if !is_lo_rom {
        vec = vec![0u8; 21];
        file.seek(SeekFrom::Start(65472)); // it's hirom
        file.read(vec.as_mut_slice()).unwrap();
    }
    return str::from_utf8(&vec).unwrap().to_string();
}

#[allow(unused_must_use)]
fn read_buffer(file: &mut File, buffer: &mut [u8; SNES_BUFFER_SIZE]) {
    file.read(buffer);
}

fn get_rom_type(buffer: [u8; SNES_BUFFER_SIZE]) -> RomType {

    number_to_enum!(buffer[RomTypeIndex] => RomType<u8>{
            ROM,
            ROMRAM,
            ROMSRAM,
            ROMDSP1,
            ROMDSP1RAM,
            ROMDSP1SRAM,
            FX,
            Unknown
        };
        panic!("Cannot convert number to RomType")
    )
}

fn get_rom_makeup_type(buffer: [u8; SNES_BUFFER_SIZE]) -> RomMarkupType {

    number_to_enum!(buffer[RomMarkupTypeIndex] => RomMarkupType<u8>{
            LoROM,
            HiROM,
            LoROMFastROM,
            HiROMFastROM,
            ExLoROM,
            ExHiROM,
            Unknown
        };
        panic!("Cannot convert number to RomMarkupType")
    )
}

fn get_video_mode(buffer: [u8; SNES_BUFFER_SIZE]) -> VideoMode {

    return match buffer[VideoModeIndex]{
        0 => VideoMode {
            country: "Japan".to_string(),
            mode: "NTSC".to_string(),
        },
        1 => VideoMode {
            country: "USA".to_string(),
            mode: "NTSC".to_string(),
        },
        2 => VideoMode {
            country: "Europe".to_string(),
            mode: "PAL".to_string(),
        },
        3 => VideoMode {
            country: "Sweden".to_string(),
            mode: "PAL".to_string(),
        },
        4 => VideoMode {
            country: "Finland".to_string(),
            mode: "PAL".to_string(),
        },
        5 => VideoMode {
            country: "Denmark".to_string(),
            mode: "PAL".to_string(),
        },
        6 => VideoMode {
            country: "France".to_string(),
            mode: "PAL".to_string(),
        },
        7 => VideoMode {
            country: "Holland".to_string(),
            mode: "PAL".to_string(),
        },
        8 => VideoMode {
            country: "Spain".to_string(),
            mode: "PAL".to_string(),
        },
        9 => VideoMode {
            country: "Germany".to_string(),
            mode: "PAL".to_string(),
        },
        10 => VideoMode {
            country: "Italy".to_string(),
            mode: "PAL".to_string(),
        },
        11 => VideoMode {
            country: "China".to_string(),
            mode: "PAL".to_string(),
        },
        12 => VideoMode {
            country: "Indonesia".to_string(),
            mode: "PAL".to_string(),
        },
        13 => VideoMode {
            country: "Korea".to_string(),
            mode: "PAL".to_string(),
        },
        _ => VideoMode {
            country: "Unknown".to_string(),
            mode: "Unknown".to_string(),
        },
    };
}

const LICENSES: &'static [&'static str] = &[
    "Invalid",
    "Nintendo",
    "",
    "",
    "",
    "",
    "Zamuse",
    "",
    "",
    "",
    "Capcom",
    "HOT B",
    "Jaleco",
    "STORM (Sales Curve) (1)",
    "",
    "",
    "",
    "",
    "Mebio Software",
    "",
    "",
    "",
    "Gremlin Graphics",
    "",
    "",
    "",
    "COBRA Team",
    "Human/Field",
    "",
    "",
    "Hudson Soft",
    "",
    "",
    "Yanoman",
    "",
    "",
    "Tecmo (1)",
    "",
    "",
    "Forum",
    "Park Place Productions / VIRGIN",
    "",
    "",
    "Tokai Engeneering (SUNSOFT?)",
    "POW",
    "Loriciel / Micro World",
    "",
    "",
    "",
    "Enix",
    "",
    "",
    "Kemco (1)",
    "Seta Co.,Ltd.",
    "",
    "",
    "",
    "",
    "Visit Co.,Ltd.",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "HECT",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "Loriciel",
    "",
    "",
    "",
    "Seika Corp.",
    "UBI Soft",
    "",
    "",
    "",
    "",
    "",
    "",
    "Spectrum Holobyte",
    "",
    "",
    "Irem",
    "",
    "",
    "Raya Systems/Sculptured Software",
    "Renovation Pruducts",
    "Malibu Games (T*HQ Inc.) / Black Pearl",
    "",
    "",
    "U.S. Gold",
    "Absolute Entertainment",
    "Acclaim",
    "Activision",
    "American Sammy",
    "GameTek",
    "Hi Tech",
    "LJN Toys",
    "",
    "",
    "",
    "",
    "Mindscape",
    "",
    "",
    "",
    "Technos Japan Corp. (Tradewest)",
    "",
    "",
    "American Softworks Corp.",
    "Titus",
    "Virgin Games",
    "Maxis",
    "",
    "",
    "",
    "",
    "",
    "Ocean",
    "",
    "",
    "Electronic Arts",
    "",
    "",
    "Laser Beam",
    "",
    "",
    "",
    "Elite",
    "Electro Brain",
    "Infogrames",
    "Interplay",
    "LucasArts",
    "Sculptured Soft",
    "",
    "",
    "STORM (Sales Curve) (2)",
    "",
    "",
    "",
    "THQ Software",
    "Accolade Inc.",
    "Triffix Entertainment",
    "",
    "",
    "Microprose",
    "",
    "",
    "",
    "Kemco (2)",
    "",
    "",
    "",
    "Namcot/Namco Ltd. (1)",
    "",
    "",
    "Koei/Koei! (second license?)",
    "",
    "",
    "Tokuma Shoten Intermedia",
    "",
    "",
    "DATAM-Polystar",
    "",
    "",
    "",
    "Bullet-Proof Software",
    "Vic Tokai",
    "",
    "",
    "",
    "I'Max",
    "",
    "",
    "CHUN Soft",
    "Video System Co., Ltd.",
    "BEC",
    "",
    "",
    "",
    "",
    "Kaneco",
    "",
    "",
    "Pack in Video",
    "Nichibutsu",
    "TECMO (2)",
    "Imagineer Co.",
    "",
    "",
    "",
    "",
    "Wolf Team",
    "",
    "",
    "",
    "",
    "Konami",
    "K.Amusement",
    "",
    "",
    "Takara",
    "",
    "",
    "Technos Jap. ????",
    "JVC",
    "",
    "",
    "Toei Animation",
    "Toho",
    "",
    "",
    "Namcot/Namco Ltd. (2)",
    "",
    "",
    "ASCII Co. Activison",
    "BanDai America",
    "",
    "",
    "Enix",
    "",
    "",
    "Halken",
    "",
    "",
    "",
    "",
    "Culture Brain",
    "Sunsoft",
    "Toshiba EMI/System Vision",
    "Sony (Japan) / Imagesoft",
    "",
    "",
    "Sammy",
    "Taito",
    "",
    "",
    "Kemco (3) ????",
    "Square",
    "NHK",
    "Data East",
    "Tonkin House",
    "",
    "",
    "KOEI",
    "",
    "",
    "Konami USA",
    "",
    "",
    "",
    "Meldac/KAZe",
    "PONY CANYON",
    "Sotsu Agency",
    "",
    "",
    "Sofel",
    "Quest Corp.",
    "Sigma",
    "",
    "",
    "",
    "Naxat",
    "",
    "",
    "Capcom Co., Ltd. (2)",
    "Banpresto",
    "",
    "",
    "Hiro",
    "",
    "",
    "NCS",
    "Human Entertainment",
    "Ringler Studios",
    "K.K. DCE / Jaleco",
    "",
    "",
    "Sotsu Agency",
    "",
    "",
    "T&ESoft",
    "EPOCH Co.,Ltd.",
    "",
    "",
    "Athena",
    "Asmik",
    "Natsume",
    "King/A Wave",
    "Atlus",
    "Sony Music",
    "",
    "",
    "Psygnosis / igs",
    "",
    "",
    "",
    "",
    "",
    "Beam Software",
    "Tec Magik",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "Hudson Soft",
];
