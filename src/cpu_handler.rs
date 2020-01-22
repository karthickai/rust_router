use actix_identity::Identity;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use std::process::Command;

pub fn cpu(id: Identity) -> HttpResponse {
    if let Some(_) = id.identity() {
        let execute = execute_cpu_command();
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

fn execute_cpu_command() -> Result<Vec<String>, &'static str> {
    let output = Command::new("sh")
        .arg("/home/root/bin/cpu.sh")
        .output()
        .expect("Unable to exectue the ./bacwi command");

    println!("Output status {}", output.status);
    if output.status.success() {
        let output_string = String::from_utf8_lossy(&output.stdout);
        let output_lines: Vec<_> = output_string.trim().lines().collect();
        let mut cpu_data: Vec<String> = Vec::new();
        for line in output_lines {
            cpu_data.push(line.to_string());
        }
        // println!("The count is {}", bacnet_data.len());
        // println!("{:?}", bacnet_data);
        Ok(cpu_data)
    } else {
        Err("Error in Execution")
    }
}
