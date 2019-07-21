#![allow(dead_code)]

use std::fs::File;

use std::io;
use std::io::Read;

mod parsing;
mod writing;
use parsing::PositionedBuffer;

mod chest;
mod container;
mod metadata;
mod misc;
mod npc;
mod tile;
mod world;

fn main() -> io::Result<()> {
    //TODO: Better error handling
    let file_name = "signtest.wld";
    //let mut file = File::open(file_name)?;

    //let header = Header::new(&mut file);

    //println!("{}", header.to_string());
    //header.print_pointers();

    //Header::new(&mut File::open("npc0.wld").unwrap()).print_pointers();
    //Header::new(&mut File::open("npc1.wld").unwrap()).print_pointers();
    //Header::new(&mut File::open("npc2.wld").unwrap()).print_pointers();
    //println!();
    //Header::new(&mut File::open("test1.wld").unwrap()).print_pointers();

    //println!("{}", mem::size_of::<RawTile>());

    let mut file = File::open(file_name)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    //let mut posbuff = positionedbuffer::new(buffer, 0);
    //let header = header::from_buffer(&mut posbuff);
    //header.print_pointers();

    /*WorldHeader::from_buffer(&mut posbuff).print();
     */

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

    println!("Starting new...");
    let mut pbuffer = PositionedBuffer::new(buffer, 0);
    let wld = world::World::from_buffer(&mut pbuffer);
    wld.file_header.print_pointers();
    println!("Done.\n\n");

    /*for villager in wld.npcs.iter() {
        villager.print();
        println!();
    }*/

    println!("Starting write...");
    let mut output_file = File::create("output2.wld")?;
    wld.write_to_file(&mut output_file);
    println!("Done.");

    //Chest::from_buffer(&buffer, 2634477).print();
    //Chest::from_buffer(&buffer, 2634733).print();
    /*for e in (2634470 as usize)..(2634500 as usize) {
    println!("{}: {}", e, buffer[e]);
    }*/

    //Sign::from_buffer(&buffer, 2685607).print();
    //posbuff.pos = header.pointers[3];
    //Sign::from_buffer(&mut posbuff).print();

    /*posbuff.pos = header.pointers[2];
    for chest in populate_chests(&mut posbuff) {
        chest.print();
    }*/

    /*for sign in populate_sign(&mut posbuff) {
        sign.print();
    }*/

    Ok(())
}
