use crate::parsing::PositionedBuffer;
use crate::writing::PrimitiveWriting;

#[derive(Clone)]
pub enum Item {
    None,
    Normal(u16, u32),
    Buffed(u16, u32, u8),
}

impl Item {
    pub fn print(&self) {
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

    pub fn from_buffer(pbuffer: &mut PositionedBuffer) -> Item {
        let amount = pbuffer.read_u16();
        if amount == 0 {
            return Item::None;
        }

        let id = pbuffer.read_u32();
        let buff = pbuffer.read_u8();

        if buff == 0 {
            return Item::Normal(amount, id);
        }

        Item::Buffed(amount, id, buff)
    }

    pub fn write(file: &mut std::fs::File, item: Self) {
        match item {
            Item::None => file.write_u16(0),

            Item::Normal(amount, id) => {
                file.write_u16(amount);
                file.write_u32(id);
                file.write_u8(0);
            }

            Item::Buffed(amount, id, buff) => {
                file.write_u16(amount);
                file.write_u32(id);
                file.write_u8(buff);
            }
        }
    }
}

pub fn parse_chest_items(pbuffer: &mut PositionedBuffer) -> Vec<Item> {
    pbuffer.read_list(&mut Item::from_buffer, &mut |_| 40)
}

#[derive(Clone)]
pub struct Chest {
    name: String,
    x: u32,
    y: u32,
    items: Vec<Item>,
    original_size: usize,
}

impl Chest {
    pub fn print(&self) {
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

    pub fn from_buffer(pbuffer: &mut PositionedBuffer) -> Chest {
        Chest {
            x: pbuffer.read_u32(),
            y: pbuffer.read_u32(),
            name: pbuffer.read_pstring(),
            items: parse_chest_items(pbuffer),
            original_size: 0,
        }
    }

    pub fn write(file: &mut std::fs::File, chest: Self) {
        file.write_u32(chest.x);
        file.write_u32(chest.y);
        file.write_string(chest.name.clone());

        file.write_list(
            &chest.items,
            &mut Item::write,
            &mut |_: &mut std::fs::File, _: u8| {},
        )
    }
}

pub fn write_chests(rooms: &Vec<Chest>, file: &mut std::fs::File) -> usize {
    file.write_u16(rooms.len() as u16);
    file.write_u16(40);
    file.write_list(rooms, &mut Chest::write, &mut PrimitiveWriting::write_void);

    file.current_pos()
}

pub fn populate_chests(pbuffer: &mut PositionedBuffer) -> Vec<Chest> {
    let count = pbuffer.read_u16();

    // I think these two bytes are redudante but not completely sure
    let capacity = pbuffer.read_u16();
    if capacity != 40 {
        panic!("Chest capacity should always be 40 but was {}. Handling other sizes is not implemented", capacity);
    }

    pbuffer.read_list(&mut Chest::from_buffer, &mut |_| count)
}
