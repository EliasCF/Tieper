pub mod create;
pub use self::create::CreateStrategy;

pub mod remove;
pub use self::remove::RemoveStrategy;

pub mod start;
pub use self::start::StartStrategy;

pub mod stop;
pub use self::stop::StopStrategy;

pub mod list;
pub use self::list::ListStrategy;
