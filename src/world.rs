use crate::{container,metadata,tile,chest,npc,misc};

pub struct World {
    file_header: container::Header,
    world_header: metadata::WorldHeader,
    tiles: Vec<tile::RawTile>,
    chests: Vec<chest::Chest>,
    signs: Vec<misc::Sign>,
    npcs: Vec<npc::NPC>,
    tile_entities: Vec<misc::TileEntity>,
    weighted_plates: Vec<misc::WeightedPressurePlate>,
    town_manager: Vec<misc::RoomLocation>,
    footer: container::Footer,
}