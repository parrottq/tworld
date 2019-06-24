
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
    pointers: Vec<u32>,
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

        let mut pointers = Vec::new();
        let mut point_raw = [0; 2];
        file.read(&mut point_raw).unwrap();

        let point_count = Cursor::new(point_raw).read_u16::<LittleEndian>().unwrap();

        for _ in 0..point_count {
            let mut value = [0; 4];
            file.read(&mut value).unwrap();
            //println!("{:?}", value);

            pointers.push(Cursor::new(value).read_u32::<LittleEndian>().unwrap());
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


struct RawTile {
    data: u64,
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
}


impl RawTile {
    fn print(&self) {
        println!("Tile ID: {},\nLiquid amount: {}/255,\nWall ID: {}, Wiring: (r: {}, g: {}, b: {}),\nLiquid Type: {}\nTile alteration: {},\nActuator: {}; active: {},\nTile paint: {},\nWall paint: {}",
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
        RawTile { data: 0 }
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

}

fn parse_tile(data: &Vec<u8>, pos: usize) -> (u16, RawTile) {
    let mut tile = RawTile::new();
    let mut offset = 1;


    if (data[pos] >> 0) & 0b1 == 0b1 {
        println!("Flag2");
        offset += 1;

        tile.set_red_wiring((data[pos + 1] >> 1) & 0b1 == 0b1);
        tile.set_blue_wiring((data[pos + 1] >> 2) & 0b1 == 0b1);
        tile.set_green_wiring((data[pos + 1] >> 3) & 0b1 == 0b1);

        // TODO: alt-slope, actuator; enabled

        if (data[pos + 1] >> 0) & 0b1 == 0b1 {
            println!("Flag3");
            offset += 1;
        }
    }

    if (data[pos] >> 1) & 0b1 == 0b1 {
        println!("Tile present, offset {}", pos + offset);

        if (data[pos] >> 5) & 0b1 == 0b1 {
            // u16 tile
            println!("u16 tile");
            tile.set_tile_id(
                Cursor::new([data[pos + offset], data[pos + offset + 1]])
                    .read_u16::<LittleEndian>()
                    .unwrap(),
            );

            offset += 2;
        } else {
            // u8 tile
            println!("u8 tile");
            tile.set_tile_id(data[pos + offset + 1] as u16);

            offset += 1;
        }
    } else {
        println!("No tile present");
        tile.set_tile_id(511);
    }

    // TODO: Frame x & y

    if offset == 3 && (data[pos] >> 3) & 0b1 == 0b1 {
        println!("Tile is painted");

        tile.set_tile_paint(data[pos + offset]);
        offset += 1;
    }

    if (data[pos] >> 2) & 0b1 == 0b1 {
        println!("Wall is present");

        tile.set_wall_id(data[pos + offset]);
        offset += 1;
    }

    if offset == 3 && (data[pos] >> 4) & 0b1 == 0b1 {
        println!("Wall is painted");

        tile.set_wall_paint(data[pos + offset]);
        offset += 1;
    }

    if (data[pos] >> 3) & 0b11 != 0b0 {
        println!("Fluid present");
        tile.set_fluid_type((data[pos] >> 3) & 0b11);

        tile.set_fluid_amount(data[pos + offset]);
        offset += 1;
    }

    let mut repetitions = 0;
    if (data[pos] >> 6) & 0b1 == 0b1 {
        print!("RLE present: ");

        if (data[pos] >> 7) & 0b1 == 0b1 {
            // u16 RLE
            println!("u16");
            repetitions = Cursor::new([data[pos + offset], data[pos + offset + 1]])
                .read_u16::<LittleEndian>()
                .unwrap();

            offset += 2;
        } else {
            // u8 RLE
            println!("u8");
            repetitions = data[pos + offset + 1] as u16;

            offset += 1;
        }
    } else {
        println!("No tile present");
        tile.set_tile_id(511);
    }

    println!("Total offset: {}", offset);

    (repetitions, tile)
}


fn main() -> io::Result<()> {
    let mut file = File::open("test1.wld")?;

    let header = Header::new(&mut file);

    println!("{}", header.to_string());

    println!("{}", mem::size_of::<RawTile>());

    let mut file = File::open("test1.wld")?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    println!("{}\n", buffer[header.pointers[1] as usize],);

    parse_tile(&buffer, header.pointers[1] as usize + 0)
        .1
        .print();

    let mut tile = RawTile::new();

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
    println!("{:b}\n{}", tile.data, tile.get_tile_id());

    tile.print();

    Ok(())
}
