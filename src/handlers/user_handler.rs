use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Credentials do not match our records")]
    BadCredientials(String),

    #[error("Error while saving user")]
    Registeration(String),

    #[error("Error while getting user")]
    Fetching(String),

    #[error("Error while deactivating user")]
    DeActivation(String),

    #[error("Error while checking user activity")]
    ActivationCheck(String),
}
