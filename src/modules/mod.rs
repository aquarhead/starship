pub mod aws;
pub mod cmd_duration;
pub mod directory;
pub mod git;
pub mod jj;
pub mod jobs;
pub mod kube;
pub mod plaio;
pub mod plaio_db;
pub mod prompt;
pub mod rust;

use crate::context::{Context, JJParent, Repo};
use crate::segment::Segment;
