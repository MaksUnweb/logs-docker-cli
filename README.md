## Introduction
Hello everybody! If you needed often check logs from Docker container, then this repo for you!
## Base functional:
1) Check logs in real time ✅
2) Check logs over definite time period: 
- 15 minutes ✅
- 30 ✅
- 1 hour ✅
- 2 hours ✅


## Usage: 
For using you need install rust and cargo compilation! At the same time, it is not lower than the version 1.95! 

For  compile (after cloning my  repo) you need use command `cargo build --release` in the root directory project. After compile you will have a `target/release/logs-docker-cli`

**WARNING! This program using UNIX time! for stable work recommended to use only on UNIX/Linux systems!**

## Keys used to operate the program:

1) `--help` - base documentation
2) `-c` - id your Docker container for connecting
3) `-m` - mode of operation (log output), there are the following modes in total: 
- `all` - output all logs in the real time 
- `last15-min` - logs for the last 15 minutes
- `last30-min` - logs for the last 30 minutes
- `last1-hour` - logs for the last 1 hour
- `last2-hours` - logs for the last 2 hours



