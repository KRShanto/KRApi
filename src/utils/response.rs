use actix_web::HttpResponse;

use crate::*;

/// JSON response types
///
/// These are the types of responses that the server can send to the client.
#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
pub enum ResponseType {
    /// The resource already exists
    AlreadyExists,
    /// The request was successful
    Success,
    /// The resource was not found
    NotFound,
    /// There was an error on the server
    ServerError,
    /// The user does not have permission to access the resource
    NoPermission,
    /// The input was invalid
    InvalidInput,
    /// The user is unauthorized
    Unauthorized,
    /// The password is incorrect
    IncorrectPassword,
}

/// The response struct that is sent to the client
///
/// Every response has a type, a message, and data.
///
/// On the client side you need to use `res.json().data` to get the data.
///
/// ## Example
///
/// Build a response and send it to the client:
///
/// ```
/// # use krapi::Response;
/// # use serde::{Deserialize, Serialize};
/// #
/// #[derive(Serialize, Deserialize, Debug)]
/// pub struct User {
///    pub id: i32,
///    pub name: String,
/// }
///
/// let data = User {
///     id: 1,
///     name: "Shanto Islam".to_string(),
/// };
///
/// // build the response
/// let response = Response::success().msg("Hello, world!").data(data);
///
/// // send the response to the client
/// response.send();
/// ```
///
/// The client will receive the following JSON:
/// ```json
/// {
///    "type": "Success",
///    "msg": "Hello, world!",
///    "data": {
///       "id": 1,  
///       "name": "Shanto Islam"
///    }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    #[serde(rename = "type")]
    response_type: ResponseType,
    msg: Option<String>,
    data: Option<serde_json::Value>,
}

impl Response {
    /// Create a new response    
    pub fn new(response_type: ResponseType) -> Self {
        Self {
            response_type,
            msg: None,
            data: None,
        }
    }

    /// Set the type to [`ResponseType::AlreadyExists`]
    pub fn already_exists() -> Self {
        Self {
            response_type: ResponseType::AlreadyExists,
            msg: None,
            data: None,
        }
    }

    /// Set the type to [`ResponseType::Success`]
    pub fn success() -> Self {
        Self {
            response_type: ResponseType::Success,
            msg: None,
            data: None,
        }
    }

    /// Set the type to [`ResponseType::NotFound`]
    pub fn not_found() -> Self {
        Self {
            response_type: ResponseType::NotFound,
            msg: None,
            data: None,
        }
    }

    /// Set the type to [`ResponseType::ServerError`]
    pub fn server_error() -> Self {
        Self {
            response_type: ResponseType::ServerError,
            msg: None,
            data: None,
        }
    }

    /// Set the type to [`ResponseType::NoPermission`]
    pub fn no_permission() -> Self {
        Self {
            response_type: ResponseType::NoPermission,
            msg: None,
            data: None,
        }
    }

    /// Set the type to [`ResponseType::InvalidInput`]
    pub fn invalid_input() -> Self {
        Self {
            response_type: ResponseType::InvalidInput,
            msg: None,
            data: None,
        }
    }

    /// Set the type to [`ResponseType::Unauthorized`]
    pub fn unauthorized() -> Self {
        Self {
            response_type: ResponseType::Unauthorized,
            msg: None,
            data: None,
        }
    }

    /// Set the type to [`ResponseType::IncorrectPassword`]
    pub fn incorrect_password() -> Self {
        Self {
            response_type: ResponseType::IncorrectPassword,
            msg: None,
            data: None,
        }
    }

    /// Set a custom message
    pub fn msg(mut self, message: &str) -> Self {
        self.msg = Some(message.to_string());
        self
    }

    /// Set the data
    ///
    /// The data must be serializable.
    pub fn data<T: serde::Serialize>(mut self, data: T) -> Self {
        self.data = Some(serde_json::to_value(data).unwrap());
        self
    }

    /// Send the response to the client
    ///
    /// This function returns an [`actix_web::HttpResponse`] so you have to annotate the route handler with `-> HttpResponse`.
    ///
    /// ## Example
    /// ```
    /// # use krapi::Response;
    /// # use serde::{Deserialize, Serialize};
    /// # use actix_web::HttpResponse;
    /// # use actix_web::get;    
    /// #
    /// #[derive(Serialize, Deserialize, Debug)]
    /// pub struct User {
    ///   pub id: i32,
    ///   pub name: String,
    /// }
    ///
    /// #[get("/greet")]
    /// pub async fn route() -> HttpResponse {
    ///    let data = User {
    ///       id: 1,
    ///       name: "Shanto Islam".to_string(),
    ///   };
    ///
    ///   Response::success().msg("Hello, world!").data(data).send()
    /// }
    /// ```
    ///
    pub fn send(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}
