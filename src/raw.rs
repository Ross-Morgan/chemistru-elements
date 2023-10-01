use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Deserialize, Serialize)]
pub struct RawElement {
    pub name: String,
    pub appearance: Option<String>,
    pub atomic_mass: f64,
    pub boil: Option<f64>,
    pub category: String,
    pub color: Option<String>,
    pub density: Option<f64>,
    pub melt: Option<f64>,
    pub molar_heat: Option<f64>,
    pub named_by: Option<String>,
    pub number: u8,
    pub period: u8,
    pub phase: String,
    pub source: String,
    pub spectral_img: Option<String>,
    pub summary: String,
    pub symbol: String,
    pub xpos: u8,
    pub ypos: u8,
    pub shells: Vec<u8>,
    pub electron_configuration: String,
    pub electron_configuration_semantic: String,
    pub electron_affinity: Option<f64>,
    pub electronegativity_pauling: Option<f64>,
    pub ionization_energies: Vec<f64>,
    pub cpk_hex: Option<String>,
}
