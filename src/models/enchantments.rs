use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::UpgradeCost;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkyblockEnchantment {
	pub internal_id: String,
	pub name: Option<String>,
	pub category: Option<String>,
	pub source: Option<String>,
	pub min_level: Option<u8>,
	pub max_level: Option<u8>,
	#[serde(default)]
	pub items: Vec<String>,
	pub pet_rarity: HashMap<String, PetRarity>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PetRarity {
	pub lore: HashMap<String, String>,
	pub value: Option<f64>,
	pub kat_upgradeable: Option<bool>,
	#[serde(default)]
	pub kat_upgrade_costs: Vec<UpgradeCost>,
	pub kat_upgrade_seconds: Option<u32>,
	pub kat_upgrade_time: Option<String>,
}
