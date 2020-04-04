use crate::auth;
use crate::structs::{FabConfig, Revision, RevisionData};
use crate::NO_BORDER_PRESET;
use clap::ArgMatches;
use comfy_table::{Attribute, Cell, CellAlignment, ContentArrangement, Table};

const DIFFERENTIAL_SEARCH_URL: &str = "api/differential.revision.search";

pub fn process_diff_command(_matches: &ArgMatches, config: &FabConfig) {
    if _matches.is_present("needs-review") {
        process_diffs_needs_review(config);
        return;
    }

    let result = get_authored_diffs(config).expect("Couldn't fetch diffs");

    render_diffs(config, &result);
}

/// Get diffs that are authored by the user.
pub fn get_authored_diffs(config: &FabConfig) -> Result<Vec<Revision>, String> {
    let json_body = json!({
        "queryKey": "authored",
        "api.token": config.api_token,
    });

    let url = format!(
        "{}{}",
        &config.hosted_instance,
        DIFFERENTIAL_SEARCH_URL.to_string()
    );

    let result = auth::send::<RevisionData>(
        config,
        reqwest::blocking::Client::new().post(&url).form(&json_body),
    );

    match result {
        Ok(result) => {
            let revisions = result
                .data
                .into_iter()
                .filter(|rev| !rev.fields.status.closed)
                .collect();
            Result::Ok(revisions)
        }
        Err(_err) => Result::Err(String::from("Couldn't fetch authored diffs")),
    }
}

/// Get the diffs that needs review from the user.
pub fn get_needs_review_diffs(config: &FabConfig) -> Result<Vec<Revision>, String> {
    let json_body = json!({
        "api.token": config.api_token,
        "constraints[reviewerPHIDs][0]": config.phid
    });

    let url = format!("{}{}", config.hosted_instance, DIFFERENTIAL_SEARCH_URL);

    let result = auth::send::<RevisionData>(
        config,
        reqwest::blocking::Client::new().post(&url).form(&json_body),
    );

    match result {
        Ok(response) => {
            let revisions = response
                .data
                .into_iter()
                .filter(|rev| !rev.fields.status.closed)
                .collect();

            Result::Ok(revisions)
        }
        Err(_err) => Result::Err(String::from("Failed to fetch needs-review diffs")),
    }
}

pub fn render_diffs(config: &FabConfig, revisions: &[Revision]) {
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
            Cell::new(&revision.url(config)).add_attribute(Attribute::Bold),
        ]);
    }
    println!("{}", table);
}

fn process_diffs_needs_review(config: &FabConfig) {
    let revisions =
        get_needs_review_diffs(config).expect("Failed to fetch response for needs-review diffs");

    // let revisions = response.data.iter().filter(|rev| !rev.fields.status.closed).collect();
    render_diffs(config, &revisions)
}
