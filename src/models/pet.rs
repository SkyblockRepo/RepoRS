use std::collections::HashMap;

#[cfg(feature = "python")]
use pyo3::pyclass;
use serde::{Deserialize, Serialize};

use crate::UpgradeCost;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "python", pyclass)]
pub struct SkyblockPet {
	pub internal_id: String,
	pub name: Option<String>,
	pub category: Option<String>,
	pub source: Option<String>,
	pub min_level: u8,
	pub max_level: u8,
	pub base_stats: Vec<String>,
	pub pet_flags: Option<PetFlags>,
	pub rarities: HashMap<String, PetRarity>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "python", pyclass)]
pub struct PetFlags {
	pub auctionable: bool,
	pub mountable: bool,
	pub tradable: bool,
	pub museumable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "python", pyclass)]
pub struct PetRarity {
	pub lore: HashMap<String, String>,
	pub value: Option<f64>,
	pub kat_upgradeable: Option<bool>,
	#[serde(default)]
	pub kat_upgrade_costs: Vec<UpgradeCost>,
	pub kat_upgrade_seconds: Option<u32>,
	pub kat_upgrade_time: Option<String>,
}
