use crate::parsing::PositionedBuffer;
use crate::{chest, container, metadata, misc, npc, tile};
pub struct World {
    pub file_header: container::Header,
    pub world_header: metadata::WorldHeader,
    pub tiles: Vec<tile::RawTile>,
    pub chests: Vec<chest::Chest>,
    pub signs: Vec<misc::Sign>,
    pub npcs: Vec<npc::NPC>,
    pub tile_entities: Vec<misc::TileEntity>,
    pub weighted_plates: Vec<misc::WeightedPressurePlate>,
    pub town_manager: Vec<misc::RoomLocation>,
    pub footer: container::Footer,
}

impl World {
    pub fn from_buffer(pbuffer: &mut PositionedBuffer) -> World {
        pbuffer.pos = 0;
        let file_header = container::Header::from_buffer(pbuffer);

        pbuffer.pos = file_header.pointers[0];
        let world_header = metadata::WorldHeader::from_buffer(pbuffer);

        pbuffer.pos = file_header.pointers[1];
        let tiles = tile::populate_tiles(pbuffer, &world_header);

        pbuffer.pos = file_header.pointers[2];
        let chests = chest::populate_chests(pbuffer);

        pbuffer.pos = file_header.pointers[3];
        let signs = misc::populate_sign(pbuffer);

        pbuffer.pos = file_header.pointers[4];
        let npcs = npc::populate_npcs(pbuffer);

        pbuffer.pos = file_header.pointers[5];
        let tile_entities = misc::populate_tile_entities(pbuffer);

        pbuffer.pos = file_header.pointers[6];
        let weighted_plates = misc::populate_weighted_pressure_plate(pbuffer);

        pbuffer.pos = file_header.pointers[7];
        let town_manager = misc::populate_town(pbuffer);

        pbuffer.pos = file_header.pointers[8];
        let footer = container::Footer::from_buffer(pbuffer);

        World {
            file_header,
            world_header,
            tiles,
            chests,
            signs,
            npcs,
            tile_entities,
            weighted_plates,
            town_manager,
            footer,
        }
    }

    pub fn write_to_file(&self, file: &mut std::fs::File) {
        let pointers = [
            self.file_header.write_to_file(file),
            self.world_header.write_to_file(file),
            tile::write_tiles(&self.tiles, file, self.world_header.world_max_height),
            chest::write_chests(&self.chests, file),
            misc::write_signs(&self.signs, file),
            npc::write_npcs(&self.npcs, file),
            misc::write_tile_entities(&self.tile_entities, file),
            misc::write_weighted_pressure_plates(&self.weighted_plates, file),
            misc::write_rooms(&self.town_manager, file),
            0,
        ];

        println!("Pointers: {:?}", pointers);

        self.footer.write_to_file(file);
        self.file_header.write_pointers(
            file,
            pointers.iter().map(|x| *x as u32).collect::<Vec<u32>>(),
        );
    }
}
