use crate::parsing::PositionedBuffer;
use crate::writing::PrimitiveWriting;

#[derive(Debug)]
pub struct WorldHeader {
    pub world_name: String,
    pub seed_text: String,
    pub world_generator_version: u64,
    pub world_unique_id: u128,
    pub world_id: u32,
    pub world_left: u32,
    pub world_right: u32,
    pub world_top: u32,
    pub world_bottom: u32,
    pub world_max_height: u32,
    pub world_max_width: u32,
    pub expert_mode: bool,
    pub creation_time: u64,
    pub moon_type: u8,
    pub tree_x0: u32,
    pub tree_x1: u32,
    pub tree_x2: u32,
    pub tree_style0: u32,
    pub tree_style1: u32,
    pub tree_style2: u32,
    pub tree_style3: u32,
    pub cave_back0: u32,
    pub cave_back1: u32,
    pub cave_back2: u32,
    pub cave_style0: u32,
    pub cave_style1: u32,
    pub cave_style2: u32,
    pub cave_style3: u32,
    pub ice_style: u32,
    pub jungle_style: u32,
    pub hell_style: u32,
    pub spawn_x: u32,
    pub spawn_y: u32,
    pub world_surface: f64,
    pub world_rock: f64,
    pub temp_time: f64,
    pub temp_day_time: u8,
    pub temp_moon_phase: u32,
    pub temp_blood_moon: u8,
    pub temp_eclipse: u8,
    pub dungeon_x: u32,
    pub dungeon_y: u32,
    pub crimson: bool,
    pub downed_boss1: bool,
    pub downed_boss2: bool,
    pub downed_boss3: bool,
    pub downed_queen_bee: bool,
    pub downed_mech_boss1: bool,
    pub downed_mech_boss2: bool,
    pub downed_mech_boss3: bool,
    pub downed_mech_boss_any: bool,
    pub downed_plant_boss: bool,
    pub downed_golem_boss: bool,
    pub downed_slime_king: bool,
    pub saved_goblin: bool,
    pub saved_wizard: bool,
    pub saved_mech: bool,
    pub downed_goblins: bool,
    pub downed_clown: bool,
    pub downed_frost: bool,
    pub downed_pirates: bool,
    pub shadow_orb_smashed: bool,
    pub spawn_meteor: bool,
    pub shadow_orb_count: u8,
    pub altar_count: u32,
    pub hard_mode: bool,
    pub invasion_delay: u32,
    pub invasion_size: u32,
    pub invasion_type: u32,
    pub invasion_x: f64,
    pub slime_rain_time: f64,
    pub sundial_cooldown: u8,
    pub temp_rain: bool,
    pub temp_rain_time: u32,
    pub temp_max_rain: f32,
    pub ore_tier1: u32,
    pub ore_tier2: u32,
    pub ore_tier3: u32,
    pub tree_bg: u8,
    pub corrupt_bg: u8,
    pub jungle_bg: u8,
    pub snow_bg: u8,
    pub hallow_bg: u8,
    pub crimson_bg: u8,
    pub desert_bg: u8,
    pub ocean_bg: u8,
    pub cloud_bgactive: u32,
    pub cloud_count: u16,
    pub wind_speed: f32,
    pub angler_who_finished_today: Vec<String>,
    pub saved_angler: bool,
    pub angler_quest: u32,
    pub saved_stylist: bool,
    pub saved_tax_collector: bool,
    pub invasion_size_start: u32,
    pub temp_cultist_delay: u32,
    pub kill_count: Vec<u32>,
    pub fast_forward_time: bool,
    pub downed_fishron: bool,
    pub downed_martians: bool,
    pub downed_ancient_cultist: bool,
    pub downed_moonlord: bool,
    pub downed_halloween_king: bool,
    pub downed_halloween_tree: bool,
    pub downed_christmas_ice_queen: bool,
    pub downed_christmas_ice_santank: bool,
    pub downed_christmas_ice_tree: bool,
    pub downed_tower_solar: bool,
    pub downed_tower_vortex: bool,
    pub downed_tower_nebula: bool,
    pub downed_tower_stardust: bool,
    pub active_tower_solar: bool,
    pub active_tower_vortex: bool,
    pub active_tower_nebula: bool,
    pub active_tower_stardust: bool,
    pub lunar_apocalypse_is_up: bool,
    pub temp_party_manual: bool,
    pub temp_party_genuine: bool,
    pub temp_party_cooldown: u32,
    pub temp_party_celebrating_npcs: Vec<u32>,
    pub temp_sandstorm_happening: bool,
    pub temp_sandstorm_time_left: u32,
    pub temp_sandstorm_severity: f32,
    pub temp_sandstorm_intended_severity: f32,
    pub saved_bartender: bool,
    pub downed_invastion_t1: bool,
    pub downed_invastion_t2: bool,
    pub downed_invastion_t3: bool,
}

/*
WorldHeader {
    world_name: String::new(),
    seed_text: String::new(),
    world_generator_version: 0,
    world_unique_id: 0,
    world_id: 0,
    world_left: 0,
    world_right: 0,
    world_top: 0,
    world_bottom: 0,
    world_max_width: 0,
    world_max_height: 0,
    expert_mode: false,
    creation_time: 0,
    moon_type: 0,
    tree_x0: 0,
    tree_x1: 0,
    tree_x2: 0,
    tree_style0: 0,
    tree_style1: 0,
    tree_style2: 0,
    tree_style3: 0,
    cave_back0: 0,
    cave_back1: 0,
    cave_back2: 0,
    cave_style0: 0,
    cave_style1: 0,
    cave_style2: 0,
    cave_style3: 0,
    ice_style: 0,
    jungle_style: 0,
    hell_style: 0,
    spawn_x: 0,
    spawn_y: 0,
    world_surface: 0.0,
    world_rock: 0.0,
    temp_time: 0.0,
    temp_day_time: 0,
    temp_moon_phase: 0,
    temp_blood_moon: 0,
    temp_eclipse: 0,
    dungeon_x: 0,
    dungeon_y: 0,
    crimson: false,
    downed_boss1: false,
    downed_boss2: false,
    downed_boss3: false,
    downed_queen_bee: false,
    downed_mech_boss1: false,
    downed_mech_boss2: false,
    downed_mech_boss3: false,
    downed_mech_boss_any: false,
    downed_plant_boss: false,
    downed_golem_boss: false,
    downed_slime_king: false,
    saved_goblin: false,
    saved_wizard: false,
    saved_mech: false,
    downed_goblins: false,
    downed_clown: false,
    downed_frost: false,
    downed_pirates: false,
    shadow_orb_smashed: false,
    spawn_meteor: false,
    shadow_orb_count: 0,
    altar_count: 0,
    hard_mode: false,
    invasion_delay: 0,
    invasion_size: 0,
    invasion_type: 0,
    invasion_x: 0.0,
    slime_rain_time: 0.0,
    sundial_cooldown: 0,
    temp_rain: false,
    temp_rain_time: 0,
    temp_max_rain: 0.0,
    ore_tier1: 0,
    ore_tier2: 0,
    ore_tier3: 0,
    tree_bg: 0,
    corrupt_bg: 0,
    jungle_bg: 0,
    snow_bg: 0,
    hallow_bg: 0,
    crimson_bg: 0,
    desert_bg: 0,
    ocean_bg: 0,
    cloud_bgactive: 0,
    cloud_count: 0,
    wind_speed: 0.0,
    angler_who_finished_today: Vec::new(),
    saved_angler: false,
    angler_quest: 0,
    saved_stylist: false,
    saved_tax_collector: false,
    invasion_size_start: 0,
    temp_cultist_delay: 0,
    kill_count: Vec::new(),
    fast_forward_time: false,
    downed_fishron: false,
    downed_martians: false,
    downed_ancient_cultist: false,
    downed_moonlord: false,
    downed_halloween_king: false,
    downed_halloween_tree: false,
    downed_christmas_ice_queen: false,
    downed_christmas_ice_santank: false,
    downed_christmas_ice_tree: false,
    downed_tower_solar: false,
    downed_tower_vortex: false,
    downed_tower_nebula: false,
    downed_tower_stardust: false,
    active_tower_solar: false,
    active_tower_vortex: false,
    active_tower_nebula: false,
    active_tower_stardust: false,
    lunar_apocalypse_is_up: false,
    temp_party_manual: false,
    temp_party_genuine: false,
    temp_party_cooldown: false,
    temp_party_celebrating_NPCs: Vec::new(),
    temp_sandstorm_happening: false,
    temp_sandstorm_time_left: 0,
    temp_sandstorm_severity: 0.0,
    temp_sandstorm_intended_severity: 0.0,
    saved_bartender: false,
    downed_invastion_t1: false,
    downed_invastion_t2: false,
    downed_invastion_t3: false,
}*/

impl WorldHeader {
    pub fn from_buffer(posbuff: &mut PositionedBuffer) -> WorldHeader {
        WorldHeader {
            world_name: posbuff.read_pstring(),
            seed_text: posbuff.read_pstring(),
            world_generator_version: posbuff.read_u64(),
            world_unique_id: posbuff.read_u128(),
            world_id: posbuff.read_u32(),
            world_left: posbuff.read_u32(),
            world_right: posbuff.read_u32(),
            world_top: posbuff.read_u32(),
            world_bottom: posbuff.read_u32(),
            world_max_height: posbuff.read_u32(),
            world_max_width: posbuff.read_u32(),
            expert_mode: posbuff.read_bool(),
            creation_time: posbuff.read_u64(),
            moon_type: posbuff.read_u8(),
            tree_x0: posbuff.read_u32(),
            tree_x1: posbuff.read_u32(),
            tree_x2: posbuff.read_u32(),
            tree_style0: posbuff.read_u32(),
            tree_style1: posbuff.read_u32(),
            tree_style2: posbuff.read_u32(),
            tree_style3: posbuff.read_u32(),
            cave_back0: posbuff.read_u32(),
            cave_back1: posbuff.read_u32(),
            cave_back2: posbuff.read_u32(),
            cave_style0: posbuff.read_u32(),
            cave_style1: posbuff.read_u32(),
            cave_style2: posbuff.read_u32(),
            cave_style3: posbuff.read_u32(),
            ice_style: posbuff.read_u32(),
            jungle_style: posbuff.read_u32(),
            hell_style: posbuff.read_u32(),
            spawn_x: posbuff.read_u32(),
            spawn_y: posbuff.read_u32(),
            world_surface: posbuff.read_f64(),
            world_rock: posbuff.read_f64(),
            temp_time: posbuff.read_f64(),
            temp_day_time: posbuff.read_u8(),
            temp_moon_phase: posbuff.read_u32(),
            temp_blood_moon: posbuff.read_u8(),
            temp_eclipse: posbuff.read_u8(),
            dungeon_x: posbuff.read_u32(),
            dungeon_y: posbuff.read_u32(),
            crimson: posbuff.read_bool(),
            downed_boss1: posbuff.read_bool(),
            downed_boss2: posbuff.read_bool(),
            downed_boss3: posbuff.read_bool(),
            downed_queen_bee: posbuff.read_bool(),
            downed_mech_boss1: posbuff.read_bool(),
            downed_mech_boss2: posbuff.read_bool(),
            downed_mech_boss3: posbuff.read_bool(),
            downed_mech_boss_any: posbuff.read_bool(),
            downed_plant_boss: posbuff.read_bool(),
            downed_golem_boss: posbuff.read_bool(),
            downed_slime_king: posbuff.read_bool(),
            saved_goblin: posbuff.read_bool(),
            saved_wizard: posbuff.read_bool(),
            saved_mech: posbuff.read_bool(),
            downed_goblins: posbuff.read_bool(),
            downed_clown: posbuff.read_bool(),
            downed_frost: posbuff.read_bool(),
            downed_pirates: posbuff.read_bool(),
            shadow_orb_smashed: posbuff.read_bool(),
            spawn_meteor: posbuff.read_bool(),
            shadow_orb_count: posbuff.read_u8(),
            altar_count: posbuff.read_u32(),
            hard_mode: posbuff.read_bool(),
            invasion_delay: posbuff.read_u32(),
            invasion_size: posbuff.read_u32(),
            invasion_type: posbuff.read_u32(),
            invasion_x: posbuff.read_f64(),
            slime_rain_time: posbuff.read_f64(),
            sundial_cooldown: posbuff.read_u8(),
            temp_rain: posbuff.read_bool(),
            temp_rain_time: posbuff.read_u32(),
            temp_max_rain: posbuff.read_f32(),
            ore_tier1: posbuff.read_u32(),
            ore_tier2: posbuff.read_u32(),
            ore_tier3: posbuff.read_u32(),
            tree_bg: posbuff.read_u8(),
            corrupt_bg: posbuff.read_u8(),
            jungle_bg: posbuff.read_u8(),
            snow_bg: posbuff.read_u8(),
            hallow_bg: posbuff.read_u8(),
            crimson_bg: posbuff.read_u8(),
            desert_bg: posbuff.read_u8(),
            ocean_bg: posbuff.read_u8(),
            cloud_bgactive: posbuff.read_u32(),
            cloud_count: posbuff.read_u16(),
            wind_speed: posbuff.read_f32(),
            angler_who_finished_today: posbuff.read_list(
                &mut PositionedBuffer::read_pstring,
                &mut PositionedBuffer::read_u32,
            ),
            saved_angler: posbuff.read_bool(),
            angler_quest: posbuff.read_u32(),
            saved_stylist: posbuff.read_bool(),
            saved_tax_collector: posbuff.read_bool(),
            invasion_size_start: posbuff.read_u32(),
            temp_cultist_delay: posbuff.read_u32(),
            kill_count: posbuff.read_list(
                &mut PositionedBuffer::read_u32,
                &mut PositionedBuffer::read_u16,
            ),
            fast_forward_time: posbuff.read_bool(),
            downed_fishron: posbuff.read_bool(),
            downed_martians: posbuff.read_bool(),
            downed_ancient_cultist: posbuff.read_bool(),
            downed_moonlord: posbuff.read_bool(),
            downed_halloween_king: posbuff.read_bool(),
            downed_halloween_tree: posbuff.read_bool(),
            downed_christmas_ice_queen: posbuff.read_bool(),
            downed_christmas_ice_santank: posbuff.read_bool(),
            downed_christmas_ice_tree: posbuff.read_bool(),
            downed_tower_solar: posbuff.read_bool(),
            downed_tower_vortex: posbuff.read_bool(),
            downed_tower_nebula: posbuff.read_bool(),
            downed_tower_stardust: posbuff.read_bool(),
            active_tower_solar: posbuff.read_bool(),
            active_tower_vortex: posbuff.read_bool(),
            active_tower_nebula: posbuff.read_bool(),
            active_tower_stardust: posbuff.read_bool(),
            lunar_apocalypse_is_up: posbuff.read_bool(),
            temp_party_manual: posbuff.read_bool(),
            temp_party_genuine: posbuff.read_bool(),
            temp_party_cooldown: posbuff.read_u32(),
            temp_party_celebrating_npcs: posbuff.read_list(
                &mut PositionedBuffer::read_u32,
                &mut PositionedBuffer::read_u32,
            ),
            temp_sandstorm_happening: posbuff.read_bool(),
            temp_sandstorm_time_left: posbuff.read_u32(),
            temp_sandstorm_severity: posbuff.read_f32(),
            temp_sandstorm_intended_severity: posbuff.read_f32(),
            saved_bartender: posbuff.read_bool(),
            downed_invastion_t1: posbuff.read_bool(),
            downed_invastion_t2: posbuff.read_bool(),
            downed_invastion_t3: posbuff.read_bool(),
        }
    }

    pub fn empty() -> WorldHeader {
        WorldHeader {
            world_name: String::new(),
            seed_text: String::new(),
            world_generator_version: 0,
            world_unique_id: 0,
            world_id: 0,
            world_left: 0,
            world_right: 0,
            world_top: 0,
            world_bottom: 0,
            world_max_width: 0,
            world_max_height: 0,
            expert_mode: false,
            creation_time: 0,
            moon_type: 0,
            tree_x0: 0,
            tree_x1: 0,
            tree_x2: 0,
            tree_style0: 0,
            tree_style1: 0,
            tree_style2: 0,
            tree_style3: 0,
            cave_back0: 0,
            cave_back1: 0,
            cave_back2: 0,
            cave_style0: 0,
            cave_style1: 0,
            cave_style2: 0,
            cave_style3: 0,
            ice_style: 0,
            jungle_style: 0,
            hell_style: 0,
            spawn_x: 0,
            spawn_y: 0,
            world_surface: 0.0,
            world_rock: 0.0,
            temp_time: 0.0,
            temp_day_time: 0,
            temp_moon_phase: 0,
            temp_blood_moon: 0,
            temp_eclipse: 0,
            dungeon_x: 0,
            dungeon_y: 0,
            crimson: false,
            downed_boss1: false,
            downed_boss2: false,
            downed_boss3: false,
            downed_queen_bee: false,
            downed_mech_boss1: false,
            downed_mech_boss2: false,
            downed_mech_boss3: false,
            downed_mech_boss_any: false,
            downed_plant_boss: false,
            downed_golem_boss: false,
            downed_slime_king: false,
            saved_goblin: false,
            saved_wizard: false,
            saved_mech: false,
            downed_goblins: false,
            downed_clown: false,
            downed_frost: false,
            downed_pirates: false,
            shadow_orb_smashed: false,
            spawn_meteor: false,
            shadow_orb_count: 0,
            altar_count: 0,
            hard_mode: false,
            invasion_delay: 0,
            invasion_size: 0,
            invasion_type: 0,
            invasion_x: 0.0,
            slime_rain_time: 0.0,
            sundial_cooldown: 0,
            temp_rain: false,
            temp_rain_time: 0,
            temp_max_rain: 0.0,
            ore_tier1: 0,
            ore_tier2: 0,
            ore_tier3: 0,
            tree_bg: 0,
            corrupt_bg: 0,
            jungle_bg: 0,
            snow_bg: 0,
            hallow_bg: 0,
            crimson_bg: 0,
            desert_bg: 0,
            ocean_bg: 0,
            cloud_bgactive: 0,
            cloud_count: 0,
            wind_speed: 0.0,
            angler_who_finished_today: Vec::new(),
            saved_angler: false,
            angler_quest: 0,
            saved_stylist: false,
            saved_tax_collector: false,
            invasion_size_start: 0,
            temp_cultist_delay: 0,
            kill_count: Vec::new(),
            fast_forward_time: false,
            downed_fishron: false,
            downed_martians: false,
            downed_ancient_cultist: false,
            downed_moonlord: false,
            downed_halloween_king: false,
            downed_halloween_tree: false,
            downed_christmas_ice_queen: false,
            downed_christmas_ice_santank: false,
            downed_christmas_ice_tree: false,
            downed_tower_solar: false,
            downed_tower_vortex: false,
            downed_tower_nebula: false,
            downed_tower_stardust: false,
            active_tower_solar: false,
            active_tower_vortex: false,
            active_tower_nebula: false,
            active_tower_stardust: false,
            lunar_apocalypse_is_up: false,
            temp_party_manual: false,
            temp_party_genuine: false,
            temp_party_cooldown: 0,
            temp_party_celebrating_npcs: Vec::new(),
            temp_sandstorm_happening: false,
            temp_sandstorm_time_left: 0,
            temp_sandstorm_severity: 0.0,
            temp_sandstorm_intended_severity: 0.0,
            saved_bartender: false,
            downed_invastion_t1: false,
            downed_invastion_t2: false,
            downed_invastion_t3: false,
        }
    }

    pub fn new(name: String, x: u32, y: u32) -> WorldHeader {
        // Several values should be filled in after; they are marked
        WorldHeader {
            world_name: name,
            seed_text: String::new(),   // Might do nothing
            world_generator_version: 0, // No idea what this is for
            world_unique_id: 0,         // Think this one is important internally
            world_id: 0,                // Don't know how different than unique_id
            world_left: 0,
            world_right: x * 16,
            world_top: 0,
            world_bottom: y * 16,
            world_max_width: x,
            world_max_height: y,
            expert_mode: false, // Hard mode
            creation_time: 0,   // Easy enough
            moon_type: 0,
            tree_x0: 0, // Styles could be important
            tree_x1: 0,
            tree_x2: 0,
            tree_style0: 0,
            tree_style1: 0,
            tree_style2: 0,
            tree_style3: 0,
            cave_back0: 0,
            cave_back1: 0,
            cave_back2: 0,
            cave_style0: 0,
            cave_style1: 0,
            cave_style2: 0,
            cave_style3: 0,
            ice_style: 0,
            jungle_style: 0,
            hell_style: 0,
            spawn_x: 0,         // Spawn point
            spawn_y: 0,         // ..
            world_surface: 0.0, // Surface level
            world_rock: 0.0,    // Rock level
            temp_time: 0.0,     // Nice to set the time
            temp_day_time: 0,
            temp_moon_phase: 0,
            temp_blood_moon: 0,
            temp_eclipse: 0,
            dungeon_x: 0,   // Dungeon
            dungeon_y: 0,   // ..
            crimson: false, // Crimson or Corruption
            downed_boss1: false,
            downed_boss2: false,
            downed_boss3: false,
            downed_queen_bee: false,
            downed_mech_boss1: false,
            downed_mech_boss2: false,
            downed_mech_boss3: false,
            downed_mech_boss_any: false,
            downed_plant_boss: false,
            downed_golem_boss: false,
            downed_slime_king: false,
            saved_goblin: false,
            saved_wizard: false,
            saved_mech: false,
            downed_goblins: false,
            downed_clown: false,
            downed_frost: false,
            downed_pirates: false,
            shadow_orb_smashed: false,
            spawn_meteor: false,
            shadow_orb_count: 0,
            altar_count: 0,
            hard_mode: false, // Hard mode
            invasion_delay: 0,
            invasion_size: 0,
            invasion_type: 0,
            invasion_x: 0.0,
            slime_rain_time: 0.0,
            sundial_cooldown: 0,
            temp_rain: false,
            temp_rain_time: 0,
            temp_max_rain: 0.0,
            ore_tier1: 0, // This might be important
            ore_tier2: 0,
            ore_tier3: 0,
            tree_bg: 0, // backgrouds
            corrupt_bg: 0,
            jungle_bg: 0,
            snow_bg: 0,
            hallow_bg: 0,
            crimson_bg: 0,
            desert_bg: 0,
            ocean_bg: 0,
            cloud_bgactive: 0,
            cloud_count: 0,  // Number of floating clouds
            wind_speed: 0.0, // This is for particle effects
            angler_who_finished_today: Vec::new(),
            saved_angler: false,
            angler_quest: 0, // Might need to be set
            saved_stylist: false,
            saved_tax_collector: false,
            invasion_size_start: 0,
            temp_cultist_delay: 0,
            kill_count: vec![0u32; 580], // TODO: Fill Vecs
            fast_forward_time: false,
            downed_fishron: false,
            downed_martians: false,
            downed_ancient_cultist: false,
            downed_moonlord: false,
            downed_halloween_king: false,
            downed_halloween_tree: false,
            downed_christmas_ice_queen: false,
            downed_christmas_ice_santank: false,
            downed_christmas_ice_tree: false,
            downed_tower_solar: false,
            downed_tower_vortex: false,
            downed_tower_nebula: false,
            downed_tower_stardust: false,
            active_tower_solar: false,
            active_tower_vortex: false,
            active_tower_nebula: false,
            active_tower_stardust: false,
            lunar_apocalypse_is_up: false,
            temp_party_manual: false,
            temp_party_genuine: false,
            temp_party_cooldown: 0,
            temp_party_celebrating_npcs: Vec::new(),
            temp_sandstorm_happening: false,
            temp_sandstorm_time_left: 0,
            temp_sandstorm_severity: 0.0,
            temp_sandstorm_intended_severity: 0.0,
            saved_bartender: false,
            downed_invastion_t1: false,
            downed_invastion_t2: false,
            downed_invastion_t3: false,
        }
    }

    pub fn print(&self) {
        println!("{:?}", self);
    }

    pub fn get_tile_count(&self) -> u32 {
        self.world_max_width * self.world_max_height
    }

    pub fn write_to_file(&self, file: &mut std::fs::File) -> usize {
        file.write_string(self.world_name.clone());
        file.write_string(self.seed_text.clone());
        file.write_u64(self.world_generator_version);
        file.write_u128(self.world_unique_id);
        file.write_u32(self.world_id);
        file.write_u32(self.world_left);
        file.write_u32(self.world_right);
        file.write_u32(self.world_top);
        file.write_u32(self.world_bottom);
        file.write_u32(self.world_max_height);
        file.write_u32(self.world_max_width);
        file.write_bool(self.expert_mode);
        file.write_u64(self.creation_time);
        file.write_u8(self.moon_type);
        file.write_u32(self.tree_x0);
        file.write_u32(self.tree_x1);
        file.write_u32(self.tree_x2);
        file.write_u32(self.tree_style0);
        file.write_u32(self.tree_style1);
        file.write_u32(self.tree_style2);
        file.write_u32(self.tree_style3);
        file.write_u32(self.cave_back0);
        file.write_u32(self.cave_back1);
        file.write_u32(self.cave_back2);
        file.write_u32(self.cave_style0);
        file.write_u32(self.cave_style1);
        file.write_u32(self.cave_style2);
        file.write_u32(self.cave_style3);
        file.write_u32(self.ice_style);
        file.write_u32(self.jungle_style);
        file.write_u32(self.hell_style);
        file.write_u32(self.spawn_x);
        file.write_u32(self.spawn_y);
        file.write_f64(self.world_surface);
        file.write_f64(self.world_rock);
        file.write_f64(self.temp_time);
        file.write_u8(self.temp_day_time);
        file.write_u32(self.temp_moon_phase);
        file.write_u8(self.temp_blood_moon);
        file.write_u8(self.temp_eclipse);
        file.write_u32(self.dungeon_x);
        file.write_u32(self.dungeon_y);
        file.write_bool(self.crimson);
        file.write_bool(self.downed_boss1);
        file.write_bool(self.downed_boss2);
        file.write_bool(self.downed_boss3);
        file.write_bool(self.downed_queen_bee);
        file.write_bool(self.downed_mech_boss1);
        file.write_bool(self.downed_mech_boss2);
        file.write_bool(self.downed_mech_boss3);
        file.write_bool(self.downed_mech_boss_any);
        file.write_bool(self.downed_plant_boss);
        file.write_bool(self.downed_golem_boss);
        file.write_bool(self.downed_slime_king);
        file.write_bool(self.saved_goblin);
        file.write_bool(self.saved_wizard);
        file.write_bool(self.saved_mech);
        file.write_bool(self.downed_goblins);
        file.write_bool(self.downed_clown);
        file.write_bool(self.downed_frost);
        file.write_bool(self.downed_pirates);
        file.write_bool(self.shadow_orb_smashed);
        file.write_bool(self.spawn_meteor);
        file.write_u8(self.shadow_orb_count);
        file.write_u32(self.altar_count);
        file.write_bool(self.hard_mode);
        file.write_u32(self.invasion_delay);
        file.write_u32(self.invasion_size);
        file.write_u32(self.invasion_type);
        file.write_f64(self.invasion_x);
        file.write_f64(self.slime_rain_time);
        file.write_u8(self.sundial_cooldown);
        file.write_bool(self.temp_rain);
        file.write_u32(self.temp_rain_time);
        file.write_f32(self.temp_max_rain);
        file.write_u32(self.ore_tier1);
        file.write_u32(self.ore_tier2);
        file.write_u32(self.ore_tier3);
        file.write_u8(self.tree_bg);
        file.write_u8(self.corrupt_bg);
        file.write_u8(self.jungle_bg);
        file.write_u8(self.snow_bg);
        file.write_u8(self.hallow_bg);
        file.write_u8(self.crimson_bg);
        file.write_u8(self.desert_bg);
        file.write_u8(self.ocean_bg);
        file.write_u32(self.cloud_bgactive);
        file.write_u16(self.cloud_count);
        file.write_f32(self.wind_speed);
        file.write_list(
            &self.angler_who_finished_today,
            &mut PrimitiveWriting::write_string,
            &mut PrimitiveWriting::write_u32,
        );
        file.write_bool(self.saved_angler);
        file.write_u32(self.angler_quest);
        file.write_bool(self.saved_stylist);
        file.write_bool(self.saved_tax_collector);
        file.write_u32(self.invasion_size_start);
        file.write_u32(self.temp_cultist_delay);
        file.write_list(
            &self.kill_count,
            &mut PrimitiveWriting::write_u32,
            &mut PrimitiveWriting::write_u16,
        );
        file.write_bool(self.fast_forward_time);
        file.write_bool(self.downed_fishron);
        file.write_bool(self.downed_martians);
        file.write_bool(self.downed_ancient_cultist);
        file.write_bool(self.downed_moonlord);
        file.write_bool(self.downed_halloween_king);
        file.write_bool(self.downed_halloween_tree);
        file.write_bool(self.downed_christmas_ice_queen);
        file.write_bool(self.downed_christmas_ice_santank);
        file.write_bool(self.downed_christmas_ice_tree);
        file.write_bool(self.downed_tower_solar);
        file.write_bool(self.downed_tower_vortex);
        file.write_bool(self.downed_tower_nebula);
        file.write_bool(self.downed_tower_stardust);
        file.write_bool(self.active_tower_solar);
        file.write_bool(self.active_tower_vortex);
        file.write_bool(self.active_tower_nebula);
        file.write_bool(self.active_tower_stardust);
        file.write_bool(self.lunar_apocalypse_is_up);
        file.write_bool(self.temp_party_manual);
        file.write_bool(self.temp_party_genuine);
        file.write_u32(self.temp_party_cooldown);
        file.write_list(
            &self.temp_party_celebrating_npcs,
            &mut PrimitiveWriting::write_u32,
            &mut PrimitiveWriting::write_u32,
        );
        file.write_bool(self.temp_sandstorm_happening);
        file.write_u32(self.temp_sandstorm_time_left);
        file.write_f32(self.temp_sandstorm_severity);
        file.write_f32(self.temp_sandstorm_intended_severity);
        file.write_bool(self.saved_bartender);
        file.write_bool(self.downed_invastion_t1);
        file.write_bool(self.downed_invastion_t2);
        file.write_bool(self.downed_invastion_t3);
        file.current_pos()
    }
}
