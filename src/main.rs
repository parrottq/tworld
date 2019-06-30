
use std::fs::File;

use std::mem;

use std::io;
use std::io::{Cursor, Read};
extern crate byteorder;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
// TODO: This package is not needed

struct Header {
    release: i32,
    magic: [u8; 7],
    filetype: u8,
    revision: u32,
    is_favorite: u64,
    pointers: Vec<usize>,
}

impl Header {
    fn new(mut file: &File) -> Header {
        let mut release_raw = [0; 4];
        file.read(&mut release_raw).unwrap();
        let mut release_cur = Cursor::new(release_raw);

        let mut magic_raw = [0; 7];
        file.read(&mut magic_raw).unwrap();

        let mut filetype_raw = [0; 1];
        file.read(&mut filetype_raw).unwrap();

        let mut revision_raw = [0; 4];
        file.read(&mut revision_raw).unwrap();

        let mut fav_raw = [0; 8];
        file.read(&mut fav_raw).unwrap();

        let mut pointers: Vec<usize> = Vec::new();
        let mut point_raw = [0; 2];
        file.read(&mut point_raw).unwrap();

        let point_count = Cursor::new(point_raw).read_u16::<LittleEndian>().unwrap();

        for _ in 0..point_count {
            let mut value = [0; 4];
            file.read(&mut value).unwrap();
            //println!("{:?}", value);

            pointers.push(Cursor::new(value).read_u32::<LittleEndian>().unwrap() as usize);
        }


        Header {
            release: release_cur.read_i32::<LittleEndian>().unwrap(),
            magic: magic_raw,
            filetype: filetype_raw[0],
            revision: Cursor::new(revision_raw)
                .read_u32::<LittleEndian>()
                .unwrap(),
            is_favorite: Cursor::new(fav_raw).read_u64::<LittleEndian>().unwrap(),
            pointers,
        }
    }

    fn to_string(&self) -> String {
        // TODO: Find a way to do this in rust
        format!(
            "{}, {:?}, {}, {}, {}, {:?}",
            self.release, self.magic, self.filetype, self.revision, self.is_favorite, self.pointers
        )
    }
}

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
struct RawTile {
    data: u64,
    important_x: u16,
    important_y: u16,
}

impl RawTile {

    const fn is_tile_important(tile_id: u16) -> bool {
        [
            0x38,
            0xFC,
            0x3F,
            0xBD,
            0x1E,
            0x04,
            0x84,
            0x20,
            0x80,
            0xE7,
            0xFE,
            0xFF,
            0xFF,
            0x47,
            0x06,
            0x60,
            0xF3,
            0xEF,
            0x21,
            0x00,
            0x20,
            0x78,
            0x04,
            0x0F,
            0x00,
            0x82,
            0x96,
            0x1F,
            0x98,
            0xFA,
            0xFF,
            0x40,
            0x00,
            0xE0,
            0xF8,
            0xEF,
            0xFF,
            0xFF,
            0x7F,
            0xF4,
            0x19,
            0xC0,
            0x0E,
            0x20,
            0xDC,
            0x1F,
            0xF0,
            0x17,
            0xFC,
            0x0F,
            0x60,
            0x7C,
            0b0110_0000,
            0b0000_0010,
            0b0000_0000,
            0b0001_1011,
            0b1111_0000,
            0b1110_1111,
            0b0001_1111,
            // Last 3 bits of last byte unused
        ][(tile_id / 8) as usize]
            >> (tile_id % 8)
            & 0b1
            == 0b1
    }

    fn print(&self) {
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

    fn new() -> RawTile {
        RawTile {
            data: 0,
            important_x: 0,
            important_y: 0,
        }
    }

    fn set_tile_id(&mut self, tile_id: u16) {
        if tile_id > 2u16.pow(9) - 1 {
            panic!("Value bigger than 511 (2^9-1)")
        }

        self.data &= !(2u64.pow(9) - 1 << 0);
        self.data |= (tile_id as u64) << 0;
    }

    fn get_tile_id(&self) -> u16 {
        (self.data & 2u64.pow(9) - 1) as u16
    }

    fn set_fluid_amount(&mut self, amount: u8) {
        self.data &= !(2u64.pow(8) - 1 << 9);
        self.data |= (amount as u64) << 9;
    }

    fn get_fluid_amount(&self) -> u8 {
        (self.data >> 9 & 2u64.pow(8) - 1) as u8
    }

    fn set_wall_id(&mut self, wall_id: u8) {
        self.data &= !(2u64.pow(8) - 1 << 17);
        self.data |= (wall_id as u64) << 17;
    }

    fn get_wall_id(&self) -> u8 {
        (self.data >> 17 & 2u64.pow(8) - 1) as u8
    }

    fn set_red_wiring(&mut self, wired: bool) {
        self.data &= !(0b1 << 25);
        self.data |= (wired as u64) << 25;
    }

    fn get_red_wiring(&self) -> u8 {
        (self.data >> 25 & 0b1) as u8
    }

    fn set_green_wiring(&mut self, wired: bool) {
        self.data &= !(0b1 << 26);
        self.data |= (wired as u64) << 26;
    }

    fn get_green_wiring(&self) -> u8 {
        (self.data >> 26 & 0b1) as u8
    }

    fn set_blue_wiring(&mut self, wired: bool) {
        self.data &= !(0b1 << 27);
        self.data |= (wired as u64) << 27;
    }

    fn get_blue_wiring(&self) -> u8 {
        (self.data >> 27 & 0b1) as u8
    }

    fn set_fluid_type(&mut self, fluid_id: u8) {
        if fluid_id > 2u8.pow(2) - 1 {
            panic!("Value bigger than 3 (2^2-1)")
        }

        self.data &= !(2u64.pow(2) - 1 << 28);
        self.data |= (fluid_id as u64) << 28;
    }

    fn get_fluid_type(&self) -> u8 {
        (self.data >> 28 & 2u64.pow(2) - 1) as u8
    }

    fn set_tile_alter(&mut self, alter: u8) {
        if alter > 2u8.pow(3) - 1 {
            panic!("Value bigger than 7 (3^2-1)")
        }

        self.data &= !(2u64.pow(3) - 1 << 30);
        self.data |= (alter as u64) << 30;
    }

    fn get_tile_alter(&self) -> u8 {
        (self.data >> 30 & 2u64.pow(4) - 1) as u8
    }

    fn set_actuator(&mut self, actuator: bool) {
        self.data &= !(0b1 << 33);
        self.data |= (actuator as u64) << 33;
    }

    fn get_actuator(&self) -> u8 {
        (self.data >> 33 & 0b1) as u8
    }

    fn set_actuator_enabled(&mut self, actuator: bool) {
        self.data &= !(0b1 << 34);
        self.data |= (actuator as u64) << 34;
    }

    fn get_actuator_enabled(&self) -> u8 {
        (self.data >> 34 & 0b1) as u8
    }

    fn set_tile_paint(&mut self, paint: u8) {
        if paint > 2u8.pow(5) - 1 {
            panic!("Value bigger than 31 (2^5-1)")
        }

        self.data &= !(2u64.pow(5) - 1 << 35);
        self.data |= (paint as u64) << 35;
    }

    fn get_tile_paint(&self) -> u8 {
        (self.data >> 35 & 2u64.pow(5) - 1) as u8
    }

    fn set_wall_paint(&mut self, paint: u8) {
        if paint > 2u8.pow(5) - 1 {
            panic!("Value bigger than 31 (2^5-1)")
        }

        self.data &= !(2u64.pow(5) - 1 << 40);
        self.data |= (paint as u64) << 40;
    }

    fn get_wall_paint(&self) -> u8 {
        (self.data >> 40 & 2u64.pow(5) - 1) as u8
    }

    fn set_important(&mut self, x: u16, y: u16) {
        self.important_x = x;
        self.important_y = y;
    }

    fn set_important_bytes(&mut self, x_1: u8, x_2: u8, y_1: u8, y_2: u8) {
        self.set_important(
            x_1 as u16 | (x_2 as u16) << 8,
            y_1 as u16 | (y_2 as u16) << 8,
        );
    }

}

fn parse_tile(data: &Vec<u8>, pos: usize) -> (u16, RawTile, u8) {
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
            tile.set_tile_id(
                Cursor::new([data[pos + offset], data[pos + offset + 1]])
                    .read_u16::<LittleEndian>()
                    .unwrap(),
            );

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
            repetitions = Cursor::new([data[pos + offset], data[pos + offset + 1]])
                .read_u16::<LittleEndian>()
                .unwrap();

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
fn populate_tiles(
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

enum Item {
    None,
    Normal(u16, u32),
    Buffed(u16, u32, u8),
}

impl Item {
    fn print(&self) {
        match self {
            Item::None => println!("No Item"),

            Item::Normal(amount, item_id) => {
                println!("{}(id): {}(amount)", item_id, amount);
            }

            Item::Buffed(amount, item_id, buff) => {
                println!("{}(id): {}(amount) with {}(buff)", item_id, amount, buff);
            }
        }
    }
    fn from_buffer(file_buffer: &Vec<u8>, item_pos: usize) -> Item {

        let amount = file_buffer[item_pos] as u16 | ((file_buffer[item_pos + 1] as u16) << 8);
        if amount == 0 {
            return Item::None;
        }

        let buff = file_buffer[item_pos + 6];

        let item_id = file_buffer[item_pos + 2] as u32
            | ((file_buffer[item_pos + 3] as u32) << 8)
            | ((file_buffer[item_pos + 4] as u32) << 16)
            | ((file_buffer[item_pos + 5] as u32) << 24);

        if buff == 0 {
            return Item::Normal(amount, item_id);
        }

        Item::Buffed(amount, item_id, buff)
    }
}

fn parse_consecutive_items(
    buffer: &Vec<u8>,
    items_start: usize,
    item_count: usize,
) -> (Vec<Item>, usize) {
    let mut items: Vec<Item> = Vec::with_capacity(item_count);
    let mut item_start = items_start;
    let mut item_count = item_count;

    while item_count > 0 {
        let item = Item::from_buffer(&buffer, item_start);

        item_start += match item {
            Item::None => 2,
            Item::Normal(_, _) | Item::Buffed(_, _, _) => 7,
        };

        item_count -= 1;
        items.push(item);
    }

    (items, item_start - items_start)

}

fn parse_chest_items(buffer: &Vec<u8>, items_start: usize) -> (Vec<Item>, usize) {
    parse_consecutive_items(&buffer, items_start, 40)
}

struct Chest {
    name: String,
    x: usize,
    y: usize,
    items: Vec<Item>,
    original_size: usize,
}

impl Chest {
    fn print(&self) {
        if self.name.is_empty() {
            println!(
                "Chest @ ({}, {}), size {}:",
                self.x, self.y, self.original_size
            );
        } else {
            println!(
                "'{}' @ ({}, {}), size {}:",
                self.name, self.x, self.y, self.original_size
            );
        }

        for item in self.items.iter() {
            print!("\t");
            item.print();
        }
    }

    fn from_buffer(buffer: &Vec<u8>, chest_start: usize) -> Chest {
        let name_size = buffer[chest_start + 8] as usize;

        let mut name = String::new();

        for letter_offset in (chest_start + 8 + 1)..(chest_start + 8 + 1 + name_size) {
            name.push(buffer[letter_offset] as char);
        }
        let (items, items_size) = parse_chest_items(
            &buffer,
            // .. + size of the string + string size byte + position bytes
            chest_start + name_size + 1 + 8,
        );

        Chest {
            name,
            x: (buffer[chest_start + 0] as u32
                | ((buffer[chest_start + 1] as u32) << 8)
                | ((buffer[chest_start + 2] as u32) << 16)
                | ((buffer[chest_start + 3] as u32) << 24)) as usize,

            y: (buffer[chest_start + 4] as u32
                | ((buffer[chest_start + 5] as u32) << 8)
                | ((buffer[chest_start + 6] as u32) << 16)
                | ((buffer[chest_start + 7] as u32) << 24)) as usize,

            items,
            original_size: 8 + 1 + name_size + items_size,
        }
    }
}

// TODO: Take the header instead?
fn populate_chests(buffer: &Vec<u8>, chest_start: usize) -> Vec<Chest> {
    let mut chest_count = buffer[chest_start] as u16 | (buffer[chest_start + 1] as u16) << 8;

    // I think these two bytes are redudante but not completely sure
    let capacity = (buffer[chest_start + 2] as u16 | (buffer[chest_start + 3] as u16) << 8);
    if capacity != 40 {
        panic!("Chest capacity should always be 40 but was {}. Handling other sizes is not implemented", capacity);
    }

    let mut chests = Vec::with_capacity(chest_count as usize);
    let mut pos = chest_start + 4;

    while chest_count > 0 {
        let chest = Chest::from_buffer(&buffer, pos);
        pos += chest.original_size;

        chests.push(chest);

        chest_count -= 1;
    }

    chests
}


fn main() -> io::Result<()> {
    let mut file = File::open("test1.wld")?;

    let header = Header::new(&mut file);

    println!("{}", header.to_string());

    //println!("{}", mem::size_of::<RawTile>());

    let mut file = File::open("test1.wld")?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    //println!("{}\n", buffer[header.pointers[1]]);


    /*parse_tile(&buffer, header.pointers[1] as usize + 0)
    .1
    .print();*/

    /*let mut tile = RawTile::new();

    println!("{:b}", !(0u64) >> (64 - 45));

    //tile.set_tile_id(511);
    tile.set_fluid_amount(255);
    //tile.set_wall_id(255);
    tile.set_red_wiring(true);
    //tile.set_green_wiring(true);
    tile.set_blue_wiring(true);
    //tile.set_fluid_type(3);
    tile.set_tile_alter(7);
    //tile.set_actuator(true);
    tile.set_actuator_enabled(true);
    //tile.set_tile_paint(31);
    tile.set_wall_paint(31);


    println!("{:b}", tile.data);

    tile.data = 0;

    tile.set_tile_id(511);
    //tile.set_fluid_amount(255);
    tile.set_wall_id(255);
    //tile.set_red_wiring(true);
    tile.set_green_wiring(true);
    //tile.set_blue_wiring(true);
    tile.set_fluid_type(3);
    //tile.set_tile_alter(7);
    tile.set_actuator(true);
    //tile.set_actuator_enabled(true);
    tile.set_tile_paint(31);
    tile.set_wall_paint(16);
    //println!("{:b}\n{}", tile.data, tile.get_tile_id());

    //tile.print();*/

    //println!("Sized: {}", mem::size_of::<Chest>());

    let mut tile_data: Vec<RawTile> = Vec::with_capacity(4200 * 1200);

    //println!("\n\nPopulation:\n");
    //populate_tiles(&mut tile_data, &buffer, header.pointers[1], 4200 * 1200);


    //Chest::from_buffer(&buffer, 2634477).print();
    //Chest::from_buffer(&buffer, 2634733).print();
    /*for e in (2634470 as usize)..(2634500 as usize) {
    println!("{}: {}", e, buffer[e]);
    }*/

    for chest in populate_chests(&buffer, header.pointers[2]) {
        chest.print();
    }

    Ok(())
}
