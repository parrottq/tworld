
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
#[derive(Clone, Copy)]
pub struct RawTile {
    data: u64,
    important_x: u16,
    important_y: u16,
}

impl RawTile {

    pub const fn is_tile_important(tile_id: u16) -> bool {
        [
            0x38, 0xFC, 0x3F, 0xBD, 0x1E, 0x04, 0x84, 0x20, 0x80, 0xE7, 0xFE, 0xFF, 0xFF, 0x47,
            0x06, 0x60, 0xF3, 0xEF, 0x21, 0x00, 0x20, 0x78, 0x04, 0x0F, 0x00, 0x82, 0x96, 0x1F,
            0x98, 0xFA, 0xFF, 0x40, 0x00, 0xE0, 0xF8, 0xEF, 0xFF, 0xFF, 0x7F, 0xF4, 0x19, 0xC0,
            0x0E, 0x20, 0xDC, 0x1F, 0xF0, 0x17, 0xFC, 0x0F, 0x60, 0x7C, 0x98, 0x2B, 0xF8, 0x3F,
            0xF0, 0xE3, 0x3F,
        ][(tile_id / 8) as usize]
            >> (tile_id % 8)
            & 0b1
            == 0b1
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
            self.get_wall_paint()
        )

    }

    pub fn new() -> RawTile {
        RawTile {
            data: 0,
            important_x: 0,
            important_y: 0,
        }
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

    pub fn set_important_bytes(&mut self, x_1: u8, x_2: u8, y_1: u8, y_2: u8) {
        self.set_important(
            x_1 as u16 | (x_2 as u16) << 8,
            y_1 as u16 | (y_2 as u16) << 8,
        );
    }

}

pub fn parse_tile(data: &Vec<u8>, pos: usize) -> (u16, RawTile, u8) {
    let mut tile = RawTile::new();
    let mut offset = 1;
    let mut flag_level = 1;


    if (data[pos] >> 0) & 0b1 == 0b1 {
        //println!("Flag2");
        flag_level = 2;
        offset += 1;

        tile.set_red_wiring((data[pos + 1] >> 1) & 0b1 == 0b1);
        tile.set_blue_wiring((data[pos + 1] >> 2) & 0b1 == 0b1);
        tile.set_green_wiring((data[pos + 1] >> 3) & 0b1 == 0b1);

        // TODO: alt-slope, actuator; enabled

        tile.set_tile_alter((data[pos + 1] >> 4) & 0b111);

        if (data[pos + 1] >> 0) & 0b1 == 0b1 {
            //println!("Flag3");
            flag_level = 3;
            offset += 1;

            tile.set_actuator((data[pos + 2] >> 1) & 0b1 == 0b1);
            tile.set_actuator_enabled((data[pos + 2] >> 2) & 0b1 == 0b1);
        }
    }

    if (data[pos] >> 1) & 0b1 == 0b1 {
        //println!("Tile present, offset {}", pos + offset);

        if (data[pos] >> 5) & 0b1 == 0b1 {
            // u16 tile
            //println!("u16 tile");
            tile.set_tile_id(data[pos + offset] as u16 | ((data[pos + offset + 1] as u16) << 8));

            offset += 2;
        } else {
            // u8 tile
            //println!("u8 tile");
            tile.set_tile_id(data[pos + offset] as u16);

            offset += 1;
        }

        if RawTile::is_tile_important(tile.get_tile_id()) {
            // Frame X & Y
            tile.set_important_bytes(
                data[pos + offset + 0],
                data[pos + offset + 1],
                data[pos + offset + 2],
                data[pos + offset + 3],
            );
            offset += 4;
            // TODO: Actually store this
        }

    } else {
        //println!("No tile present");
        tile.set_tile_id(511);
    }


    if flag_level == 3 && (data[pos + 2] >> 3) & 0b1 == 0b1 {
        //println!("Tile is painted");

        tile.set_tile_paint(data[pos + offset]);
        offset += 1;
    }

    if (data[pos] >> 2) & 0b1 == 0b1 {
        //println!("Wall is present");

        tile.set_wall_id(data[pos + offset]);
        offset += 1;
    }

    if flag_level == 3 && (data[pos + 2] >> 4) & 0b1 == 0b1 {
        //println!("{}: Wall is painted", data[pos + offset]);

        tile.set_wall_paint(data[pos + offset]);
        offset += 1;
    }

    if (data[pos] >> 3) & 0b11 != 0b0 {
        //println!("Fluid present");
        tile.set_fluid_type((data[pos] >> 3) & 0b11);

        tile.set_fluid_amount(data[pos + offset]);
        offset += 1;
    }

    let mut repetitions = 0;
    if (data[pos] >> 6) & 0b11 != 0b0 {
        //print!("RLE present: ");

        if (data[pos] >> 7) & 0b1 == 0b1 {
            // u16 RLE
            //println!("u16");
            repetitions = data[pos + offset] as u16 | ((data[pos + offset + 1] as u16) << 8);

            offset += 2;
        } else {
            // u8 RLE
            //println!("u8");
            repetitions = data[pos + offset] as u16;

            offset += 1;
        }

        if (data[pos] >> 6) & 0b11 == 0b10 {
            //println!("Alignment flag!");
        }
    } else {
        //println!("No RLE");
    }

    //println!("Total offset: {}", offset);
    //println!("Repetitions: {}\n", repetitions);
    //tile.print();
    //println!("");

    // TODO: Test this
    for e in [419, 420, 421, 422, 423, 424, 425, 440, 441, 442, 460].iter() {
        // 443
        if tile.get_tile_id() == *e {
            panic!("Found a new logic tile; make sure there is no funny business")
        }
    }

    (repetitions, tile, offset as u8)
}

// TODO: Return the data (tile_data) instead of making the caller create one
// TODO: Take the header instead?
pub fn populate_tiles(
    tile_data: &mut Vec<RawTile>,
    file_buffer: &Vec<u8>,
    tile_start: usize,
    total_tile_count: usize,
) {
    let mut block_count = 0;
    let mut tile_start = tile_start;

    while block_count < total_tile_count {
        //println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        //println!("Parse location: {}", tile_start);
        let (duplicate_num, tile, tile_size) = parse_tile(&file_buffer, tile_start);
        /*println!(
            "Parse result: dup {}, tile_size {}",
            duplicate_num, tile_size
        );

        println!("Block count: {}", block_count);
        println!("X: {}, Y: {}", block_count / 1200, block_count % 1200);
        */
        if (block_count % 1200) == 0 && ((block_count / 1200) + 1) % 120 == 0 {
            //println!("Tile x: {}", block_count / 1200);
        }

        // Populate the blocks
        for _e in 0..(duplicate_num as usize + 1) {
            //tile_data[tile_pos] = tile;
            tile_data.push(tile);
        }

        // the current tile + duplicate tiles added to total count
        block_count += 1 + duplicate_num as usize;

        tile_start += tile_size as usize;

    }

    println!("Tile count: {}", tile_start);
    if block_count != total_tile_count {
        panic!("Block count does not match: {}", block_count);
    }
}