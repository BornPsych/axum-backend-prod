// We create an entirely different setup for dev database becuase we don't want to mess with any prod db for test or staging, or we want quick iterations on our db, do quick nuking and generatoin of db, also stage and prod db dont have any seed values or something like this.

use std::{fmt::Error, fs, path::PathBuf, time::Duration};

use axum::extract::path;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tracing::info;

type Db = Pool<Postgres>;

// NOTE: Hardcode to prevent deployed system db update.
const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:welcome@localhost/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost/app_db";

// sql files
const SQL_RECREATE_DB: &str = "sql/dev_initial/00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_initial";

pub(crate) async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
	info!("{:<12} init_dev_db()", "FOR_DEV_ONLY");
	// Create the app_db/app_user with the postgres user.
	{
		let admin_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
		pexec(&admin_db, SQL_RECREATE_DB).await?;

		// We put this in it's own code block because
	}
	let app_db = new_db_pool(PG_DEV_APP_URL).await?;

	// -- Get sql files
	let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
		.filter_map(|entry| entry.ok().map(|e| e.path()))
		.collect();
	paths.sort();

	for path in paths {
		if let Some(path) = path.to_str() {
			let path = path.replace('\\', "/"); // for windows.

			// Only take the .sql and skip the SQL_RECREATE_DB
			if path.ends_with(".sql") && path != SQL_RECREATE_DB {
				pexec(&app_db, &path).await?;
			}
		}
	}

	Ok(())
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
	info!("{:<12} - pexec: {file}", "FOR-DEV-ONLY");

	// -- Read the file.
	let content = fs::read_to_string(file)?;

	let sqls: Vec<&str> = content.split(';').collect();

	for sql in sqls {
		sqlx::query(sql).execute(db).await?;
	}

	Ok(())
}

async fn new_db_pool(db_con_url: &str) -> Result<Db, sqlx::Error> {
	PgPoolOptions::new()
		.max_connections(1)
		.acquire_timeout(Duration::from_millis(1000))
		.connect(db_con_url)
		.await
}
