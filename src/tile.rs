use crate::metadata;
use crate::parsing::PositionedBuffer;
use crate::writing::PrimitiveWriting;
// Data
// 0-8: tile id
// 9-16: liquid amount
// 17-24: wall id
// 25-27: wiring
// 28-29: liquid type
// 30-32: tile alterations
// 33: actuator present
// 34: actuator selected
// 35-39: tile paint
// 40-44: wall paint
#[derive(Clone, Copy, PartialEq)]
pub struct RawTile {
    data: u64,
    pub important_x: u16,
    pub important_y: u16,
}

pub static IMPORTANTS_BYTES: [u8; 59] = [
    0x38, 0xFC, 0x3F, 0xBD, 0x1E, 0x04, 0x84, 0x20, 0x80, 0xE7, 0xFE, 0xFF, 0xFF, 0x47, 0x06, 0x60,
    0xF3, 0xEF, 0x21, 0x00, 0x20, 0x78, 0x04, 0x0F, 0x00, 0x82, 0x96, 0x1F, 0x98, 0xFA, 0xFF, 0x40,
    0x00, 0xE0, 0xF8, 0xEF, 0xFF, 0xFF, 0x7F, 0xF4, 0x19, 0xC0, 0x0E, 0x20, 0xDC, 0x1F, 0xF0, 0x17,
    0xFC, 0x0F, 0x60, 0x7C, 0x98, 0x2B, 0xF8, 0x3F, 0xF0, 0xE3, 0x3F,
];

impl RawTile {
    pub fn is_tile_important(tile_id: u16) -> bool {
        IMPORTANTS_BYTES[(tile_id / 8) as usize] >> (tile_id % 8) & 0b1 == 0b1
    }

    pub fn print(&self) {
        println!("{}: Tile ID,\n{}: Liquid amount(x/255),\n{}: Wall ID,\n{}{}{}: Wiring(rgb),\n{}: Liquid Type\n{}: Tile alteration,\n{};{}: Actuator; active,\n{}: Tile paint,\n{}: Wall paint",
            self.get_tile_id(),
            self.get_fluid_amount(),
            self.get_wall_id(),
            self.get_red_wiring(),
            self.get_green_wiring(),
            self.get_blue_wiring(),
            self.get_fluid_type(),
            self.get_tile_alter(),
            self.get_actuator(),
            self.get_actuator_enabled(),
            self.get_tile_paint(),
            self.get_wall_paint(),
        )
    }

    pub fn new() -> RawTile {
        let mut e = RawTile {
            data: 0,
            important_x: 0,
            important_y: 0,
        };
        e.set_tile_id(511);
        e
    }

    pub fn set_tile_id(&mut self, tile_id: u16) {
        if tile_id > 2u16.pow(9) - 1 {
            panic!("Value bigger than 511 (2^9-1)")
        }

        self.data &= !(2u64.pow(9) - 1 << 0);
        self.data |= (tile_id as u64) << 0;
    }

    pub fn get_tile_id(&self) -> u16 {
        (self.data & 2u64.pow(9) - 1) as u16
    }

    pub fn set_fluid_amount(&mut self, amount: u8) {
        self.data &= !(2u64.pow(8) - 1 << 9);
        self.data |= (amount as u64) << 9;
    }

    pub fn get_fluid_amount(&self) -> u8 {
        (self.data >> 9 & 2u64.pow(8) - 1) as u8
    }

    pub fn set_wall_id(&mut self, wall_id: u8) {
        self.data &= !(2u64.pow(8) - 1 << 17);
        self.data |= (wall_id as u64) << 17;
    }

    pub fn get_wall_id(&self) -> u8 {
        (self.data >> 17 & 2u64.pow(8) - 1) as u8
    }

    pub fn set_red_wiring(&mut self, wired: bool) {
        self.data &= !(0b1 << 25);
        self.data |= (wired as u64) << 25;
    }

    pub fn get_red_wiring(&self) -> u8 {
        (self.data >> 25 & 0b1) as u8
    }

    pub fn set_green_wiring(&mut self, wired: bool) {
        self.data &= !(0b1 << 26);
        self.data |= (wired as u64) << 26;
    }

    pub fn get_green_wiring(&self) -> u8 {
        (self.data >> 26 & 0b1) as u8
    }

    pub fn set_blue_wiring(&mut self, wired: bool) {
        self.data &= !(0b1 << 27);
        self.data |= (wired as u64) << 27;
    }

    pub fn get_blue_wiring(&self) -> u8 {
        (self.data >> 27 & 0b1) as u8
    }

    pub fn set_fluid_type(&mut self, fluid_id: u8) {
        if fluid_id > 2u8.pow(2) - 1 {
            panic!("Value bigger than 3 (2^2-1)")
        }

        self.data &= !(2u64.pow(2) - 1 << 28);
        self.data |= (fluid_id as u64) << 28;
    }

    pub fn get_fluid_type(&self) -> u8 {
        (self.data >> 28 & 2u64.pow(2) - 1) as u8
    }

    pub fn set_tile_alter(&mut self, alter: u8) {
        if alter > 2u8.pow(3) - 1 {
            panic!("Value bigger than 7 (3^2-1)")
        }

        self.data &= !(2u64.pow(3) - 1 << 30);
        self.data |= (alter as u64) << 30;
    }

    pub fn get_tile_alter(&self) -> u8 {
        (self.data >> 30 & 2u64.pow(4) - 1) as u8
    }

    pub fn set_actuator(&mut self, actuator: bool) {
        self.data &= !(0b1 << 33);
        self.data |= (actuator as u64) << 33;
    }

    pub fn get_actuator(&self) -> u8 {
        (self.data >> 33 & 0b1) as u8
    }

    pub fn set_actuator_enabled(&mut self, actuator: bool) {
        self.data &= !(0b1 << 34);
        self.data |= (actuator as u64) << 34;
    }

    pub fn get_actuator_enabled(&self) -> u8 {
        (self.data >> 34 & 0b1) as u8
    }

    pub fn set_tile_paint(&mut self, paint: u8) {
        if paint > 2u8.pow(5) - 1 {
            panic!("Value bigger than 31 (2^5-1)")
        }

        self.data &= !(2u64.pow(5) - 1 << 35);
        self.data |= (paint as u64) << 35;
    }

    pub fn get_tile_paint(&self) -> u8 {
        (self.data >> 35 & 2u64.pow(5) - 1) as u8
    }

    pub fn set_wall_paint(&mut self, paint: u8) {
        if paint > 2u8.pow(5) - 1 {
            panic!("Value bigger than 31 (2^5-1)")
        }

        self.data &= !(2u64.pow(5) - 1 << 40);
        self.data |= (paint as u64) << 40;
    }

    pub fn get_wall_paint(&self) -> u8 {
        (self.data >> 40 & 2u64.pow(5) - 1) as u8
    }

    pub fn set_important(&mut self, x: u16, y: u16) {
        self.important_x = x;
        self.important_y = y;
    }

    pub fn write(&self, file: &mut std::fs::File, repetitions: u16) {
        let mut flag3 = 0;
        flag3 |= self.get_actuator() << 1;
        flag3 |= self.get_actuator_enabled() << 2;
        flag3 |= if self.get_tile_paint() != 0 {
            1 << 3
        } else {
            0
        };
        flag3 |= if self.get_wall_paint() != 0 {
            1 << 4
        } else {
            0
        };

        let mut flag2 = 0;
        flag2 |= if flag3 != 0 { 1 } else { 0 };
        flag2 |= self.get_red_wiring() << 1;
        flag2 |= self.get_blue_wiring() << 2;
        flag2 |= self.get_green_wiring() << 3;
        flag2 |= self.get_tile_alter() << 4;

        let mut flag1 = 0;
        flag1 |= if flag2 != 0 { 1 } else { 0 };
        flag1 |= if self.get_tile_id() != 511 { 1 << 1 } else { 0 };
        flag1 |= if self.get_wall_id() != 0 { 1 << 2 } else { 0 };
        flag1 |= self.get_fluid_type() << 3;
        flag1 |= if self.get_tile_id() != 511 && self.get_tile_id() > ((1 << 8) - 1) {
            //println!("ID 16");
            1 << 5
        } else {
            0
        };
        flag1 |= if repetitions > ((1 << 8) - 1) {
            //println!("RLE 16");
            1 << 7
        } else {
            flag1 |= if repetitions > 0 { 1 << 6 } else { 0 };
            0
        };
        //println!("{}:1\n{}:2\n{}:3\n", flag1, flag2, flag3);

        file.write_u8(flag1);
        if flag2 != 0 {
            file.write_u8(flag2);

            if flag3 != 0 {
                file.write_u8(flag3);
            }
        }

        // Tile ID
        let tile_id = self.get_tile_id();
        if tile_id != 511 {
            if tile_id > ((1 << 8) - 1) {
                // u16 tile
                file.write_u16(tile_id);
            } else {
                // u8 tile
                file.write_u8(tile_id as u8);
            }

            // Important
            if RawTile::is_tile_important(tile_id) {
                file.write_u16(self.important_x);
                file.write_u16(self.important_y);
            }

            // Tile Paint
            let tile_paint = self.get_tile_paint();
            if tile_paint != 0 {
                file.write_u8(tile_paint);
            }
        }

        // Wall ID
        let wall_id = self.get_wall_id();
        if wall_id != 0 {
            file.write_u8(wall_id);
        }

        // Wall Paint
        let wall_paint = self.get_wall_paint();
        if wall_paint != 0 {
            file.write_u8(wall_paint);
        }

        // Fluid amount
        let fluid_amount = self.get_fluid_amount();
        if fluid_amount != 0 {
            file.write_u8(fluid_amount);
        }

        if repetitions > 0 {
            if repetitions > ((1 << 8) - 1) {
                // u16
                file.write_u16(repetitions);
            } else {
                // u8
                file.write_u8(repetitions as u8);
            }
        }
    }
}

// TODO: Move into RawTile impl
pub fn parse_tile(pbuffer: &mut PositionedBuffer) -> (u16, RawTile) {
    let mut tile = RawTile::new();

    let flag1 = pbuffer.read_u8();
    let mut flag3 = 0;
    let mut flag_level = 1;

    // Flag parsing
    if (flag1 >> 0) & 0b1 == 0b1 {
        //println!("Flag2");
        let flag2 = pbuffer.read_u8();
        flag_level = 2;

        tile.set_red_wiring((flag2 >> 1) & 0b1 == 0b1);
        tile.set_blue_wiring((flag2 >> 2) & 0b1 == 0b1);
        tile.set_green_wiring((flag2 >> 3) & 0b1 == 0b1);

        tile.set_tile_alter((flag2 >> 4) & 0b111);

        if (flag2 >> 0) & 0b1 == 0b1 {
            //println!("Flag3");
            flag3 = pbuffer.read_u8();
            flag_level = 3;

            tile.set_actuator((flag3 >> 1) & 0b1 == 0b1);
            tile.set_actuator_enabled((flag3 >> 2) & 0b1 == 0b1);
        }
    }

    // Tile ID parsing
    if (flag1 >> 1) & 0b1 == 0b1 {
        //println!("Tile present, offset {}", pos + offset);

        if (flag1 >> 5) & 0b1 == 0b1 {
            // u16 tile
            //println!("u16 tile");
            tile.set_tile_id(pbuffer.read_u16());
        } else {
            // u8 tile
            //println!("u8 tile");
            tile.set_tile_id(pbuffer.read_u8() as u16);
        }

        if RawTile::is_tile_important(tile.get_tile_id()) {
            // Frame X & Y
            tile.set_important(pbuffer.read_u16(), pbuffer.read_u16());
        }
    } else {
        //println!("No tile present");
        tile.set_tile_id(511);
    }

    if flag_level == 3 && (flag3 >> 3) & 0b1 == 0b1 {
        //println!("Tile is painted");

        tile.set_tile_paint(pbuffer.read_u8());
    }

    if (flag1 >> 2) & 0b1 == 0b1 {
        //println!("Wall is present");

        tile.set_wall_id(pbuffer.read_u8());
    }

    if flag_level == 3 && (flag3 >> 4) & 0b1 == 0b1 {
        //println!("Wall is painted");

        tile.set_wall_paint(pbuffer.read_u8());
    }

    if (flag1 >> 3) & 0b11 != 0b0 {
        //println!("Fluid present");
        tile.set_fluid_type((flag1 >> 3) & 0b11);

        tile.set_fluid_amount(pbuffer.read_u8());
    }

    // Run Length Encoding parsing
    let mut reps = 0;
    if (flag1 >> 6) & 0b11 != 0b0 {
        //print!("RLE present: ");

        reps = if (flag1 >> 7) & 0b1 == 0b1 {
            // u16 RLE
            //println!("u16");
            pbuffer.read_u16()
        } else {
            // u8 RLE
            //println!("u8");
            pbuffer.read_u8() as u16
        };
    } else {
        //println!("No RLE");
    }

    for e in [419, 420, 421, 422, 423, 424, 425, 440, 441, 442, 460].iter() {
        // 443
        if tile.get_tile_id() == *e {
            panic!("Found a new logic tile; make sure there is no funny business")
        }
    }

    (reps, tile)
}

pub fn populate_tiles(
    pbuffer: &mut PositionedBuffer,
    world_metadata: &metadata::WorldHeader,
) -> Vec<RawTile> {
    let mut tile_count = world_metadata.get_tile_count();

    let mut tiles = Vec::with_capacity(tile_count as usize);

    while tile_count > 0 {
        //println!("Tile_count: {}", tile_count);
        let (repetitions, tile) = parse_tile(pbuffer);

        for _ in 0..(repetitions + 1) {
            tiles.push(tile);
        }
        tile_count -= 1 + repetitions as u32;
    }

    if tile_count != 0 {
        panic!("Block count does not match: {}", tile_count);
    }

    tiles
}

pub fn write_tiles(tiles: &Vec<RawTile>, file: &mut std::fs::File, height: u32) -> usize {
    let mut last_tile = tiles[0];
    let mut count = 0;
    let mut pos = 1;

    while pos < tiles.len() {
        let tile = tiles[pos];

        if tile != last_tile || (pos % height as usize) == 0 {
            last_tile.write(file, count);
            last_tile = tile;
            count = 0;
        } else {
            count += 1;
        }

        pos += 1;
    }
    last_tile.write(file, count);

    file.current_pos()
}
