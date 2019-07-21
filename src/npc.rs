use crate::parsing::PositionedBuffer;
use crate::writing::PrimitiveWriting;

pub struct NPC {
    pub variety: u32,
    pub name: String,
    pub x: f32, // This is in pixels not tiles. 1 tile = 16 pixels
    pub y: f32, // ..
    pub homeless: bool,
    pub x_home: u32,
    pub y_home: u32,
    pub is_town: bool,
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
            is_town: true,
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
            is_town: false,
        }
    }

    pub fn print(&self) {
        println!("{}: Type", self.variety);

        if self.name.is_empty() {
            println!("No name");
        } else {
            println!("{}: Name", self.name);
        }

        println!(
            "@ P({}, {})\n@ T({}, {})",
            self.x,
            self.y,
            self.x / 16.0,
            self.y / 16.0
        );

        println!("{}: Homeless", self.homeless);
        if !self.homeless {
            println!("Home @ ({}, {})", self.x_home, self.y_home);
        }
    }

    pub fn write(&self, file: &mut std::fs::File) {
        file.write_u32(&self.variety);

        if self.is_town {
            file.write_string(&self.name);
        }

        file.write_f32(&self.x);
        file.write_f32(&self.y);

        if self.is_town {
            file.write_bool(&self.homeless);
            file.write_u32(&self.x_home);
            file.write_u32(&self.y_home);
        }
    }
}

pub fn write_npcs(npcs: &Vec<NPC>, file: &mut std::fs::File) -> usize {
    let mut town = Vec::new();
    let mut npc_enemies = Vec::new();
    for npc in npcs {
        if npc.is_town {
            town.push(npc);
        } else {
            npc_enemies.push(npc);
        }
    }

    for npc in town {
        file.write_bool(&true);
        npc.write(file);
    }
    file.write_bool(&false);

    for npc in npc_enemies {
        file.write_bool(&true);
        npc.write(file);
    }
    file.write_bool(&false);

    file.current_pos()
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
