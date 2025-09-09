use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkyblockRecipe {
	pub name: Option<String>,
	pub r#type: RecipeType,
	pub result_id: Option<String>,
	pub result_quantity: i32,
	pub crafting: HashMap<String, RecipeIngredient>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeIngredient {
	pub item_id: String,
	pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum RecipeType {
	#[default]
	Crafting,
}
