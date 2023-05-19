mod greet;
mod user;

pub use user::create_user::route as create_user_route;
pub use user::get_users::route as get_users_route;
pub use user::match_user::route as match_user_route;
pub use user::update_password::route as update_password_route;
pub use user::update_user::route as update_user_route;

pub use greet::route as greet_route;
