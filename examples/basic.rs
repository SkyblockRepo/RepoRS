use skyblock_repo::{SkyblockRepo, delete_repo, download_repo};

#[tokio::main]
async fn main() {
	download_repo(true).await.unwrap();

	let data = SkyblockRepo::new().unwrap();

	println!("{:?}", data.get_enchantment_by_id("TELEKINESIS"));

	delete_repo().await.unwrap();
}
