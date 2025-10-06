pub type PoiseResult<T> = std::result::Result<T, PoiseError>;
pub type PoiseError = Box<dyn std::error::Error + Send + Sync>;
pub type PoiseContext<'a> = poise::Context<'a, Data, PoiseError>;

// To be implemented later, yet necessary for poise
pub struct Data {}