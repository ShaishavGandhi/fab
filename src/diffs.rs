use crate::structs::{FabConfig, Revision, RevisionData};
use crate::NO_BORDER_PRESET;
use crate::{auth, users};
use clap::ArgMatches;
use comfy_table::{Attribute, Cell, CellAlignment, ContentArrangement, Table};
use failure::Error;
use serde_json::{Map, Value};
use tokio::runtime::Runtime;

const DIFFERENTIAL_SEARCH_URL: &str = "api/differential.revision.search";

/// Get diffs that are authored by the user.
pub async fn get_authored_diffs(config: &FabConfig) -> Result<Vec<Revision>, Error> {
    let json_body = json!({
        "queryKey": "authored",
        "api.token": config.api_token,
    });

    let url = format!(
        "{}{}",
        &config.hosted_instance,
        DIFFERENTIAL_SEARCH_URL.to_string()
    );

    let result =
        auth::send::<RevisionData>(config, reqwest::Client::new().post(&url).form(&json_body))
            .await?
            .data
            .into_iter()
            .filter(|rev| !rev.fields.status.closed)
            .collect();

    Ok(result)
}

/// Get diffs authored by given author
pub async fn get_diffs(config: &FabConfig, author: &Option<&str>) -> Result<Vec<Revision>, Error> {
    if author.is_none() {
        return Err(failure::err_msg("No author specified"));
    }

    let author = author.unwrap();

    let user = users::get_user(&author, config).await?;

    let url = format!(
        "{}{}",
        &config.hosted_instance,
        DIFFERENTIAL_SEARCH_URL.to_string()
    );

    let mut map = Map::new();
    map.insert(
        "api.token".to_string(),
        Value::from(config.api_token.clone()),
    );
    map.insert(
        "constraints[authorPHIDs][0]".to_string(),
        Value::from(user.phid.clone()),
    );

    let json_body = Value::Object(map);

    let result =
        auth::send::<RevisionData>(config, reqwest::Client::new().post(&url).form(&json_body))
            .await?
            .data
            .into_iter()
            .filter(|rev| !rev.fields.status.closed)
            .collect();

    Ok(result)
}

/// Get the diffs that needs review from the user.
pub async fn get_needs_review_diffs(config: &FabConfig) -> Result<Vec<Revision>, Error> {
    let json_body = json!({
        "api.token": config.api_token,
        "constraints[reviewerPHIDs][0]": config.phid
    });

    let url = format!("{}{}", config.hosted_instance, DIFFERENTIAL_SEARCH_URL);

    let result =
        auth::send::<RevisionData>(config, reqwest::Client::new().post(&url).form(&json_body))
            .await?
            .data
            .into_iter()
            .filter(|rev| !rev.fields.status.closed)
            .collect();

    Ok(result)
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

pub fn process_diff_command(_matches: &ArgMatches, config: &FabConfig) -> Result<(), Error> {
    if _matches.is_present("needs-review") {
        process_diffs_needs_review(config)?;
        return Ok(());
    }
    if _matches.is_present("author") {
        process_authored_diffs(config, _matches.value_of("author"))?;
        return Ok(());
    }

    let result = Runtime::new()?.block_on(get_authored_diffs(config))?;

    render_diffs(config, &result);
    Ok(())
}

fn process_diffs_needs_review(config: &FabConfig) -> Result<(), Error> {
    let revisions = Runtime::new()?.block_on(get_needs_review_diffs(config))?;

    render_diffs(config, &revisions);
    Ok(())
}

fn process_authored_diffs(config: &FabConfig, author: Option<&str>) -> Result<(), Error> {
    let revisions = Runtime::new()?.block_on(get_diffs(config, &author))?;

    render_diffs(config, &revisions);
    Ok(())
}
