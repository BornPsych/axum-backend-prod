use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::base::{self, DbBmc};
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::{Fields, HasFields};
use sqlx::{FromRow, postgres::PgRow};
use uuid::Uuid;

// startregion --- User Type

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct User {
	pub id: i64,
	pub username: String,
}

#[derive(Deserialize)]
pub struct UserForCreate {
	pub username: String,
	pub pwd_clear: String,
}

#[derive(Fields)]
struct UserForInsert {
	username: String,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForLogin {
	pub id: i64,
	pub username: String,

	// -- pwd and token info
	pub pwd: Option<String>, // encrypted, #_schema_id_#...
	pub pwd_salt: Uuid,
	pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForAuth {
	pub id: i64,
	pub username: String,

	// -- token infos
	pub token_salt: Uuid,
}

/// Marker Traits
pub trait UserBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserForLogin {}
impl UserBy for UserForAuth {}

// endregion --- User Type

pub struct UserBmc;

impl DbBmc for UserBmc {
	const TABLE: &'static str = "user";
}

impl UserBmc {
	pub async fn get<E>(ctx: Ctx, mm: ModelManager, id: i64) -> Result<E>
	where
		E: UserBy,
	{
		base::get::<Self, E>(&ctx, &mm, id).await
	}

	pub async fn first_by_username<E>(
		ctx: Ctx,
		mm: ModelManager,
		username: &str,
	) -> Result<Option<E>>
	where
		E: UserBy,
	{
		let db = mm.db();
		let user = sqlb::select()
			.table(Self::TABLE)
			.and_where("username", "=", username)
			.fetch_optional::<_, E>(db)
			.await?;

		Ok(user)
	}

	pub async fn create(
		ctx: Ctx,
		mm: ModelManager,
		usre_c: UserForLogin,
	) -> Result<User> {
		todo!()
	}
}
