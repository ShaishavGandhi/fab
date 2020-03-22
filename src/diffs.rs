use crate::structs::{Revision, RevisionResponse, FabConfig};
use clap::ArgMatches;

const DIFFERENTIAL_SEARCH_URL: &str = "api/differential.revision.search";

pub fn process_diff_command(_matches: &ArgMatches, config: &FabConfig) {
    if _matches.is_present("needs-review") {
        process_diffs_needs_review(config);
        return;
    }

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

    render_diffs(config, &revisions)
}

fn process_diffs_needs_review(config: &FabConfig) {
    let json_body = json!({
            "api.token": config.api_token,
            "constraints[reviewerPHIDs][0]": config.phid
        });

    let url = format!("{}{}", config.hosted_instance, DIFFERENTIAL_SEARCH_URL);

    let response = reqwest::blocking::Client::new()
        .post(&url)
        .form(&json_body)
        .send()
        .expect("Failed to fetch response for needs-review diffs")
        .json::<RevisionResponse>()
        .expect("Failed to deserialize diffs");

    let revisions = response.result.data.iter().filter(|rev| !rev.fields.status.closed).collect();
    render_diffs(config, &revisions)

}

fn render_diffs(config: &FabConfig, revisions: &Vec<&Revision>) {
    for revision in revisions {
        println!("{} {} {}", revision.status(), revision.fields.title, revision.url(config))
    }
}
