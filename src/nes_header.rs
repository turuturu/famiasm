#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NesHeader {
    // Number of 16kB ROM banks.
    pub prg_rom_count: u8,
    // Number of 8kB VROM banks.
    pub chr_rom_count: u8,
    // See https://wiki.nesdev.org/w/index.php/INES
    pub mapper: u8,
    // See https://github.com/camsaul/nesasm/blob/master/ines_header_format.txt
    pub mirror: u8,
    /*
     Number of 8kB RAM banks. For compatibility with the previous
     versions of the .NES format, assume 1x8kB RAM page when this
     byte is zero.
    */
    pub ram_size: u8,
    // PAL or NTSC
    pub is_pal: bool,
}

impl Default for NesHeader {
    fn default() -> Self {
        Self {
            prg_rom_count: 1,
            chr_rom_count: 1,
            mapper: 0,
            mirror: 1,
            ram_size: 0,
            is_pal: false,
        }
    }
}

impl NesHeader {
    pub fn gen_binary(&self) -> [u8; 16] {
        let mut arr: [u8; 16] = Default::default();
        arr[0] = 0x4E; // N
        arr[1] = 0x45; // E
        arr[2] = 0x53; // S
        arr[3] = 0xaA; //
        arr[4] = self.prg_rom_count;
        arr[5] = self.chr_rom_count;
        /*
         bit 0     1 for vertical mirroring, 0 for horizontal mirroring.
         bit 1     1 for battery-backed RAM at $6000-$7FFF.
         bit 2     1 for a 512-byte trainer at $7000-$71FF.
         bit 3     1 for a four-screen VRAM layout.
         bit 4-7   Four lower bits of ROM Mapper Type
        */
        arr[6] = ((self.mapper & 0x0F) << 4) | (self.mirror & 0x0F);
        /*
         bit 0     1 for VS-System cartridges.
         bit 1-3   Reserved, must be zeroes!
         bit 4-7   Four higher bits of ROM Mapper Type.
        */
        arr[7] = (self.mapper & 0xF0) | 0b00000000;
        arr[8] = self.ram_size & 0xFF;
        /*
         bit 0     1 for PAL cartridges, otherwise assume NTSC.
         bit 1-7   Reserved, must be zeroes!
        */
        arr[9] = if self.is_pal { 0x01 } else { 0x00 };
        // 10 ~ 15 reserved
        arr
    }
}
