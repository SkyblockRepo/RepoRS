pub mod models;
mod utils;

use std::fs;

use log::{trace, warn};
use models::enchantment::SkyblockEnchantment;
use models::item::SkyblockItem;
use models::pet::SkyblockPet;
pub use models::{UpgradeCost, UpgradeType, enchantment, item, pet, recipe};
use rustc_hash::FxHashMap;
pub use utils::repo::download_zip as download_repo;

pub struct SkyblockRepo {
	pub enchantments: FxHashMap<String, SkyblockEnchantment>,
	pub items: FxHashMap<String, SkyblockItem>,
	pub pets: FxHashMap<String, SkyblockPet>,
}

impl SkyblockRepo {
	pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
		let entries = fs::read_dir("SkyblockRepo")?;

		let mut repo = Self {
			enchantments: FxHashMap::default(),
			items: FxHashMap::default(),
			pets: FxHashMap::default(),
		};

		for repo_entry in entries {
			let path = repo_entry?.path();

			if !path.is_dir() {
				continue;
			}

			if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
				let data_entries = fs::read_dir(&path)?;

				for json in data_entries {
					let path = path.join(json?.path());
					trace!("{:?}", path);
					let content = fs::read_to_string(path)?;

					match dir_name {
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
						| other => {
							warn!("Unknown dir found while parsing SkyblockData: {}", other);
							continue;
						},
					}
				}
			}
		}

		Ok(repo)
	}

	pub fn get_enchantment_by_id(
		&self,
		id: &str,
	) -> Option<&SkyblockEnchantment> {
		self.enchantments.get(id)
	}

	pub fn get_item_by_id(
		&self,
		id: &str,
	) -> Option<&SkyblockItem> {
		self.items.get(id)
	}

	pub fn get_pet_by_id(
		&self,
		id: &str,
	) -> Option<&SkyblockPet> {
		self.pets.get(id)
	}
}
