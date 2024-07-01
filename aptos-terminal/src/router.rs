#[derive(Debug)]
pub enum Router {
    HOME,
    ACCOUNT,
    TRANSACTION,
}

impl Default for Router {
    fn default() -> Self {
        Router::HOME
    }
}
