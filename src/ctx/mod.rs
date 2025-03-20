// region:    --- Modules

mod error;

// Always re-export the errors because, it defined a cohesive API for other modules to import result and erros
pub use self::error::{Error, Result};

// endregion: --- Modules

#[derive(Clone, Debug)]
pub struct Ctx {
	user_id: i64,
}

// Constructor.
impl Ctx {
	pub fn root_ctx() -> Self {
		Ctx { user_id: 0 }
	}

	pub fn new(user_id: i64) -> Result<Self> {
		if user_id == 0 {
			Err(Error::CtxCannotNewRootCtx)
		} else {
			Ok(Self { user_id })
		}
	}
}

// Property Accessors.
impl Ctx {
	pub fn user_id(&self) -> i64 {
		self.user_id
	}
}

/*
	Context (Ctx) does not store mutable state because:

	Request-Specific Data:
	It only carries metadata like authentication details, request ID, and permissions.
	It is created per request and discarded after processing.
	Avoids Shared Mutable State:

	If Ctx held state, it could cause race conditions in concurrent environments.
	Keeping it immutable allows multiple requests to process simultaneously without interference.

*/
