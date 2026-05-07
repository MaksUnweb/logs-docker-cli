use clap::Parser;
use clap::ValueEnum;
mod includes;
use includes::{ 
    connect_to_docker,
    connect_and_get_logs,
    connect_and_get_logs_follow,
    create_log_options,
    get_time_as_secs
};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
   #[arg(short, long)] 
    container_id: String,
    #[arg(value_enum, long, short)]
    mode: Actions 
}

#[derive(ValueEnum, Clone, Debug)]
enum Actions{
    All,
    Last15Min,
    Last30Min,
    Last1Hour,
    Last2Hours
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    //Getting an id:
     let container_id = &args.container_id;

    //Connecting to Docker: 
    //WARNING! IF CONNECTING FAILS, PROGRAM CALLING PANIC!
    let docker = connect_to_docker().await;

    match &args.mode {
        Actions::All => {
             let _ = connect_and_get_logs_follow(&docker, &container_id)
                 .await 
                 .map_err(|e| {
                    eprintln!("Error get logs: {} Please check your CONTAINER ID or Docker!", e);
                    std::process::exit(1);
                 });
        }
        Actions::Last15Min  => {
            let since = get_time_as_secs(15);
            let options = create_log_options(since);
            let _ = connect_and_get_logs(&docker, &container_id, options).await;
        }
        Actions::Last30Min => {
            let since = get_time_as_secs(30);
            let options = create_log_options(since);
            let _ = connect_and_get_logs(&docker, &container_id, options).await;
        }
        Actions::Last1Hour => {
            let since = get_time_as_secs(60);
            let options = create_log_options(since);
            let _ = connect_and_get_logs(&docker, &container_id, options).await;
        }
        Actions::Last2Hours => {
            let since = get_time_as_secs(120);
            let options = create_log_options(since);
            let _ = connect_and_get_logs(&docker, &container_id, options).await;
        }
    }
}
