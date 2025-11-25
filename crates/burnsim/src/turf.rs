pub struct Hotspot;

pub struct MutableAppearance (String, String, String);

pub struct Turf {
    pub pressure_difference: f32,
    pub pressure_direction: f32,
    pub active_hotspot: Hotspot,

    pub max_process_cell_operations: f32,

    pub atmos_operations: f32,
    pub max_atmos_operations: f32,

    pub gas_overlays: Vec<MutableAppearance>,

    pub dist_to_space: f32,

    
}