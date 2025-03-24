use sqlb::HasFields;
use sqlx::{FromRow, postgres::PgRow};

use super::ModelManager;
use crate::{
	ctx::Ctx,
	model::{Error, Result},
};
pub trait DbBmc {
	const TABLE: &'static str;
}

pub async fn create<MC, E>(_ctx: &Ctx, mm: &ModelManager, data: E) -> Result<i64>
where
	MC: DbBmc,
	E: HasFields,
{
	let db = mm.db();
	let field = data.not_none_fields();
	let (id,) = sqlb::insert()
		.table(MC::TABLE)
		.data(field)
		.returning(&["id"])
		.fetch_one(db)
		.await?;
	Ok(id)
}

pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where
	MC: DbBmc,
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send + HasFields,
{
	let db = mm.db();
	// let sql = format!("SELECT * FROM {} WHERE id = $1", MC::TABLE);

	let entity: E = sqlb::select()
		.table(MC::TABLE)
		.columns(E::field_names())
		.and_where("id", "=", id)
		.fetch_optional(db)
		.await?
		.ok_or(Error::EntityNotFound {
			entity: MC::TABLE,
			id,
		})?;

	Ok(entity)
}

pub async fn list<MC, E>(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<E>>
where
	MC: DbBmc,
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send + HasFields,
{
	let db = mm.db();

	let tasks: Vec<E> = sqlb::select()
		.table(MC::TABLE)
		.columns(E::field_names())
		.order_by("id")
		.fetch_all(db)
		.await?;

	Ok(tasks)
}

pub async fn update<MC, E>(
	_ctx: &Ctx,
	mm: &ModelManager,
	id: i64,
	data: E,
) -> Result<()>
where
	MC: DbBmc,
	E: HasFields,
{
	let db = mm.db();
	let field = data.not_none_fields();
	let count = sqlb::update()
		.table(MC::TABLE)
		.and_where("id", "=", id)
		.data(field)
		.exec(db)
		.await?;
	Ok(())
}
