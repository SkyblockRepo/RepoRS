pub mod models;
mod utils;

use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[cfg(feature = "log")]
use log::{trace, warn};
use models::enchantment::SkyblockEnchantment;
use models::item::SkyblockItem;
use models::pet::SkyblockPet;
pub use models::{UpgradeCost, UpgradeType, enchantment, item, pet, recipe};
use rustc_hash::FxHashMap;
use serde::Deserialize;
pub use utils::{delete_repo, download_repo};

#[derive(Deserialize)]
struct RepoStructure {
	#[allow(dead_code)]
	version: u8,
	paths: HashMap<String, String>,
}

pub struct SkyblockRepo {
	pub enchantments: FxHashMap<String, SkyblockEnchantment>,
	pub items: FxHashMap<String, SkyblockItem>,
	pub pets: FxHashMap<String, SkyblockPet>,
}

impl SkyblockRepo {
	/// Creates HashMaps for each category
	///
	/// Throws warning log if it encounters a category it did not expect
	///
	/// Requires that the `SkyblockRepo` directory exists, which you can create via
	///
	/// ```rust
	/// skyblock_repo::download_repo(true);
	/// ```
	#[must_use]
	pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
		let structure: RepoStructure =
			serde_json::from_str(&fs::read_to_string("SkyblockRepo/manifest.json")?)?;

		let mut repo = Self {
			enchantments: FxHashMap::default(),
			items: FxHashMap::default(),
			pets: FxHashMap::default(),
		};

		for path_name in structure.paths.values() {
			let path = &format!("SkyblockRepo/{}", path_name);
			let path = Path::new(path);
			let data_entries = fs::read_dir(&path)?;

			for json in data_entries {
				let json = json?.path();
				#[cfg(feature = "log")]
				trace!("parsing {:?}", json);
				let content = fs::read_to_string(&json)?;

				match path_name.as_str() {
					| "enchantments" => {
						let parsed: SkyblockEnchantment = serde_json::from_str(&content)?;
						repo.enchantments.insert(parsed.internal_id.clone(), parsed);
					},
					| "items" => {
						let parsed: SkyblockItem = serde_json::from_str(&content)?;
						repo.items.insert(parsed.internal_id.clone(), parsed);
					},
					| "pets" => {
						let parsed: SkyblockPet = serde_json::from_str(&content)?;
						repo.pets.insert(parsed.internal_id.clone(), parsed);
					},
					#[cfg_attr(not(feature = "log"), allow(unused_variables))]
					| other => {
						#[cfg(feature = "log")]
						warn!("Unknown dir found while parsing SkyblockData: {}", other);
						continue;
					},
				}
			}
		}

		Ok(repo)
	}

	/// Retrieves an enchantment by its `internalId`
	#[must_use]
	#[inline]
	pub fn get_enchantment_by_id(
		&self,
		id: &str,
	) -> Option<&SkyblockEnchantment> {
		self.enchantments.get(id)
	}

	/// Retrieves an item by its `internalId`
	#[must_use]
	#[inline]
	pub fn get_item_by_id(
		&self,
		id: &str,
	) -> Option<&SkyblockItem> {
		self.items.get(id)
	}

	/// Retrieves a pet by its `internalId`
	#[must_use]
	#[inline]
	pub fn get_pet_by_id(
		&self,
		id: &str,
	) -> Option<&SkyblockPet> {
		self.pets.get(id)
	}
}
