
#![allow(dead_code)]
use std::fs::File;

use std::io;
use std::io::{Cursor, Read};

extern crate byteorder;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
// TODO: This package is not needed

extern crate num;
use num::ToPrimitive;

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

    fn print_pointers(&self) {
        println!("{}: Headers\n{}: Tiles\n{}: Chests\n{}: Signs\n{}: NPCs\n{}: Entities\n{}: PressurePlates\n{}: TownManager\n{}: Footer\n{}", 
            self.pointers[0],
            self.pointers[1],
            self.pointers[2],
            self.pointers[3],
            self.pointers[4],
            self.pointers[5],
            self.pointers[6],
            self.pointers[7],
            self.pointers[8],
            self.pointers[9]
        );
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

struct Sign {
    text: String,
    x: usize,
    y: usize,
}

impl Sign {
    fn print(&self) {
        if self.text.len() == 0 {
            println!("Empty Sign @ ({}, {})", self.x, self.y);
        } else {
            println!("Sign @ ({}, {}):", self.x, self.y);
            println!("{}", self.text);
        }
    }

    fn get_size(&self) -> usize {
        1 + 8 + self.text.len()
    }

    fn from_buffer(buffer: &Vec<u8>, sign_start: usize) -> Sign {
        let text_size = buffer[sign_start] as usize;
        let mut text = String::new();

        for letter_offset in (sign_start + 1)..(sign_start + 1 + text_size) {
            text.push(buffer[letter_offset] as char);
        }

        Sign {
            x: (buffer[sign_start + text_size + 1] as u32
                | ((buffer[sign_start + text_size + 2] as u32) << 8)
                | ((buffer[sign_start + text_size + 3] as u32) << 16)
                | ((buffer[sign_start + text_size + 4] as u32) << 24)) as usize,
            y: (buffer[sign_start + text_size + 5] as u32
                | ((buffer[sign_start + text_size + 6] as u32) << 8)
                | ((buffer[sign_start + text_size + 7] as u32) << 16)
                | ((buffer[sign_start + text_size + 8] as u32) << 24)) as usize,
            text,
        }
    }
}

fn populate_sign(buffer: &Vec<u8>, sign_start: usize) -> Vec<Sign> {
    let mut sign_count = buffer[sign_start] as usize | (buffer[sign_start + 1] as usize) << 8;
    let mut signs: Vec<Sign> = Vec::with_capacity(sign_count);

    let mut pos = sign_start + 2;

    while sign_count > 0 {
        let sign = Sign::from_buffer(&buffer, pos);
        pos += sign.get_size();

        signs.push(sign);
        sign_count -= 1;
    }


    signs
}

#[derive(Debug)]
struct WorldHeader {
    world_name: String,
    seed_text: String,
    world_generator_version: u64,
    world_unique_id: u128,
    world_id: u32,
    world_left: u32,
    world_right: u32,
    world_top: u32,
    world_bottom: u32,
    world_max_width: u32,
    world_max_height: u32,
    expert_mode: bool,
    creation_time: u64,
    moon_type: u8,
    tree_x0: u32,
    tree_x1: u32,
    tree_x2: u32,
    tree_style0: u32,
    tree_style1: u32,
    tree_style2: u32,
    tree_style3: u32,
    cave_back0: u32,
    cave_back1: u32,
    cave_back2: u32,
    cave_style0: u32,
    cave_style1: u32,
    cave_style2: u32,
    cave_style3: u32,
    ice_style: u32,
    jungle_style: u32,
    hell_style: u32,
    spawn_x: u32,
    spawn_y: u32,
    world_surface: f64,
    world_rock: f64,
    temp_time: f64,
    temp_day_time: u8,
    temp_moon_phase: u32,
    temp_blood_moon: u8,
    temp_eclipse: u8,
    dungeon_x: u32,
    dungeon_y: u32,
    crimson: bool,
    downed_boss1: bool,
    downed_boss2: bool,
    downed_boss3: bool,
    downed_queen_bee: bool,
    downed_mech_boss1: bool,
    downed_mech_boss2: bool,
    downed_mech_boss3: bool,
    downed_mech_boss_any: bool,
    downed_plant_boss: bool,
    downed_golem_boss: bool,
    downed_slime_king: bool,
    saved_goblin: bool,
    saved_wizard: bool,
    saved_mech: bool,
    downed_goblins: bool,
    downed_clown: bool,
    downed_frost: bool,
    downed_pirates: bool,
    shadow_orb_smashed: bool,
    spawn_meteor: bool,
    shadow_orb_count: u8,
    altar_count: u32,
    hard_mode: bool,
    invasion_delay: u32,
    invasion_size: u32,
    invasion_type: u32,
    invasion_x: f64,
    slime_rain_time: f64,
    sundial_cooldown: u8,
    temp_rain: bool,
    temp_rain_time: u32,
    temp_max_rain: f32,
    ore_tier1: u32,
    ore_tier2: u32,
    ore_tier3: u32,
    tree_bg: u8,
    corrupt_bg: u8,
    jungle_bg: u8,
    snow_bg: u8,
    hallow_bg: u8,
    crimson_bg: u8,
    desert_bg: u8,
    ocean_bg: u8,
    cloud_bgactive: u32,
    cloud_count: u16,
    wind_speed: f32,
    angler_who_finished_today: Vec<String>,
    saved_angler: bool,
    angler_quest: u32,
    saved_stylist: bool,
    saved_tax_collector: bool,
    invasion_size_start: u32,
    temp_cultist_delay: u32,
    kill_count: Vec<u32>,
    fast_forward_time: bool,
    downed_fishron: bool,
    downed_martians: bool,
    downed_ancient_cultist: bool,
    downed_moonlord: bool,
    downed_halloween_king: bool,
    downed_halloween_tree: bool,
    downed_christmas_ice_queen: bool,
    downed_christmas_ice_santank: bool,
    downed_christmas_ice_tree: bool,
    downed_tower_solar: bool,
    downed_tower_vortex: bool,
    downed_tower_nebula: bool,
    downed_tower_stardust: bool,
    active_tower_solar: bool,
    active_tower_vortex: bool,
    active_tower_nebula: bool,
    active_tower_stardust: bool,
    lunar_apocalypse_is_up: bool,
    temp_party_manual: bool,
    temp_party_genuine: bool,
    temp_party_cooldown: bool,
    temp_party_celebrating_NPCs: Vec<u32>,
    temp_sandstorm_happening: bool,
    temp_sandstorm_time_left: u32,
    temp_sandstorm_severity: f32,
    temp_sandstorm_intended_severity: f32,
    saved_bartender: bool,
    downed_invastion_t1: bool,
    downed_invastion_t2: bool,
    downed_invastion_t3: bool,
}

/*
WorldHeader {
    world_name: String::new(),
    seed_text: String::new(),
    world_generator_version: 0,
    world_unique_id: 0,
    world_id: 0,
    world_left: 0,
    world_right: 0,
    world_top: 0,
    world_bottom: 0,
    world_max_width: 0,
    world_max_height: 0,
    expert_mode: false,
    creation_time: 0,
    moon_type: 0,
    tree_x0: 0,
    tree_x1: 0,
    tree_x2: 0,
    tree_style0: 0,
    tree_style1: 0,
    tree_style2: 0,
    tree_style3: 0,
    cave_back0: 0,
    cave_back1: 0,
    cave_back2: 0,
    cave_style0: 0,
    cave_style1: 0,
    cave_style2: 0,
    cave_style3: 0,
    ice_style: 0,
    jungle_style: 0,
    hell_style: 0,
    spawn_x: 0,
    spawn_y: 0,
    world_surface: 0.0,
    world_rock: 0.0,
    temp_time: 0.0,
    temp_day_time: 0,
    temp_moon_phase: 0,
    temp_blood_moon: 0,
    temp_eclipse: 0,
    dungeon_x: 0,
    dungeon_y: 0,
    crimson: false,
    downed_boss1: false,
    downed_boss2: false,
    downed_boss3: false,
    downed_queen_bee: false,
    downed_mech_boss1: false,
    downed_mech_boss2: false,
    downed_mech_boss3: false,
    downed_mech_boss_any: false,
    downed_plant_boss: false,
    downed_golem_boss: false,
    downed_slime_king: false,
    saved_goblin: false,
    saved_wizard: false,
    saved_mech: false,
    downed_goblins: false,
    downed_clown: false,
    downed_frost: false,
    downed_pirates: false,
    shadow_orb_smashed: false,
    spawn_meteor: false,
    shadow_orb_count: 0,
    altar_count: 0,
    hard_mode: false,
    invasion_delay: 0,
    invasion_size: 0,
    invasion_type: 0,
    invasion_x: 0.0,
    slime_rain_time: 0.0,
    sundial_cooldown: 0,
    temp_rain: false,
    temp_rain_time: 0,
    temp_max_rain: 0.0,
    ore_tier1: 0,
    ore_tier2: 0,
    ore_tier3: 0,
    tree_bg: 0,
    corrupt_bg: 0,
    jungle_bg: 0,
    snow_bg: 0,
    hallow_bg: 0,
    crimson_bg: 0,
    desert_bg: 0,
    ocean_bg: 0,
    cloud_bgactive: 0,
    cloud_count: 0,
    wind_speed: 0.0,
    angler_who_finished_today: Vec::new(),
    saved_angler: false,
    angler_quest: 0,
    saved_stylist: false,
    saved_tax_collector: false,
    invasion_size_start: 0,
    temp_cultist_delay: 0,
    kill_count: Vec::new(),
    fast_forward_time: false,
    downed_fishron: false,
    downed_martians: false,
    downed_ancient_cultist: false,
    downed_moonlord: false,
    downed_halloween_king: false,
    downed_halloween_tree: false,
    downed_christmas_ice_queen: false,
    downed_christmas_ice_santank: false,
    downed_christmas_ice_tree: false,
    downed_tower_solar: false,
    downed_tower_vortex: false,
    downed_tower_nebula: false,
    downed_tower_stardust: false,
    active_tower_solar: false,
    active_tower_vortex: false,
    active_tower_nebula: false,
    active_tower_stardust: false,
    lunar_apocalypse_is_up: false,
    temp_party_manual: false,
    temp_party_genuine: false,
    temp_party_cooldown: false,
    temp_party_celebrating_NPCs: Vec::new(),
    temp_sandstorm_happening: false,
    temp_sandstorm_time_left: 0,
    temp_sandstorm_severity: 0.0,
    temp_sandstorm_intended_severity: 0.0,
    saved_bartender: false,
    downed_invastion_t1: false,
    downed_invastion_t2: false,
    downed_invastion_t3: false,
}*/

impl WorldHeader {
    fn from_buffer(posbuff: &mut PositionedBuffer) -> WorldHeader {

        WorldHeader {
            world_name: posbuff.read_pstring(),
            seed_text: posbuff.read_pstring(),
            world_generator_version: posbuff.read_u64(),
            world_unique_id: posbuff.read_u128(),
            world_id: posbuff.read_u32(),
            world_left: posbuff.read_u32(),
            world_right: posbuff.read_u32(),
            world_top: posbuff.read_u32(),
            world_bottom: posbuff.read_u32(),
            world_max_width: posbuff.read_u32(),
            world_max_height: posbuff.read_u32(),
            expert_mode: posbuff.read_bool(),
            creation_time: posbuff.read_u64(),
            moon_type: posbuff.read_u8(),
            tree_x0: posbuff.read_u32(),
            tree_x1: posbuff.read_u32(),
            tree_x2: posbuff.read_u32(),
            tree_style0: posbuff.read_u32(),
            tree_style1: posbuff.read_u32(),
            tree_style2: posbuff.read_u32(),
            tree_style3: posbuff.read_u32(),
            cave_back0: posbuff.read_u32(),
            cave_back1: posbuff.read_u32(),
            cave_back2: posbuff.read_u32(),
            cave_style0: posbuff.read_u32(),
            cave_style1: posbuff.read_u32(),
            cave_style2: posbuff.read_u32(),
            cave_style3: posbuff.read_u32(),
            ice_style: posbuff.read_u32(),
            jungle_style: posbuff.read_u32(),
            hell_style: posbuff.read_u32(),
            spawn_x: posbuff.read_u32(),
            spawn_y: posbuff.read_u32(),
            world_surface: posbuff.read_f64(),
            world_rock: posbuff.read_f64(),
            temp_time: posbuff.read_f64(),
            temp_day_time: posbuff.read_u8(),
            temp_moon_phase: posbuff.read_u32(),
            temp_blood_moon: posbuff.read_u8(),
            temp_eclipse: posbuff.read_u8(),
            dungeon_x: posbuff.read_u32(),
            dungeon_y: posbuff.read_u32(),
            crimson: posbuff.read_bool(),
            downed_boss1: posbuff.read_bool(),
            downed_boss2: posbuff.read_bool(),
            downed_boss3: posbuff.read_bool(),
            downed_queen_bee: posbuff.read_bool(),
            downed_mech_boss1: posbuff.read_bool(),
            downed_mech_boss2: posbuff.read_bool(),
            downed_mech_boss3: posbuff.read_bool(),
            downed_mech_boss_any: posbuff.read_bool(),
            downed_plant_boss: posbuff.read_bool(),
            downed_golem_boss: posbuff.read_bool(),
            downed_slime_king: posbuff.read_bool(),
            saved_goblin: posbuff.read_bool(),
            saved_wizard: posbuff.read_bool(),
            saved_mech: posbuff.read_bool(),
            downed_goblins: posbuff.read_bool(),
            downed_clown: posbuff.read_bool(),
            downed_frost: posbuff.read_bool(),
            downed_pirates: posbuff.read_bool(),
            shadow_orb_smashed: posbuff.read_bool(),
            spawn_meteor: posbuff.read_bool(),
            shadow_orb_count: posbuff.read_u8(),
            altar_count: posbuff.read_u32(),
            hard_mode: posbuff.read_bool(),
            invasion_delay: posbuff.read_u32(),
            invasion_size: posbuff.read_u32(),
            invasion_type: posbuff.read_u32(),
            invasion_x: posbuff.read_f64(),
            slime_rain_time: posbuff.read_f64(),
            sundial_cooldown: posbuff.read_u8(),
            temp_rain: posbuff.read_bool(),
            temp_rain_time: posbuff.read_u32(),
            temp_max_rain: posbuff.read_f32(),
            ore_tier1: posbuff.read_u32(),
            ore_tier2: posbuff.read_u32(),
            ore_tier3: posbuff.read_u32(),
            tree_bg: posbuff.read_u8(),
            corrupt_bg: posbuff.read_u8(),
            jungle_bg: posbuff.read_u8(),
            snow_bg: posbuff.read_u8(),
            hallow_bg: posbuff.read_u8(),
            crimson_bg: posbuff.read_u8(),
            desert_bg: posbuff.read_u8(),
            ocean_bg: posbuff.read_u8(),
            cloud_bgactive: posbuff.read_u32(),
            cloud_count: posbuff.read_u16(),
            wind_speed: posbuff.read_f32(),
            angler_who_finished_today: posbuff.read_list(
                &mut PositionedBuffer::read_pstring,
                &mut PositionedBuffer::read_u32,
            ),
            saved_angler: posbuff.read_bool(),
            angler_quest: posbuff.read_u32(),
            saved_stylist: posbuff.read_bool(),
            saved_tax_collector: posbuff.read_bool(),
            invasion_size_start: posbuff.read_u32(),
            temp_cultist_delay: posbuff.read_u32(),
            kill_count: posbuff.read_list(
                &mut PositionedBuffer::read_u32,
                &mut PositionedBuffer::read_u16,
            ),
            fast_forward_time: posbuff.read_bool(),
            downed_fishron: posbuff.read_bool(),
            downed_martians: posbuff.read_bool(),
            downed_ancient_cultist: posbuff.read_bool(),
            downed_moonlord: posbuff.read_bool(),
            downed_halloween_king: posbuff.read_bool(),
            downed_halloween_tree: posbuff.read_bool(),
            downed_christmas_ice_queen: posbuff.read_bool(),
            downed_christmas_ice_santank: posbuff.read_bool(),
            downed_christmas_ice_tree: posbuff.read_bool(),
            downed_tower_solar: posbuff.read_bool(),
            downed_tower_vortex: posbuff.read_bool(),
            downed_tower_nebula: posbuff.read_bool(),
            downed_tower_stardust: posbuff.read_bool(),
            active_tower_solar: posbuff.read_bool(),
            active_tower_vortex: posbuff.read_bool(),
            active_tower_nebula: posbuff.read_bool(),
            active_tower_stardust: posbuff.read_bool(),
            lunar_apocalypse_is_up: posbuff.read_bool(),
            temp_party_manual: posbuff.read_bool(),
            temp_party_genuine: posbuff.read_bool(),
            temp_party_cooldown: posbuff.read_bool(),
            temp_party_celebrating_NPCs: posbuff.read_list(
                &mut PositionedBuffer::read_u32,
                &mut PositionedBuffer::read_u32,
            ),
            temp_sandstorm_happening: posbuff.read_bool(),
            temp_sandstorm_time_left: posbuff.read_u32(),
            temp_sandstorm_severity: posbuff.read_f32(),
            temp_sandstorm_intended_severity: posbuff.read_f32(),
            saved_bartender: posbuff.read_bool(),
            downed_invastion_t1: posbuff.read_bool(),
            downed_invastion_t2: posbuff.read_bool(),
            downed_invastion_t3: posbuff.read_bool(),
        }
    }

    fn print(&self) {
        println!("{:?}", self);
    }


    fn get_world_id(&self) -> u32 {
        //self._get_u32(offset: usize)
        0
    }
}

struct PositionedBuffer {
    data: Vec<u8>,
    pos: usize,
}

impl PositionedBuffer {
    fn new(buff: Vec<u8>, pos: usize) -> PositionedBuffer {
        PositionedBuffer { data: buff, pos }
    }

    fn read_u8(&mut self) -> u8 {
        //println!("Pos: {} u8", self.pos);
        self.pos += 1;
        self.data[self.pos - 1]
    }

    fn read_u16(&mut self) -> u16 {
        //println!("Pos: {} u16", self.pos);
        self.pos += 2;

        self.data[self.pos - 2] as u16 | (self.data[self.pos - 1] as u16) << 8
    }

    fn read_u32(&mut self) -> u32 {
        //println!("Pos: {} u32", self.pos);
        self.pos += 4;

        self.data[self.pos - 4] as u32
            | (self.data[self.pos - 3] as u32) << 8
            | (self.data[self.pos - 2] as u32) << 16
            | (self.data[self.pos - 1] as u32) << 24
    }

    fn read_bool(&mut self) -> bool {
        //print!("B");
        if self.read_u8() == 1 {
            true
        } else {
            false
        }
    }

    fn read_u64(&mut self) -> u64 {
        //println!("Pos: {} u64", self.pos);
        let mut raw = [0; 8];
        for offset in 0..8 {
            raw[offset] = self.data[self.pos + offset];
        }
        self.pos += 8;
        u64::from_le_bytes(raw)
    }

    fn read_f64(&mut self) -> f64 {
        //print!("F");
        f64::from_bits(self.read_u64())
    }

    fn read_f32(&mut self) -> f32 {
        //print!("F");
        f32::from_bits(self.read_u32())
    }
    fn print_location(&self) -> Vec<String> {
        //println!("Vector {}", self.pos);
        Vec::new()
    }

    fn read_u128(&mut self) -> u128 {
        //println!("Pos: {} u128", self.pos);
        let mut raw = [0; 16];
        for offset in 0..16 {
            raw[offset] = self.data[self.pos + offset];
        }
        self.pos += 16;
        u128::from_le_bytes(raw)
    }

    fn read_pstring(&mut self) -> String {
        //println!("Pos: {} string", self.pos);
        // A single prefixed byte determines the strings length
        // max length is therefor 255
        let size: usize = self.data[self.pos] as usize;

        let mut string = String::with_capacity(size);
        for offset in 1..(size + 1) {
            string.push(self.data[self.pos + offset] as char)
        }
        self.pos += size + 1;

        string
    }

    /// Read a list of generic types
    ///
    /// * `reader` - A function that reads the specified type
    ///
    /// # Example
    /// `
    /// // Setup the data
    /// let data: Vec<u8> = vec!
    ///     [
    ///         0x03, 0x00, 0x00, 0x00, // u32 denoting size
    ///         1, // First u8
    ///         2,
    ///         3, // Last u8
    ///     ];
    ///
    /// let mut posbuff = PositionedBuffer::new(data, 0);
    ///
    /// // Use the read_8 method to read items
    /// // read_u8 -> u8 therefor read_list -> Vec<u8>
    /// let result = posbuff.read_list(&mut PositionedBuffer::read_u8);
    ///
    /// assert_eq!(result, vec![1, 2, 3]);
    /// `
    fn read_list<T, G: ToPrimitive>(
        &mut self,
        reader: &mut FnMut(&mut Self) -> T,
        sizer: &mut FnMut(&mut Self) -> G,
    ) -> Vec<T> {
        let size = ToPrimitive::to_usize(&sizer(self)).unwrap();
        let mut items = Vec::with_capacity(size);

        for _ in 0..size {
            let v: T = reader(self);
            items.push(v);
        }

        items
    }
}

struct TileEntity {
    variety: u8,
    id: u32,
    x: u16,
    y: u16,
}

impl TileEntity {
    fn from_buffer(pbuffer: &mut PositionedBuffer) -> TileEntity {
        TileEntity {
            variety: pbuffer.read_u8(),
            id: pbuffer.read_u32(),
            x: pbuffer.read_u16(),
            y: pbuffer.read_u16(),
        }
    }
}

fn populate_tile_entities(pbuffer: &mut PositionedBuffer) -> Vec<TileEntity> {
    pbuffer.read_list(
        &mut TileEntity::from_buffer,
        &mut PositionedBuffer::read_u32,
    )
}

struct WeightedPressurePlate {
    x: u32,
    y: u32,
}

impl WeightedPressurePlate {
    fn from_buffer(pbuffer: &mut PositionedBuffer) -> WeightedPressurePlate {
        WeightedPressurePlate {
            x: pbuffer.read_u32(),
            y: pbuffer.read_u32(),
        }
    }
}

fn populate_weighted_pressure_plate(pbuffer: &mut PositionedBuffer) -> Vec<WeightedPressurePlate> {
    pbuffer.read_list(
        &mut WeightedPressurePlate::from_buffer,
        &mut PositionedBuffer::read_u32,
    )
}

struct RoomLocation {
    npc_id: u32,
    x: u32,
    y: u32,
}

impl RoomLocation {
    fn from_buffer(pbuffer: &mut PositionedBuffer) -> RoomLocation {
        RoomLocation {
            npc_id: pbuffer.read_u32(),
            x: pbuffer.read_u32(),
            y: pbuffer.read_u32(),
        }
    }
}

fn populate_town(pbuffer: &mut PositionedBuffer) -> Vec<RoomLocation> {
    pbuffer.read_list(
        &mut RoomLocation::from_buffer,
        &mut PositionedBuffer::read_u32,
    )
}

struct Footer {
    boolean: bool,
    world_name: String,
    world_id: u32,
}

impl Footer {
    fn from_buffer(pbuffer: &mut PositionedBuffer) -> Footer {
        Footer {
            boolean: pbuffer.read_bool(),
            world_name: pbuffer.read_pstring(),
            world_id: pbuffer.read_u32(),
        }
    }
}

struct NPC {
    variety: u32,
    name: String,
    x: f32,
    y: f32,
    homeless: bool,
    x_home: u32,
    y_home: u32,
}

impl NPC {
    fn from_buffer_town(pbuffer: &mut PositionedBuffer) -> NPC {
        NPC {
            variety: pbuffer.read_u32(),
            name: pbuffer.read_pstring(),
            x: pbuffer.read_f32(),
            y: pbuffer.read_f32(),
            homeless: pbuffer.read_bool(),
            x_home: pbuffer.read_u32(),
            y_home: pbuffer.read_u32(),
        }
    }

    fn from_buffer_enemy(pbuffer: &mut PositionedBuffer) -> NPC {
        // At the moment this only appears to be the pillars
        NPC {
            variety: pbuffer.read_u32(),
            name: String::new(),
            x: pbuffer.read_f32(),
            y: pbuffer.read_f32(),
            homeless: false,
            x_home: 0,
            y_home: 0,
        }
    }
}

fn populate_npcs(pbuffer: &mut PositionedBuffer) -> Vec<NPC> {
    let mut npcs = Vec::new();

    while pbuffer.read_bool() {
        npcs.push(NPC::from_buffer_town(pbuffer));
    }

    while pbuffer.read_bool() {
        npcs.push(NPC::from_buffer_enemy(pbuffer));
    }

    npcs
}

struct World {
    file_header: Header,
    world_header: WorldHeader,
    tiles: Vec<RawTile>,
    chests: Vec<Chest>,
    signs: Vec<Sign>,
    npcs: Vec<NPC>,
    tile_entities: Vec<TileEntity>,
    weighted_plates: Vec<WeightedPressurePlate>,
    town_manager: Vec<RoomLocation>,
    footer: Footer,
}

fn main() -> io::Result<()> {
    let file_name = "test1.wld";
    let mut file = File::open(file_name)?;

    let header = Header::new(&mut file);

    //println!("{}", header.to_string());
    header.print_pointers();

    //Header::new(&mut File::open("npc0.wld").unwrap()).print_pointers();
    //Header::new(&mut File::open("npc1.wld").unwrap()).print_pointers();
    //Header::new(&mut File::open("npc2.wld").unwrap()).print_pointers();
    println!();
    //Header::new(&mut File::open("test1.wld").unwrap()).print_pointers();

    //println!("{}", mem::size_of::<RawTile>());

    let mut file = File::open(file_name)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    let mut posbuff = PositionedBuffer::new(buffer, header.pointers[0]);
    WorldHeader::from_buffer(&mut posbuff).print();

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

    //println!("Sized: {}", std::mem::size_of::<WorldHeader>());

    let mut tile_data: Vec<RawTile> = Vec::with_capacity(4200 * 1200);

    //println!("\n\nPopulation:\n");
    //populate_tiles(&mut tile_data, &buffer, header.pointers[1], 4200 * 1200);


    //Chest::from_buffer(&buffer, 2634477).print();
    //Chest::from_buffer(&buffer, 2634733).print();
    /*for e in (2634470 as usize)..(2634500 as usize) {
    println!("{}: {}", e, buffer[e]);
    }*/

    /*Sign::from_buffer(&buffer, 2685607).print();
    Sign::from_buffer(&buffer, 2685616).print();
    */

    /*for chest in populate_chests(&buffer, header.pointers[2]) {
        chest.print();
    }*/

    /*for sign in populate_sign(&buffer, header.pointers[3]) {
        sign.print();
    }*/


    Ok(())
}
