pub mod home;
pub mod login;
pub mod register;

pub mod prelude {
    pub use super::home::Home;
    pub use super::login::Login;
    pub use super::register::Register;
}
