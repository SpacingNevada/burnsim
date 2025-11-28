//#![allow(unused_assignments)]

use std::rc::Rc;

use common::{
    CELL_VOLUME,
    MINIMUM_REACT_QUANTITY,
    PLASMA_MINIMUM_BURN_TEMPERATURE,
    PLASMA_OXYGEN_FULLBURN,
    PLASMA_UPPER_TEMPERATURE,
    quantize,
    FIRE_PLASMA_ENERGY_RELEASED,
    MINIMUM_HEAT_CAPACITY,
    FIRE_MINIMUM_TEMPERATURE_TO_EXIST,
    MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER,
    MINIMUM_AIR_TO_SUSPEND};

use crate::gas::Gas;

pub struct GasResult(Option<usize>);

impl ::core::ops::Deref for GasResult {
    type Target = Option<usize>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct GasMixture {
    pub reaction_active: bool,
    pub combustion_active: bool,

    pub gases: Vec<Gas>,
    archived_gases: Vec<Gas>,

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
            reaction_active: false,
            combustion_active: false,

            gases: vec!(Gas::default()),
            archived_gases: vec!(Gas::default()),

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

    pub fn gases(mut self, gas: Vec<Gas>) -> Self {
        self.gases = gas;
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
        self.archived_gases = self.gases.clone();
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
    pub fn zero(mut self) {
        for gas in self.gases.iter_mut() {
            gas.moles = 0.0;
            gas.specific_heat = 0.0;
        }
        
        self.clear_trace_gases();
    }

    /// Perform all handeling required to clear out trace gases
    pub fn clear_trace_gases(mut self) {
        self.trace_gases.clear();

    }

    /// Remove trace gas from a gas_mixture and handle clearing the trace_gases when applicable
    pub fn remove_trace_gas(mut self, trace_gas: &Gas) {
        let gas_result = self.has_trace_gas(trace_gas.name.clone());

        if let Some(gas_idx) = gas_result {
            self.trace_gases.remove(gas_idx);
        } else {
            println!("No such gas!");
        }

    }

    /// Create a gas for a gas mixture based on the gas name
    pub fn add_trace_gas_by_name(&mut self, name: Rc<str>) {
        let gas_result = self.has_trace_gas(name.clone());

        if !gas_result.is_some() {
            let trace_gas = Gas::new().name(name.clone());
            self.trace_gases.push(trace_gas.clone());

        } else {
            println!("ALREADY IN THERE DING DONG!");

        }
    }

    /// Retrieve a gas by name
    pub fn get_trace_gas_by_name(self, name: Rc<str>) -> Option<Gas>{
        let gas_result = self.has_trace_gas(name);

        if let Some(gas_idx) = gas_result {
            Some(self.trace_gases[gas_idx].clone())
        } else {
            None
        }
    }

    pub fn check_tile_graphic() {
        unimplemented!()
    }

    pub fn react(&mut self) {

        let tox_result = self.has_gas("toxins".into());
        let carb_result = self.has_gas("carbon_dioxide".into());
        let fart_result = self.has_gas("farts".into());
        
        if self.temperature > 900.0 &&
            let Some(fart_idx) = fart_result &&
            let Some(tox_idx) = tox_result &&
            let Some(carb_idx) = carb_result
        {
                let tox_moles = self.gases[tox_idx].moles;
                let carb_moles = self.gases[carb_idx].moles;
                let fart_moles = self.gases[fart_idx].moles;

                if fart_moles > MINIMUM_REACT_QUANTITY &&
                    tox_moles > MINIMUM_REACT_QUANTITY &&
                    carb_moles > MINIMUM_REACT_QUANTITY
                {
                    let mut reaction_rate = (carb_moles*0.75).min((tox_moles*0.25).min(fart_moles*0.05));
                    reaction_rate = quantize(reaction_rate);

                    self.gases[carb_idx].moles -= reaction_rate;
                    self.gases[tox_idx].moles += reaction_rate;
                    self.gases[fart_idx].moles -= reaction_rate*0.05;

                    self.temperature += (reaction_rate*10000.0)/self.heat_capacity_full().clone();
                    self.combustion_active = true;
                }
        }

        self.fuel_burnt = 0.0;
        if self.temperature > FIRE_MINIMUM_TEMPERATURE_TO_EXIST {
            if self.fire() > 0.0 {
                self.combustion_active = true;
            }
        }

        }

    pub fn fire(&mut self) -> f32 {
        let mut energy_released = 0.0;
        let old_heat_capacity = self.heat_capacity_full().clone();
        let tox_result = self.has_gas("toxins".into());
        if let Some(tox_idx) = tox_result { 
            let tox_moles = self.gases[tox_idx].moles;
            if tox_moles > MINIMUM_REACT_QUANTITY {
                let mut plasma_burn_rate = 0.0;
                let mut temperature_scale = 0.0;
                if self.temperature > PLASMA_UPPER_TEMPERATURE {
                    temperature_scale = 1.0;
                } else {
                    temperature_scale = self.temperature - PLASMA_MINIMUM_BURN_TEMPERATURE;
                }
                if temperature_scale > 0.0 {
                    let oxygen_burn_rate = 1.4 - temperature_scale;
                    let oxy_result = self.has_gas("oxygen".into());
                    if let Some(oxy_idx) = oxy_result {
                        let oxy_moles = self.gases[oxy_idx].moles;
                        if oxy_moles > (tox_moles * PLASMA_OXYGEN_FULLBURN) {
                            plasma_burn_rate = (tox_moles * temperature_scale)/4.0;
                        } else {
                            plasma_burn_rate = temperature_scale * (oxy_moles / PLASMA_OXYGEN_FULLBURN)/4.0;
                        }
                        let carb_result = self.has_gas("carbon_dioxide".into());
                        if plasma_burn_rate > MINIMUM_REACT_QUANTITY {
                            self.gases[tox_idx].moles -= quantize(plasma_burn_rate/3.0);
                            self.gases[oxy_idx].moles -= quantize(plasma_burn_rate*oxygen_burn_rate);
                            if let Some(carb_idx) = carb_result {
                                self.gases[carb_idx].moles += quantize(plasma_burn_rate/3.0);
                            }
                            energy_released += FIRE_PLASMA_ENERGY_RELEASED * plasma_burn_rate;
                            self.fuel_burnt += plasma_burn_rate * (1.0 + oxygen_burn_rate);
                        }
                    }
                }
            }
        }
        if energy_released > 0.0 {
            let new_heat_capacity = self.heat_capacity_full().clone();
            if new_heat_capacity > MINIMUM_HEAT_CAPACITY {
                self.temperature = (self.temperature * old_heat_capacity + energy_released) / new_heat_capacity;
            }
        }
        self.fuel_burnt
    }

    pub fn check_then_merge(mut self, giver: GasMixture) {
        self.merge(giver);
    }

    // Merges all gas from giver into self.
    pub fn merge(&mut self, giver: GasMixture) {
        if (self.temperature - giver.temperature).abs() > MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER {
            let self_heat_capacity = self.heat_capacity_full()*self.group_multiplier;
            let giver_heat_capacity = giver.heat_capacity_full()*giver.group_multiplier;
            let combined_heat_capacity = self_heat_capacity + giver_heat_capacity;

            if combined_heat_capacity != 0.0 {
                self.temperature = (giver.temperature*giver_heat_capacity + self.temperature*self_heat_capacity)/combined_heat_capacity;
            }
        }

        for gas in giver.gases.iter() {
            let gas_result = self.has_gas(gas.name.clone());
            if let Some(gas_idx) = gas_result {
                if self.group_multiplier > 1.0 || giver.group_multiplier > 1.0 {
                    self.gases[gas_idx].moles += gas.moles*giver.group_multiplier/self.group_multiplier;
                    self.gases[gas_idx].specific_heat += gas.specific_heat*giver.group_multiplier/self.group_multiplier;
                } else {
                    self.gases[gas_idx].moles += gas.moles;
                    self.gases[gas_idx].specific_heat += gas.specific_heat;
                }
            }
        }

        if !giver.trace_gases.is_empty() {
            for gas in giver.trace_gases.iter() {
                let gas_result = self.has_trace_gas(gas.name.clone());
                if let Some(gas_idx) = gas_result {
                    self.trace_gases[gas_idx].moles += gas.moles;
                } else {
                    self.add_trace_gas_by_name(gas.name.clone());
                    let gas_result = self.has_trace_gas(gas.name.clone());
                    self.trace_gases[gas_result.unwrap()].moles += gas.moles;
                }
            }
        }
    }

    pub fn remove(&mut self, amount: f32) -> GasMixture {
        let sum = self.total_moles();
        let mut removed = self.clone();

        for (idx, gas) in self.gases.iter_mut().enumerate() {
            removed.gases[idx].moles = quantize((gas.moles/sum)*amount).min(gas.moles);
            gas.moles -= removed.gases[idx].moles/self.group_multiplier;
        }

        removed
    }

    pub fn remove_ratio() {
        unimplemented!()
    }

    pub fn copy_ratio() {
        unimplemented!()
    }

    pub fn check_then_remove() {
        unimplemented!()
    }

    pub fn copy_from() {
        unimplemented!()
    }

    pub fn subtract() {
        unimplemented!()
    }

    pub fn check_gas_mixture() {
        unimplemented!()
    }

    pub fn check_turf() {
        unimplemented!()
    }

    pub fn share() {
        unimplemented!()
    }

    pub fn mimic() {
        unimplemented!()
    }

    pub fn check_both_then_temperature_share() {
        unimplemented!()
    }

    pub fn check_me_then_temperature_share() {
        unimplemented!()
    }

    pub fn check_me_then_temperature_turf_share() {
        unimplemented!()
    }

    pub fn check_me_then_temperature_mimic() {
        unimplemented!()
    }

    pub fn temperature_share() {
        unimplemented!()
    }

    pub fn temperature_mimic() {
        unimplemented!()
    }

    pub fn temperature_turf_share() {
        unimplemented!()
    }

    pub fn compare() {
        unimplemented!()
    }

// Other [Things I've had to kinda make up because wtf byond]
    pub fn total_moles(&self) -> f32 {
        let mut total = 0.0;
        for gas in self.gases.iter() {
            total += gas.moles;
        }
        for gas in self.trace_gases.iter() {
            total += gas.moles;
        }
        total
    }

    pub fn heat_capacity_full(&self) -> f32 {
        let mut total = 0.0;
        for gas in self.gases.iter() {
            total += gas.moles * gas.specific_heat;
        }

        for gas in self.trace_gases.iter() {
            total += gas.moles * gas.specific_heat;
        }

        total
    }

    pub fn has_gas(&self, name: Rc<str>) -> Option<usize> {
        let mut result = None;

        for (idx, gas) in self.gases.iter().enumerate() {
            if gas.name == name {
                result = Some(idx);
                break
            }
        }

        result
    }

    pub fn has_trace_gas(&self, name: Rc<str>) -> Option<usize> {
        let mut result = None;

        for (idx, gas) in self.trace_gases.iter().enumerate() {
            if gas.name == name {
                result = Some(idx);
                break
            }
        }

        result
    }

}