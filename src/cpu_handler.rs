use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use std::process::Command;

pub fn cpu() -> HttpResponse {
    let execute = execute_scan_command();
    match execute {
        Ok(v) => HttpResponse::build(StatusCode::OK)
            .content_type("application/json")
            .body(v[0].to_string()),
        Err(_) => {
            HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
        }
    }
}

fn execute_scan_command() -> Result<Vec<String>, &'static str> {
    let output = Command::new("./cpu.sh")
        .output()
        .expect("Unable to exectue the ./bacwi command");

    println!("Output status {}", output.status);
    if output.status.success() {
        let output_string = String::from_utf8_lossy(&output.stdout);
        let output_lines: Vec<_> = output_string.trim().lines().collect();
        let mut bacnet_data: Vec<String> = Vec::new();
        for line in output_lines {
            bacnet_data.push(line.to_string());
        }
        // println!("The count is {}", bacnet_data.len());
        // println!("{:?}", bacnet_data);
        Ok(bacnet_data)
    } else {
        Err("Error in Execution")
    }
}
