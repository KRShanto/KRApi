mod user;

pub use user::create_user::route as create_user_route;
pub use user::get_users::route as get_users_route;
pub use user::match_user::route as match_user_route;
pub use user::update_password::route as update_password_route;
