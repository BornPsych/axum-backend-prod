#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: u64,
}

// Constructor
impl Ctx {
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }
}

// Property accessors
impl Ctx {
    pub fn user_id(&self) -> u64 {
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