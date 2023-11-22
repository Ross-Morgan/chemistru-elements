use crate::{
    element::{Element, Elemental},
    inner::InnerElement,
};

pub struct Ion {
    pub element: Element,
    pub oxidation_state: i8,
}

impl Elemental for Ion {
    fn name(&self) -> String {
        self.element.name()
    }

    fn symbol(&self) -> String {
        self.element.symbol()
    }

    fn atomic_mass(&self) -> f64 {
        self.element.atomic_mass()
    }

    fn atomic_number(&self) -> u8 {
        self.element.atomic_number()
    }

    fn data(&self) -> &'static InnerElement {
        self.element.data()
    }
}
