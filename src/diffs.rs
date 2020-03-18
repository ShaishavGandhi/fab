use crate::structs::{Revision, RevisionResponse};
use clap::ArgMatches;

const DIFFERENTIAL_SEARCH_URL: &str = "https://code.uberinternal.com/api/differential.revision.search";

pub fn process_diff_command(_matches: &ArgMatches) {
    let json_body = json!({
            "queryKey": "authored",
            "api.token": "cli-lsmimleim4ohaovkxj23xcnrgoc6",
        });


    let response = reqwest::blocking::Client::new()
        .post(DIFFERENTIAL_SEARCH_URL)
        .form(&json_body)
        .send()
        .unwrap();

    let revision: Vec<Revision> = (response.json::<RevisionResponse>()).unwrap().result.data;

    let revisions: Vec<&Revision> = revision.iter().filter(|rev| !rev.fields.status.closed).collect();

    for revision in revisions {
        println!("{} {} {}", revision.status(), revision.fields.title, revision.url())
    }
}
