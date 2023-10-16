use super::element::Element;

use std::cmp::PartialEq;

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.atomic_mass() == other.atomic_mass() && self.atomic_number() == other.atomic_number()
    }
}
