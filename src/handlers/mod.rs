// handlers realted to login
pub mod login_handlers;

// handlers related to logout
pub mod logout_handlers;

// handlers related to registration -> comapny, undergraduate
pub mod registration_handlers;

// handlers related to verification -> otp, email verification
pub mod verification_handlers;

// handlers related to forgot password -> email verification and reset password
pub mod forgot_password_handlers;

// handlers related to test
pub mod test_handlers;

// handlers related to profile
pub mod profile_handlers;

// handlers related to posts -> create, update, delete, get
pub mod post_handlers;

//handlers related to blogs -> create, update, delete, get
pub mod blog_handlers;

// handlers related to projects -> create, update, delete, get
pub mod project_handlers;

// handlers related to clubs -> create, update, delete, get
pub mod club_handlers;

// handlers related to chat -> websocket connections
pub mod chat;
