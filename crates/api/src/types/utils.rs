pub enum ResponseStatus {
    Ok,
    BadRequest,
    Unauthorized,
    NotFound,
    InternalServerError,
}

impl ResponseStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResponseStatus::Ok => "Ok",
            ResponseStatus::BadRequest => "Bad Request",
            ResponseStatus::Unauthorized => "Unauthorized",
            ResponseStatus::NotFound => "Not Found",
            ResponseStatus::InternalServerError => "Internal Server Error",
        }
    }
}
