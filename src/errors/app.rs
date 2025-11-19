#[derive(Debug)]
pub enum Error {
    // Other
    Internal,

    // User
    UserNotFound,
}

impl std::convert::From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        match err {
            // Other
            Error::Internal => crate::Error::Internal(String::new()),

            // User
            Error::UserNotFound => crate::Error::NotFound(String::from("User not found.")),
        }
    }
}
