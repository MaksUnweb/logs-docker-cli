use bollard::{Docker, container::LogOutput, query_parameters::LogsOptions};
use futures::{StreamExt, TryStreamExt};
use std::{time::{SystemTime, UNIX_EPOCH}};
use colored::Colorize;


//Function for connectiong to docker:
pub async fn connect_to_docker() -> Docker {
    let docker = match Docker::connect_with_defaults() {
        Ok(docker) => { docker }
        Err(_) => { 
            eprintln!("Error connecting to Docker!");
            std::process::exit(1);
        }
   };

   docker
}

//Function for output all logs in real time
pub async fn connect_and_get_logs_follow(docker: &Docker, container_id: &String) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let options = LogsOptions {
        follow: true, 
        stdout: true, 
        stderr: true, 
        timestamps: true,
        tail: "all".to_string(),
        since: 0,
        until: 0 
    };
   
   let mut logs = docker.logs(container_id, Some(options));
 while let Some(log_result) = logs.next().await {
     match log_result {
         Ok(LogOutput::StdOut { message }) => {
             println!("\n");
             println!("{:?}", message);
         }
         Ok(LogOutput::StdErr { message }) => {
             let message = String::from_utf8_lossy(&message).to_string();
             println!("\n");
             println!("{}", message.red());
         }
         _ => {}
         }
     };
 
    Ok(())
}

//Function for output with time:
pub async fn connect_and_get_logs(docker: &Docker, container_id: &String, options: LogsOptions) {

    let logs: Vec<LogOutput> = match docker.logs(&container_id, Some(options)) 
        .try_collect()
        .await {
            Ok(logs) => { logs }
            Err(_) => { 
                eprintln!("Error getting logs! Check Docker or container");
                std::process::exit(1);
            }
        };
        
    for log in logs {
        match log {
            LogOutput::StdOut { message } => {
                let message = String::from_utf8_lossy(&message).to_string();
                println!("{}", message);
            }
            LogOutput::StdErr { message } => {
                let message = String::from_utf8_lossy(&message).to_string();
                println!("{}", message.red());
            }
            _ => {}
        }
    }
}

//Function for creating unique options:
pub fn create_log_options(since: i32) -> LogsOptions {
    let options = LogsOptions {
        follow: false, 
        stdout: true, 
        stderr: true, 
        timestamps: true,
        tail: "all".to_string(),
        since,
        until: 0 
    };
    options
}

//Function for calculating time:
pub fn get_time_as_secs(need_time: i32) -> i32 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Error getting current time!")
        .as_secs() as i32;

    //Calculating which timestamp last: 
    let time_ago = now - need_time * 60;
    time_ago
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use bollard::{query_parameters::LogsOptions};

    use crate::includes::{create_log_options, get_time_as_secs};


    #[test]
    fn test_create_log_options(){
       let test_since: i32 = 15;
       let returned_options = create_log_options(test_since);
       let true_options = LogsOptions {
           follow: false, 
           stdout: true, 
           stderr: true, 
           timestamps: true,
           tail: "all".to_string(),
           since: 15,
           until: 0 
       };
       assert_eq!(true_options, returned_options);
    }
    

    #[test]
    fn test_time_as_secs(){
        let need_time = 5;
        let result = get_time_as_secs(need_time);

        let now = SystemTime::now() 
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i32;
        let expected = now - (5 * 60);
        let difference = (result - expected).abs();
        assert!(difference <= 1,
            "Expected {} but got {}, difference: {}",
            expected,
            result,
            difference
        );
    }
}
