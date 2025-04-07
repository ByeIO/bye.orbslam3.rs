#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_fmt_panics)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]
#![allow(unsafe_code)]
#![allow(clippy::undocumented_unsafe_blocks)]
#![allow(unused_must_use)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]

use std::collections::LinkedList;
use std::fs::File;
use std::io::{self, BufWriter, Write, Read, BufReader};
use std::path::Path;
use std::rc::Rc;
use std::cell::{RefMut, Ref, RefCell};
use base64::{engine::general_purpose, Engine as _};

use crate::Types::{Point3d, Pose6d, Point3dCollection, Point3dList, OcTreeVolume, Vector3, Quaternion, Pose6D};
use crate::PointCloud::Pointcloud;
use crate::Utils::{
    deg_to_rad, rad_to_deg, logodds, probability,
};
use crate::ScanGraph::{
    ScanNode, ScanEdge, ScanGraph,
};

