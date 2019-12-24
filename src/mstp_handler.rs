use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use std::process::Command;

pub fn get_mstp() -> HttpResponse {
    let execute = execute_mstp_command();
    match execute {
        Ok(v) => HttpResponse::build(StatusCode::OK)
            .content_type("application/json")
            .body(v[0].to_string()),
        Err(_) => {
            HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
        }
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
