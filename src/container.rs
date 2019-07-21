use crate::parsing::PositionedBuffer;
use crate::tile::IMPORTANTS_BYTES;
use crate::writing::PrimitiveWriting;

use std::io::Write;

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

    pub fn write_to_file(&self, file: &mut std::fs::File) -> usize {
        file.write_u32(self.release);
        file.write(&("relogic".bytes().map(|x| x as u8).collect::<Vec<u8>>()))
            .unwrap();
        file.write_u8(self.filetype);
        file.write_u32(self.revision);
        file.write_u64(self.is_favorite);
        file.write_list(
            &vec![0u32; 10],
            &mut PrimitiveWriting::write_u32,
            &mut PrimitiveWriting::write_u16,
        );

        file.write_list(
            &IMPORTANTS_BYTES.to_vec(),
            &mut PrimitiveWriting::write_u8,
            &mut |s: &mut std::fs::File, _: u16| s.write_u16(470),
        );

        file.current_pos()
    }

    pub fn write_pointers(&self, file: &mut std::fs::File, pointers: Vec<u32>) {
        file.set_current_pos(24);
        file.write_list(
            &pointers,
            &mut PrimitiveWriting::write_u32,
            &mut PrimitiveWriting::write_u16,
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

    pub fn write_to_file(&self, file: &mut std::fs::File) -> usize {
        file.write_bool(true);
        file.write_string(self.world_name.clone());
        file.write_u32(self.world_id);

        file.current_pos()
    }
}
