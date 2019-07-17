use crate::parsing::PositionedBuffer;

#[derive(Debug)]
pub struct WorldHeader {
    world_name: String,
    seed_text: String,
    world_generator_version: u64,
    world_unique_id: u128,
    world_id: u32,
    world_left: u32,
    world_right: u32,
    world_top: u32,
    world_bottom: u32,
    world_max_width: u32,
    world_max_height: u32,
    expert_mode: bool,
    creation_time: u64,
    moon_type: u8,
    tree_x0: u32,
    tree_x1: u32,
    tree_x2: u32,
    tree_style0: u32,
    tree_style1: u32,
    tree_style2: u32,
    tree_style3: u32,
    cave_back0: u32,
    cave_back1: u32,
    cave_back2: u32,
    cave_style0: u32,
    cave_style1: u32,
    cave_style2: u32,
    cave_style3: u32,
    ice_style: u32,
    jungle_style: u32,
    hell_style: u32,
    spawn_x: u32,
    spawn_y: u32,
    world_surface: f64,
    world_rock: f64,
    temp_time: f64,
    temp_day_time: u8,
    temp_moon_phase: u32,
    temp_blood_moon: u8,
    temp_eclipse: u8,
    dungeon_x: u32,
    dungeon_y: u32,
    crimson: bool,
    downed_boss1: bool,
    downed_boss2: bool,
    downed_boss3: bool,
    downed_queen_bee: bool,
    downed_mech_boss1: bool,
    downed_mech_boss2: bool,
    downed_mech_boss3: bool,
    downed_mech_boss_any: bool,
    downed_plant_boss: bool,
    downed_golem_boss: bool,
    downed_slime_king: bool,
    saved_goblin: bool,
    saved_wizard: bool,
    saved_mech: bool,
    downed_goblins: bool,
    downed_clown: bool,
    downed_frost: bool,
    downed_pirates: bool,
    shadow_orb_smashed: bool,
    spawn_meteor: bool,
    shadow_orb_count: u8,
    altar_count: u32,
    hard_mode: bool,
    invasion_delay: u32,
    invasion_size: u32,
    invasion_type: u32,
    invasion_x: f64,
    slime_rain_time: f64,
    sundial_cooldown: u8,
    temp_rain: bool,
    temp_rain_time: u32,
    temp_max_rain: f32,
    ore_tier1: u32,
    ore_tier2: u32,
    ore_tier3: u32,
    tree_bg: u8,
    corrupt_bg: u8,
    jungle_bg: u8,
    snow_bg: u8,
    hallow_bg: u8,
    crimson_bg: u8,
    desert_bg: u8,
    ocean_bg: u8,
    cloud_bgactive: u32,
    cloud_count: u16,
    wind_speed: f32,
    angler_who_finished_today: Vec<String>,
    saved_angler: bool,
    angler_quest: u32,
    saved_stylist: bool,
    saved_tax_collector: bool,
    invasion_size_start: u32,
    temp_cultist_delay: u32,
    kill_count: Vec<u32>,
    fast_forward_time: bool,
    downed_fishron: bool,
    downed_martians: bool,
    downed_ancient_cultist: bool,
    downed_moonlord: bool,
    downed_halloween_king: bool,
    downed_halloween_tree: bool,
    downed_christmas_ice_queen: bool,
    downed_christmas_ice_santank: bool,
    downed_christmas_ice_tree: bool,
    downed_tower_solar: bool,
    downed_tower_vortex: bool,
    downed_tower_nebula: bool,
    downed_tower_stardust: bool,
    active_tower_solar: bool,
    active_tower_vortex: bool,
    active_tower_nebula: bool,
    active_tower_stardust: bool,
    lunar_apocalypse_is_up: bool,
    temp_party_manual: bool,
    temp_party_genuine: bool,
    temp_party_cooldown: bool,
    temp_party_celebrating_npcs: Vec<u32>,
    temp_sandstorm_happening: bool,
    temp_sandstorm_time_left: u32,
    temp_sandstorm_severity: f32,
    temp_sandstorm_intended_severity: f32,
    saved_bartender: bool,
    downed_invastion_t1: bool,
    downed_invastion_t2: bool,
    downed_invastion_t3: bool,
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
            world_max_width: posbuff.read_u32(),
            world_max_height: posbuff.read_u32(),
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
            temp_party_cooldown: posbuff.read_bool(),
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

    pub fn print(&self) {
        println!("{:?}", self);
    }

}