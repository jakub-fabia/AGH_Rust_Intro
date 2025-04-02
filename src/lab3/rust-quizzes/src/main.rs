mod models;
mod services;    
 
fn main() {
    models::user::participant::create_participant();
    services::auth::login();
}