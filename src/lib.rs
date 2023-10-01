mod image;
// mod bounds;
mod surfel;
pub use surfel::{Surfel, SurfelBuilder, SurfelModel, SurfelFusion, SurfelFusionParameters};
mod viz;
pub use viz::SurfelNode;
mod utils;
mod error;
pub use error::{SurfelReconsResult, SurfelReconstError};

#[cfg(test)]
mod unit_test;
