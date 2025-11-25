use std::rc::Rc;

use common::CELL_VOLUME;

use crate::gas::Gas;

#[derive(Debug)]
pub struct GasMixture {
    pub gas: Gas,
    archived_gas: Gas,

    pub temperature: f32,
    archived_temp: f32,

    pub volume: f32,
    pub group_multiplier: f32,

    pub graphic: Rc<str>,
    archived_graphic: Rc<str>,

    pub trace_gases: Vec<Gas>,
    pub fuel_burnt: f32,
}

impl Default for GasMixture {
    fn default() -> Self {
        Self {
            gas: Gas::default(),
            archived_gas: Gas::default(),

            temperature: 0.0,
            archived_temp: 0.0,

            volume: CELL_VOLUME,
            group_multiplier: 1.0,

            graphic: "".into(),
            archived_graphic: "".into(),

            trace_gases: vec![],
            fuel_burnt: 0.0,
        }
    }
}

impl GasMixture {
// Builders
    pub fn new() -> Self {GasMixture::default()}

    pub fn gas(mut self, gas: &Gas) -> Self {
        self.gas = gas.clone();
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }

    pub fn volume(mut self, volume: f32) -> Self {
        self.volume = volume;
        self
    }

    pub fn group_multiplier(mut self, multiplier: f32) -> Self {
        self.group_multiplier = multiplier;
        self
    }

    fn graphic(mut self, graphic: Rc<str>) -> Self {
        self.graphic = graphic;
        self
    }

    fn trace_gases(mut self, gases: Vec<Gas>) -> Self {
        self.trace_gases = gases;
        self
    }

    fn fuel_burnt(mut self, amount: f32) -> Self {
        self.fuel_burnt = amount;
        self
    }

// Archivers
    fn archive_gas(&mut self) {
        self.archived_gas = self.gas.clone();
    }

    fn archive_temperature(&mut self) {
        self.archived_temp = self.temperature;
    }

    fn archive_graphic(&mut self) {
        self.archived_graphic = self.graphic.clone();
    }

    pub fn archive(mut self) {
        self.archive_gas();
        self.archive_temperature();
        self.archive_graphic();
    }

// General functions

    pub fn total_moles(self) -> f32 {
        let mut total = self.gas.moles;
        for gas in self.trace_gases.iter() {
            total += gas.moles;
        }

        total
    }

    /// Perform all handeling required to clear out trace gases
    pub fn clear_trace_gases(mut self) {
        self.trace_gases.clear();

    }

    pub fn zero(mut self) {
        self.gas = self.gas.zero();
        self.clear_trace_gases();
    }

    /// Remove trace gas from a gas_mixture and handle clearing the trace_gases when applicable
    pub fn remove_trace_gas(mut self, trace_gas: &Gas) {
        let mut removed_gas = None;
        for (idx, gas) in self.trace_gases.iter_mut().enumerate() {
            if gas == trace_gas {
                removed_gas = Some(idx);
            }
        }

        if let Some(idx) = removed_gas {
            self.trace_gases.remove(idx);
        }
    }

    /// Create a gas for a gas mixture based on the gas name
    pub fn add_trace_gas_by_name(mut self, name: Rc<str>) -> Self {
        
        let trace_gas = Gas::new().name(name.clone());
        let mut exists = false;

        for gas in self.trace_gases.iter_mut() {
            if gas.name == name {
                exists = true;
                break
            }
        }

        if !exists {
            self.trace_gases.push(trace_gas.clone());
        } else {
            println!("ALREADY IN THERE DING DONG!")
        }
        self
    }

    /// Retrieve a gas by name
    pub fn get_trace_gas_by_name(mut self, name: Rc<str>) -> Option<Gas>{
        let mut exists = false;
        let mut trace_gas: Gas = Gas::new();

        for gas in self.trace_gases.iter_mut() {
            if gas.name == name {
                exists = true;
                trace_gas = gas.clone();
                break
            }
        }

        if exists {
            Some(trace_gas)
        } else {
            None
        }
    }

    pub fn check_tile_graphic() {
        // Stub
    }

    pub fn react() {
        // Stub
    }

    pub fn fire() {
        // Stub
    }

    pub fn check_then_merge() {
        // Stub
    }

    pub fn merge() {
        // Stub
    }

    pub fn remove() {
        // Stub
    }

    pub fn remove_ratio() {
        // Stub
    }

    pub fn copy_ratio() {
        // Stub
    }

    pub fn check_then_remove() {
        // Stub
    }

    pub fn copy_from() {
        // Stub
    }

    pub fn subtract() {
        // Stub
    }

    pub fn check_gas_mixture() {
        // Stub
    }

    pub fn check_turf() {
        // Stub
    }

    pub fn share() {
        // Stub
    }

    pub fn mimic() {
        // Stub
    }

    pub fn check_both_then_temperature_share() {
        // Stub
    }

    pub fn check_me_then_temperature_share() {
        // Stub
    }

    pub fn check_me_then_temperature_turf_share() {
        // Stub
    }

    pub fn check_me_then_temperature_mimic() {
        // Stub
    }

    pub fn temperature_share() {
        // Stub
    }

    pub fn temperature_mimic() {
        // Stub
    }

    pub fn temperature_turf_share() {
        // Stub
    }

    pub fn compare() {
        // Stub
    }

}