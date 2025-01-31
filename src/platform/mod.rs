pub(crate) trait WindowLike
{
	fn open(&self);
}

#[cfg(feature = "desktop")]
pub mod desktop;

#[cfg(feature = "desktop")]
pub use desktop::*;
#[cfg(feature = "web")]
pub mod web;

#[cfg(feature = "web")]
pub use web::*;