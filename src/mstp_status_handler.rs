use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use std::process::Command;

pub fn status() -> HttpResponse {
    let execute = execute_scan_command();
    match execute {
        Ok(v) => {
            let data = format!("{:?}", v);
            HttpResponse::build(StatusCode::OK)
                .content_type("application/json")
                .body(data)
        }
        Err(_) => {
            HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BACNet {
    device: String,
    mac: String,
    snet: String,
    saddr: String,
    apdu: String,
}

fn execute_scan_command() -> Result<Vec<BACNet>, &'static str> {
    let output = Command::new("./bacwiip")
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
