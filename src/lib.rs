#![deny(unused_extern_crates)]
#![warn(
    box_pointers,
    clippy::all,
    clippy::nursery,
    clippy::pedantic,
    future_incompatible,
    missing_copy_implementations,
    // missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_qualifications
)]

pub mod actions;
pub mod apps;
pub mod languages;
pub mod logger;
pub mod os_package_managers;

pub(crate) mod error;

pub use self::{error::Error, logger::Logger};
pub use dirs;

pub trait NewPluginManager {
    type PluginManager;

    fn new_plugin_manager(&self) -> Self::PluginManager;
}
