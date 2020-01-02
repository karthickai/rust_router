use crate::models::MSTP;
use actix_identity::Identity;
use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::HttpResponse;
use std::process::Command;

pub fn get_mstp(id: Identity) -> HttpResponse {
    if let Some(_) = id.identity() {
        let execute = execute_mstp_command();
        match execute {
            Ok(v) => HttpResponse::build(StatusCode::OK)
                .content_type("application/json")
                .body(v[0].to_string()),
            Err(_) => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
        }
    } else {
        HttpResponse::Unauthorized().json("Unauthorized")
    }
}

fn execute_mstp_command() -> Result<Vec<String>, &'static str> {
    let output = Command::new("/usr/bin/python3")
        .arg("/home/root/bin/utils.py")
        .arg("mstp")
        .output()
        .expect("Unable to exectue the utils.py command");

    println!("Output status {}", output.status);
    if output.status.success() {
        let output_string = String::from_utf8_lossy(&output.stdout);
        let output_lines: Vec<_> = output_string.trim().lines().collect();
        let mut mstp_data: Vec<String> = Vec::new();
        for line in output_lines {
            mstp_data.push(line.to_string());
        }
        Ok(mstp_data)
    } else {
        Err("Error in Execution")
    }
}

pub fn post_mstp(mstp_data: web::Json<MSTP>, id: Identity) -> HttpResponse {
    if let Some(_) = id.identity() {
        let execute = execute_post_mstp_command(mstp_data.into_inner());
        match execute {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
        }
    } else {
        HttpResponse::Unauthorized().json("Unauthorized")
    }
}

fn execute_post_mstp_command(mstp_data: MSTP) -> Result<Vec<String>, &'static str> {
    let mstp = serde_json::to_string(&mstp_data).expect("Unable to convert json string");
    let output = Command::new("/usr/bin/python3")
        .arg("/home/root/bin/utils.py")
        .arg("mstp")
        .arg("--data")
        .arg(mstp)
        .output()
        .expect("Unable to exectue the utils.py command");
    if output.status.success() {
        let output_string = String::from_utf8_lossy(&output.stdout);
        let output_lines: Vec<_> = output_string.trim().lines().collect();
        let mut mstp_data: Vec<String> = Vec::new();
        for line in output_lines {
            mstp_data.push(line.to_string());
        }
        Ok(mstp_data)
    } else {
        Err("Error in Execution")
    }
}
