use crate::models::IP;
use actix_identity::Identity;
use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::HttpResponse;
use std::process::Command;

pub fn get_ip(id: Identity) -> HttpResponse {
    if let Some(_) = id.identity() {
        let execute = execute_ip_command();
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

fn execute_ip_command() -> Result<Vec<String>, &'static str> {
    let output = Command::new("/usr/bin/python3")
        .arg("/home/root/bin/utils.py")
        .arg("ip")
        .output()
        .expect("Unable to exectue the utils.py command");

    println!("Output status {}", output.status);
    if output.status.success() {
        let output_string = String::from_utf8_lossy(&output.stdout);
        let output_lines: Vec<_> = output_string.trim().lines().collect();
        let mut ip_data: Vec<String> = Vec::new();
        for line in output_lines {
            ip_data.push(line.to_string());
        }
        Ok(ip_data)
    } else {
        Err("Error in Execution")
    }
}

pub fn post_ip(ip_data: web::Json<IP>, id: Identity) -> HttpResponse {
    if let Some(_) = id.identity() {
        let execute = execute_post_ip_command(ip_data.into_inner());
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

fn execute_post_ip_command(ip_data: IP) -> Result<&'static str, &'static str> {
    let ip = serde_json::to_string(&ip_data).expect("Unable to convert json string");
    println!("{:?}", ip);
    let output = Command::new("/usr/bin/python3")
        .arg("/home/root/bin/utils.py")
        .arg("ip")
        .arg("--data")
        .arg(ip)
        .output()
        .expect("Unable to exectue the utils.py command");

    println!("Output status {}", output.status);
    if output.status.success() {
        Ok("done")
    } else {
        Err("Error in Execution")
    }
}
