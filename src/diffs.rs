use crate::structs::{Revision, FabConfig, RevisionData};
use clap::ArgMatches;
use comfy_table::{Table, Cell, ContentArrangement, Attribute, CellAlignment};
use crate::NO_BORDER_PRESET;
use crate::auth;

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

    let result = auth::send::<RevisionData>(config, reqwest::blocking::Client::new()
        .post(&url)
        .form(&json_body))
        .expect("Couldn't fetch diffs");

    let revisions: Vec<&Revision> = result.data.iter().filter(|rev| !rev.fields.status.closed).collect();

    render_diffs(config, &revisions);
}

fn process_diffs_needs_review(config: &FabConfig) {
    let json_body = json!({
            "api.token": config.api_token,
            "constraints[reviewerPHIDs][0]": config.phid
        });

    let url = format!("{}{}", config.hosted_instance, DIFFERENTIAL_SEARCH_URL);

    let response = auth::send::<RevisionData>(config, reqwest::blocking::Client::new()
        .post(&url)
        .form(&json_body))
        .expect("Failed to fetch response for needs-review diffs");

    let revisions = response.data.iter().filter(|rev| !rev.fields.status.closed).collect();
    render_diffs(config, &revisions)

}

fn render_diffs(config: &FabConfig, revisions: &Vec<&Revision>) {
    let mut table = Table::new();

    table
        .load_preset(NO_BORDER_PRESET)
        .set_content_arrangement(ContentArrangement::Dynamic);

    for revision in revisions {
        table.add_row(vec![
            Cell::new(&revision.fields.status.name)
                .bg(revision.get_background())
                .fg(revision.get_foreground())
                .set_alignment(CellAlignment::Center)
                .add_attribute(Attribute::Bold),
            Cell::new(&revision.fields.title),
            Cell::new(&revision.url(config))
                .add_attribute(Attribute::Bold)
        ]);
    }
    println!("{}", table);
}
