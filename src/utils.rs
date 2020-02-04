use crate::errors::ServiceError;
use easy_password::bcrypt::{hash_password, verify_password};
use std::process::Command;

lazy_static::lazy_static! {
pub  static ref SECRET_KEY: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
}

// WARNING THIS IS ONLY FOR DEMO PLEASE DO MORE RESEARCH FOR PRODUCTION USE
pub fn hash(password: &str) -> Result<String, ServiceError> {
    hash_password(password, SECRET_KEY.as_bytes(), 12).map_err(|err| {
        dbg!(err);
        ServiceError::InternalServerError
    })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    verify_password(password, hash, SECRET_KEY.as_bytes()).map_err(|err| {
        dbg!(err);
        ServiceError::Unauthorized
    })
}

pub fn execute_router_service_stop() {
    Command::new("systemctl")
        .arg("stop")
        .arg("router.service")
        .output()
        .expect("Unable to stop the router.service command");
}

pub fn execute_router_service_start() {
    Command::new("systemctl")
        .arg("start")
        .arg("router.service")
        .output()
        .expect("Unable to start the router.service command");
}
