// Modified imports for tract.rs
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
#[cfg(feature = "timings")]
use std::time::Instant;

use anyhow::{bail, Context, Result};
use flate2::read::GzDecoder;
use ini::Ini;
use ndarray::{prelude::*, Axis};
use tar::Archive;
use tract_core::internal::tract_itertools::izip;
use tract_core::internal::tract_smallvec::alloc::collections::VecDeque;
use tract_core::ops;
use tract_core::prelude::*;
use tract_onnx::{prelude::*, tract_hir::shapefactoid};
use tract_pulse::{internal::ToDim, model::*};

use crate::*;

// Rest of the file would be copied from tract.rs with ini::Properties replaced by rust_ini::Properties