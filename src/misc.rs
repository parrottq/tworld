use crate::parsing::PositionedBuffer;
use crate::writing::PrimitiveWriting;

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

    pub fn write(file: &mut std::fs::File, sign: &Self) {
        file.write_string(&sign.text);
        file.write_u32(&sign.x);
        file.write_u32(&sign.y);
    }
}

pub fn write_signs(signs: &Vec<Sign>, file: &mut std::fs::File) -> usize {
    file.write_list(&signs, &mut Sign::write, &mut PrimitiveWriting::write_u16);

    file.current_pos()
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

    pub fn write(file: &mut std::fs::File, tile_entity: &Self) {
        file.write_u8(&tile_entity.variety);
        file.write_u32(&tile_entity.id);
        file.write_u16(&tile_entity.x);
        file.write_u16(&tile_entity.y);
    }
}

pub fn write_tile_entities(tile_entities: &Vec<TileEntity>, file: &mut std::fs::File) -> usize {
    file.write_list(
        &tile_entities,
        &mut TileEntity::write,
        &mut PrimitiveWriting::write_u32,
    );

    file.current_pos()
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

    pub fn write(file: &mut std::fs::File, plate: &Self) {
        file.write_u32(&plate.x);
        file.write_u32(&plate.y);
    }
}

pub fn write_weighted_pressure_plates(
    weighted_pressure_plates: &Vec<WeightedPressurePlate>,
    file: &mut std::fs::File,
) -> usize {
    file.write_list(
        &weighted_pressure_plates,
        &mut WeightedPressurePlate::write,
        &mut PrimitiveWriting::write_u32,
    );

    file.current_pos()
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

    pub fn write(file: &mut std::fs::File, room: &Self) {
        file.write_u32(&room.npc_id);
        file.write_u32(&room.x);
        file.write_u32(&room.y);
    }
}

pub fn write_rooms(rooms: &Vec<RoomLocation>, file: &mut std::fs::File) -> usize {
    file.write_list(
        &rooms,
        &mut RoomLocation::write,
        &mut PrimitiveWriting::write_u32,
    );

    file.current_pos()
}

pub fn populate_town(pbuffer: &mut PositionedBuffer) -> Vec<RoomLocation> {
    pbuffer.read_list(
        &mut RoomLocation::from_buffer,
        &mut PositionedBuffer::read_u32,
    )
}
