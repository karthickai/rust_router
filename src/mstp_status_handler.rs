use actix_identity::Identity;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use std::process::Command;
use crate::utils;


pub fn deep_scan(id: Identity) -> HttpResponse {
    if let Some(_) = id.identity() {
        utils::execute_router_service_stop();
        let execute = execute_scan_command();
        utils::execute_router_service_start();
        match execute {
            Ok(v) => {
                let serialized_outter = serde_json::to_string(&v).unwrap();
                let execute = execute_db_util_command_deep(serialized_outter);
                match execute {
                    Ok(v_in) => HttpResponse::build(StatusCode::OK)
                        .content_type("application/json")
                        .body(v_in),
                    Err(_) => HttpResponse::InternalServerError()
                        .json("Internal Server Error, Please try later"),
                }
            }
            Err(_) => {
                utils::execute_router_service_start();
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
        }
    } else {
        HttpResponse::Unauthorized().json("Unauthorized")
    }
}

pub fn quick_scan(id: Identity) -> HttpResponse {
    if let Some(_) = id.identity() {
        let execute = execute_db_util_command_quick();
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
struct BACNet {
    device: String,
    mac: String,
    snet: String,
    saddr: String,
    apdu: String,
}

fn execute_scan_command() -> Result<Vec<BACNet>, &'static str> {
    let output = Command::new("/home/root/bin/bacwi")
        .arg("-1")
        .output()
        .expect("Unable to exectue the ./bacwi command");

    println!("Output status {}", output.status);
    if output.status.success() {
        let output_string = String::from_utf8_lossy(&output.stdout);
        let output_lines: Vec<_> = output_string.trim().lines().collect();
        let mut bacnet_data: Vec<BACNet> = Vec::new();
        for line in output_lines {
            let data_vec: Vec<&str> = line.split_whitespace().collect();
            let bacnet = BACNet {
                device: String::from(data_vec[0]),
                mac: String::from(data_vec[1]),
                snet: String::from(data_vec[2]),
                saddr: String::from(data_vec[3]),
                apdu: String::from(data_vec[4]),
            };
            bacnet_data.push(bacnet);
        }
        // println!("The count is {}", bacnet_data.len());
        //println!("{:?}", bacnet_data);
        Ok(bacnet_data)
    } else {
        Err("Error in Execution")
    }
}

fn execute_db_util_command_quick() -> Result<Vec<String>, &'static str> {
    let output = Command::new("/usr/bin/python3")
        .arg("/home/root/bin/dbutils.py")
        .arg("get")
        .output()
        .expect("Unable to exectue the ./bacwi command");

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

fn execute_db_util_command_deep(data: String) -> Result<String, &'static str> {
    let output = Command::new("/usr/bin/python3")
        .arg("/home/root/bin/dbutils.py")
        .arg("update")
        .arg("--data")
        .arg(data)
        .output()
        .expect("Unable to exectue the ./bacwi command");

    if output.status.success() {
        let output_string = String::from_utf8_lossy(&output.stdout);
        let output_lines: Vec<_> = output_string.trim().lines().collect();
        Ok(output_lines[0].to_string())
    } else {
        Err("Error in Execution")
    }
}


