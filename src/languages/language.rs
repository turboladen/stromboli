pub mod ruby;

pub use ruby::Ruby;

pub trait Language {
    const NAME: &'static str;
}
