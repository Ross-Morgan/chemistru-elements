use proc_macro2::{TokenStream, Group, Delimiter, TokenTree};
use quote::{TokenStreamExt, ToTokens, quote};
use serde::{Deserialize, Serialize};
use staticvec::StaticVec;

use crate::data::electron::orbital;

use super::data::prelude::*;

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize, Serialize)]
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
    pub ionization_energies: Vec<f64>,
    pub cpk_hex: Option<&'static str>,
}

impl RawElement {
    pub fn into_inner(self) -> InnerElement {
        let electron_configuration = self.electron_configuration
            .split(" ")
            .map(parse_suborbital);

        InnerElement {
            name: self.name,
            symbol: self.symbol,
            description: self.summary,
            atomic_data: AtomicData {
                atomic_number: self.number,
                nucleon_number: self.atomic_mass.round() as u16,
                atomic_mass: self.atomic_mass,
            },
            state_data: StateData {
                boiling_point: self.boil,
                melting_point: self.melt,
            },
            electron_data: ElectronData {
                electron_configuration: ElectronConfiguration(electron_configuration),
                ionisation_energies: StaticVec::new_from_slice(self.ionization_energies.as_slice()),
            },
        }
    }
}

fn parse_suborbital(s: &str) -> Box<dyn orbital::SubOrbital> {
    let mut chars = s.char_indices();

    let mut orbital_number = None;
    let mut suborbital_letter = None;
    let mut suborbital_fullness = 0u8;
    
    while let Some((idx, c)) = chars.next() {
        if c.is_digit(10) && suborbital_letter.is_none() {
            orbital_number = Some(c);
        } else if c.is_alphabetic() {
            suborbital_letter = Some(c);
        } else if c.is_digit(10) {
            suborbital_fullness *= 10;
            suborbital_fullness += c.to_digit(10).unwrap();
        }
    }

    if let (Some(number), Some(letter), Some(quantity)) = (orbital_number, suborbital_letter, suborbital_fullness) {
        match letter {
            's' => Box::new(orbital::SOrbital(quantity, number)),
            'p' => Box::new(orbital::POrbital(quantity, number)),
            'd' => Box::new(orbital::DOrbital(quantity, number)),
            'f' => Box::new(orbital::FOrbital(quantity, number)),
            _ => panic!("Invalid suborbital letter")
        }
    } else {
        panic!("Invalid orbital number, suborbital letter, or electron quantity");
    }
}
