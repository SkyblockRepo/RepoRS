use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkyblockPet {
	pub internal_id: String,
	pub name: Option<String>,
	pub category: Option<String>,
	pub source: Option<String>,
	pub min_level: u8,
	pub max_level: u8,
	pub base_stats: Vec<String>,
	pub pet_flags: Option<PetFlags>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub struct PetFlags {
	pub auctionable: bool,
	pub mountable: bool,
	pub tradable: bool,
	pub museumable: bool,
}
