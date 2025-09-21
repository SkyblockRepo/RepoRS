#[cfg(feature = "python")]
use pyo3::pyclass;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::Coordinates;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "python", pyclass)]
pub struct SkyblockZone {
	#[serde(default)]
	pub internal_id: String,
	pub name: Option<String>,
	pub source: Option<String>,
	pub discovery_text: Option<String>,
	#[serde(default)]
	pub npcs: Vec<Value>,
	#[serde(default)]
	pub mobs: Vec<Value>,
	#[serde(default)]
	pub mob_drops: Vec<Value>,
	#[serde(default)]
	pub fairy_souls: Vec<FairySoul>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[cfg_attr(feature = "python", pyclass)]
pub struct FairySoul {
	pub location: Option<String>,
	#[serde(default)]
	pub number: i32,
	pub coordinates: Option<Coordinates>,
}
