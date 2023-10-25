use serde::{Deserialize, Serialize};

use crate::data::electron::orbital;
use crate::inner::InnerElement;

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
    pub summary: String,
    pub symbol: &'static str,
    pub xpos: u8,
    pub ypos: u8,
    pub shells: Vec<u8>,
    pub electron_configuration: &'static str,
    pub electron_configuration_semantic: &'static str,
    pub electron_affinity: Option<f64>,
    pub electronegativity_pauling: Option<f64>,
    pub ionization_energies: Vec<f64>,
    pub cpk_hex: Option<&'static str>,
}

impl RawElement {
    pub fn into_inner(self) -> InnerElement {
        let sub_orbitals = self.electron_configuration.split(" ").map(parse_suborbital).collect::<Vec<_>>();

        let mut electron_configuration = [1u8, 2, 3, 4, 5, 6]
            .map(|i| Orbital(
                SOrbital(i, 0),
                POrbital(i, 0),
                DOrbital(i, 0),
                FOrbital(i, 0),
            ));

        sub_orbitals.iter().for_each(|so| match so.capacity() {
            2  => electron_configuration[so.orbital_number() as usize -  1].0.1 = so.electrons(),
            6  => electron_configuration[so.orbital_number() as usize -  1].1.1 = so.electrons(),
            10 => electron_configuration[so.orbital_number() as usize -  1].2.1 = so.electrons(),
            14 => electron_configuration[so.orbital_number() as usize -  1].3.1 = so.electrons(),
            cap => panic!("Invalid suborbital capacity [{cap}]"),
        });

        let electron_configuration = ElectronConfiguration(electron_configuration);

        let mut ionisation_energies = [0.0f64; 30];

        let s = ionisation_energies.as_mut_slice();

        for (idx, &item) in self.ionization_energies.iter().enumerate() {
            s[idx] = item;
        }

        let mut shells = [0u8; 8];

        let s = shells.as_mut_slice();

        for (idx, &item) in self.shells.iter().enumerate() {
            s[idx] = item;
        }

        InnerElement {
            name: self.name,
            symbol: self.symbol,
            description: Box::leak(self.summary.into_boxed_str()),
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
                electron_configuration,
                ionisation_energies,
                shells,
                electron_affinity: self.electron_affinity,
                electronegativity: self.electronegativity_pauling,
            },
        }
    }
}

pub fn parse_suborbital(s: &str) -> Box<dyn orbital::SubOrbital> {
    let mut chars = s.chars();

    let mut orbital_number = None;
    let mut suborbital_letter = None;
    let mut suborbital_fullness = 0u8;
    
    while let Some(c) = chars.next() {
        if c.is_digit(10) && suborbital_letter.is_none() {
            orbital_number = Some(c);
        } else if c.is_alphabetic() {
            suborbital_letter = Some(c);
        } else if c.is_digit(10) {
            suborbital_fullness *= 10;
            suborbital_fullness += c.to_digit(10).unwrap() as u8;
        }
    }

    if let (Some(number), Some(letter), quantity) = (orbital_number, suborbital_letter, suborbital_fullness) {
        match letter {
            's' => Box::new(orbital::SOrbital(number.to_digit(10).unwrap() as u8, quantity)),
            'p' => Box::new(orbital::POrbital(number.to_digit(10).unwrap() as u8, quantity)),
            'd' => Box::new(orbital::DOrbital(number.to_digit(10).unwrap() as u8, quantity)),
            'f' => Box::new(orbital::FOrbital(number.to_digit(10).unwrap() as u8, quantity)),
            _ => panic!("Invalid suborbital letter")
        }
    } else {
        panic!("Invalid orbital number, suborbital letter, or electron quantity");
    }
}
