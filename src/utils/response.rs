use actix_web::HttpResponse;

use crate::*;

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
pub enum ResponseType {
    AlreadyExists,
    Success,
    NotFound,
    ServerError,
    NoPermission,
    InvalidInput,
    Unauthorized,
    IncorrectPassword,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    #[serde(rename = "type")]
    response_type: ResponseType,
    message: Option<String>,
    data: Option<serde_json::Value>,
}

impl Response {
    pub fn new(response_type: ResponseType) -> Self {
        Self {
            response_type,
            message: None,
            data: None,
        }
    }

    pub fn already_exists() -> Self {
        Self {
            response_type: ResponseType::AlreadyExists,
            message: None,
            data: None,
        }
    }

    pub fn success() -> Self {
        Self {
            response_type: ResponseType::Success,
            message: None,
            data: None,
        }
    }

    pub fn not_found() -> Self {
        Self {
            response_type: ResponseType::NotFound,
            message: None,
            data: None,
        }
    }

    pub fn server_error() -> Self {
        Self {
            response_type: ResponseType::ServerError,
            message: None,
            data: None,
        }
    }

    pub fn no_permission() -> Self {
        Self {
            response_type: ResponseType::NoPermission,
            message: None,
            data: None,
        }
    }

    pub fn invalid_input() -> Self {
        Self {
            response_type: ResponseType::InvalidInput,
            message: None,
            data: None,
        }
    }

    pub fn unauthorized() -> Self {
        Self {
            response_type: ResponseType::Unauthorized,
            message: None,
            data: None,
        }
    }

    pub fn incorrect_password() -> Self {
        Self {
            response_type: ResponseType::IncorrectPassword,
            message: None,
            data: None,
        }
    }

    pub fn msg(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn data<T: serde::Serialize>(mut self, data: T) -> Self {
        self.data = Some(serde_json::to_value(data).unwrap());
        self
    }

    pub fn send(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}
