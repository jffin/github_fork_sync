extern crate clap;
extern crate tokio;

mod http;

use clap::{crate_version, App};
use http::make_request;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let yaml = clap::load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();

    if let Some(target_repo) = matches.value_of("TARGET_REPOSITORY") {
        let github_token: &str = matches.value_of("GITHUB_TOKEN")
            .expect("You didn't provide github token");

        make_request(target_repo, github_token).await
            .expect("Something Went Wrong");
    }

    Ok(())
}
