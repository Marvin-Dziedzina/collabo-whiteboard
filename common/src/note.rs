use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, PartialEq, Serialize, Deserialize)]
#[require(Transform, Text2d, Mesh2d, MeshMaterial2d<ColorMaterial>)]
pub struct Note;
