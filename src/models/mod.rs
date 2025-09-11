#[cfg(feature = "python")]
use pyo3::pyclass;
use serde::{Deserialize, Serialize};

pub mod enchantment;
pub mod item;
pub mod pet;
pub mod recipe;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[cfg_attr(feature = "python", pyclass)]
pub struct UpgradeCost {
	pub r#type: Option<UpgradeType>,
	pub item_id: Option<String>,
	pub essence_type: Option<String>,
	pub amount: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "python", pyclass)]
pub enum UpgradeType {
	Item,
	Essence,
	Coins,
}
