use crate::structs::{Revision, RevisionResponse, FabConfig};
use clap::ArgMatches;

const DIFFERENTIAL_SEARCH_URL: &str = "api/differential.revision.search";

pub fn process_diff_command(_matches: &ArgMatches, config: &FabConfig) {
    let json_body = json!({
            "queryKey": "authored",
            "api.token": config.api_token,
        });


    let url = format!("{}{}", &config.hosted_instance, DIFFERENTIAL_SEARCH_URL.to_string());
    let response = reqwest::blocking::Client::new()
        .post(&url)
        .form(&json_body)
        .send()
        .unwrap();

    let revision: Vec<Revision> = (response.json::<RevisionResponse>()).unwrap().result.data;

    let revisions: Vec<&Revision> = revision.iter().filter(|rev| !rev.fields.status.closed).collect();

    for revision in revisions {
        println!("{} {} {}", revision.status(), revision.fields.title, revision.url(config))
    }
}
