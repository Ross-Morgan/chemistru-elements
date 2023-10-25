pub mod atomic;
pub mod electron;
pub mod state;

pub mod prelude {
    use super::*;

    pub use atomic::AtomicData;
    pub use electron::{ElectronConfiguration, ElectronData, orbital::*};
    pub use state::StateData;
}
