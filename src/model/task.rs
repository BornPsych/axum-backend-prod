use crate::model::ModelManager;
use crate::model::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region: -- Task Types

pub struct Task {
	pub id: i64,
	pub title: String,
}
// endregion: -- Task Types
