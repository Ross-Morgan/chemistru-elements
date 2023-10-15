use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct RawElement {
    pub name: &'static str,
    pub appearance: Option<&'static str>,
    pub atomic_mass: f64,
    pub boil: Option<f64>,
    pub category: &'static str,
    pub color: Option<&'static str>,
    pub density: Option<f64>,
    pub melt: Option<f64>,
    pub molar_heat: Option<f64>,
    pub named_by: Option<&'static str>,
    pub number: u8,
    pub period: u8,
    pub phase: &'static str,
    pub source: &'static str,
    pub spectral_img: Option<&'static str>,
    pub summary: &'static str,
    pub symbol: &'static str,
    pub xpos: u8,
    pub ypos: u8,
    pub shells: &'static [u8],
    pub electron_configuration: &'static str,
    pub electron_configuration_semantic: &'static str,
    pub electron_affinity: Option<f64>,
    pub electronegativity_pauling: Option<f64>,
    pub ionization_energies: &'static [Option<f64>; 30],
    pub cpk_hex: Option<&'static str>,
}

impl RawElement {
    pub fn into_inner(self) -> InnerElement {
        InnerElement {
            name: self.name,
            symbol: self.symbol,
            atomic_data: AtomicData { atomic_number: self.number, nucleon_number: self.atomic_mass.round() as u16, atomic_mass: self.atomic_mass },
            state_data: StateData { boiling_point: self.boil, melting_point: self.melt },
            ion_data: IonData { electron_configuration: ElectronConfiguration }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct InnerElement {
    pub name: &'static str,
    pub symbol: &'static str,
    pub atomic_data: AtomicData,
    pub state_data: StateData,
    pub ion_data: IonData,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct AtomicData {
    pub atomic_number: u8,
    pub nucleon_number: u16,
    pub atomic_mass: f64,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct StateData {
    pub boiling_point: Option<f64>,
    pub melting_point: Option<f64>,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct IonData {
    pub electron_configuration: ElectronConfiguration,
    pub ionisation_energies: Vec<f64>,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ElectronConfiguration {
    
}

impl ElectronConfiguration {
    pub fn to_semantic_string(&self) -> String {
        
    }
}