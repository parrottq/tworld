use crate::parsing::PositionedBuffer;

pub struct Header {
    pub release: u32,
    pub filetype: u8,
    pub revision: u32,
    pub is_favorite: u64,
    pub pointers: Vec<usize>,
}

impl Header {

    pub fn from_buffer(pbuffer: &mut PositionedBuffer) -> Header {
        let release = pbuffer.read_u32();

        for reference in ['r', 'e', 'l', 'o', 'g', 'i', 'c'].iter() {
            if *reference != (pbuffer.read_u8() as char) {
                println!("'relogic' siganture does not match");
            }
        }
        let filetype = pbuffer.read_u8();
        let revision = pbuffer.read_u32();
        let is_favorite = pbuffer.read_u64();
        let pointers = pbuffer
            .read_list(
                &mut PositionedBuffer::read_u32,
                &mut PositionedBuffer::read_u16,
            )
            .iter()
            .map(|&x| x as usize)
            .collect();

        Header {
            release,
            filetype,
            revision,
            is_favorite,
            pointers,
        }
    }
    pub fn to_string(&self) -> String {
        // TODO: Find a way to do this in rust
        format!(
            "{}, {}, {}, {}, {:?}",
            self.release, self.filetype, self.revision, self.is_favorite, self.pointers
        )
    }

    pub fn print_pointers(&self) {
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

pub struct Footer {
    pub boolean: bool,
    pub world_name: String,
    pub world_id: u32,
}

impl Footer {
    pub fn from_buffer(pbuffer: &mut PositionedBuffer) -> Footer {
        Footer {
            boolean: pbuffer.read_bool(),
            world_name: pbuffer.read_pstring(),
            world_id: pbuffer.read_u32(),
        }
    }
}