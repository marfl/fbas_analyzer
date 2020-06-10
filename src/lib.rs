//! A library and tool for analyzing the quorum structure of federated byzantine agreement systems
//! (FBASs) like [Stellar](https://www.stellar.org/). Related research paper
//! [here](https://arxiv.org/abs/2002.08101).
//!
//! We recommend using the [`Analysis`](struct.Analysis.html) struct for doing analyses.
//!
//! # Example analysis
//! We will load a simple FBAS from the `test_data` folder. We will not use an organizations file;
//! analyses will be based on raw nodes.
//! ```
//! #[macro_use] extern crate fbas_analyzer; // for `bitset!` and `bitsetvec!`
//!
//! use fbas_analyzer::Fbas;
//! use fbas_analyzer::Analysis;
//!
//! let fbas = Fbas::from_json_file(std::path::Path::new("test_data/correct.json"));
//! let analysis = Analysis::new(&fbas, None);
//!
//! assert!(analysis.has_quorum_intersection());
//!
//! // "Unwrapping" analysis results gives us their internal representation, with node IDs
//! // corresponding to node indices in the input JSON.
//! assert_eq!(bitsetvec![{0,1},{0,10},{1,10}], analysis.minimal_blocking_sets().unwrap());
//! assert_eq!(bitsetvec![{0},{1},{10}], analysis.minimal_splitting_sets().unwrap());
//! assert_eq!(bitset!{0,1,10}, analysis.top_tier().unwrap());
//!
//! // You can also serialize results using serde.
//! assert_eq!(
//!     "[[0,1],[0,10],[1,10]]",
//!     serde_json::to_string(&analysis.minimal_blocking_sets()).unwrap()
//! );
//! ```

mod analysis;
mod core_types;
mod io;
mod shrinking;

pub use analysis::*;
pub use core_types::{Fbas, NodeId, NodeIdSet, Organizations, QuorumSet};
pub use io::AnalysisResult;

use core_types::*;
use shrinking::*;

use log::{debug, info, warn};

#[cfg(feature = "qsc-simulation")]
mod graph;
#[cfg(feature = "qsc-simulation")]
mod simulation;
#[cfg(feature = "qsc-simulation")]
pub use graph::Graph;
#[cfg(feature = "qsc-simulation")]
pub use simulation::{
    monitors, quorum_set_configurators, QuorumSetConfigurator, SimulationMonitor, Simulator,
};
