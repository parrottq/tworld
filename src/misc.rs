use crate::parsing::PositionedBuffer;

pub struct Sign {
    text: String,
    x: u32,
    y: u32,
}

impl Sign {
    pub fn print(&self) {
        if self.text.len() == 0 {
            println!("Empty Sign @ ({}, {})", self.x, self.y);
        } else {
            println!("Sign @ ({}, {}):", self.x, self.y);
            println!("{}", self.text);
        }
    }

    pub fn from_buffer(pbuffer: &mut PositionedBuffer) -> Sign {
        Sign {
            text: pbuffer.read_pstring(),
            x: pbuffer.read_u32(),
            y: pbuffer.read_u32(),
        }
    }
}

pub fn populate_sign(pbuffer: &mut PositionedBuffer) -> Vec<Sign> {
    pbuffer.read_list(&mut Sign::from_buffer, &mut PositionedBuffer::read_u16)
}

pub struct TileEntity {
    variety: u8,
    id: u32,
    x: u16,
    y: u16,
}

impl TileEntity {
    pub fn from_buffer(pbuffer: &mut PositionedBuffer) -> TileEntity {
        TileEntity {
            variety: pbuffer.read_u8(),
            id: pbuffer.read_u32(),
            x: pbuffer.read_u16(),
            y: pbuffer.read_u16(),
        }
    }
}

pub fn populate_tile_entities(pbuffer: &mut PositionedBuffer) -> Vec<TileEntity> {
    pbuffer.read_list(
        &mut TileEntity::from_buffer,
        &mut PositionedBuffer::read_u32,
    )
}

pub struct WeightedPressurePlate {
    x: u32,
    y: u32,
}

impl WeightedPressurePlate {
    pub fn from_buffer(pbuffer: &mut PositionedBuffer) -> WeightedPressurePlate {
        WeightedPressurePlate {
            x: pbuffer.read_u32(),
            y: pbuffer.read_u32(),
        }
    }
}

pub fn populate_weighted_pressure_plate(
    pbuffer: &mut PositionedBuffer,
) -> Vec<WeightedPressurePlate> {
    pbuffer.read_list(
        &mut WeightedPressurePlate::from_buffer,
        &mut PositionedBuffer::read_u32,
    )
}

pub struct RoomLocation {
    npc_id: u32,
    x: u32,
    y: u32,
}

impl RoomLocation {
    pub fn from_buffer(pbuffer: &mut PositionedBuffer) -> RoomLocation {
        RoomLocation {
            npc_id: pbuffer.read_u32(),
            x: pbuffer.read_u32(),
            y: pbuffer.read_u32(),
        }
    }
}

pub fn populate_town(pbuffer: &mut PositionedBuffer) -> Vec<RoomLocation> {
    pbuffer.read_list(
        &mut RoomLocation::from_buffer,
        &mut PositionedBuffer::read_u32,
    )
}
