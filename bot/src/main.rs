mod config;

use config::Config;
//use config::ProgramConfiguration;

fn main() {

    //default values for config
    let mut conf = Config {
        host: Some("sandbox-invest-public-api.tinkoff.ru".to_string()),
        port: Some(443),
        token: Some("".to_string()),
        path_token_log: Some("".to_string()),
        path_file_log: Some("/var/log/".to_string()),
        rotation: Some(2)
    };

    println!("{:?}", conf.host);
}
