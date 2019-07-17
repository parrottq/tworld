use crate::parsing::PositionedBuffer;

pub struct NPC {
    variety: u32,
    name: String,
    x: f32,
    y: f32,
    homeless: bool,
    x_home: u32,
    y_home: u32,
}

impl NPC {
    pub fn from_buffer_town(pbuffer: &mut PositionedBuffer) -> NPC {
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

    pub fn from_buffer_enemy(pbuffer: &mut PositionedBuffer) -> NPC {
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

pub fn populate_npcs(pbuffer: &mut PositionedBuffer) -> Vec<NPC> {
    let mut npcs = Vec::new();

    while pbuffer.read_bool() {
        npcs.push(NPC::from_buffer_town(pbuffer));
    }

    while pbuffer.read_bool() {
        npcs.push(NPC::from_buffer_enemy(pbuffer));
    }

    npcs
}