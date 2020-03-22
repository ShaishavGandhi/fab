use crate::structs::{Revision, RevisionResponse, FabConfig};
use clap::ArgMatches;
use comfy_table::{Table, Cell, ContentArrangement, TableComponent, Attribute, CellAlignment};
use comfy_table::presets::UTF8_FULL;

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
    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_style(TableComponent::BottomBorder, ' ')
        .set_style(TableComponent::TopBorder, ' ')
        .set_style(TableComponent::LeftBorder, ' ')
        .set_style(TableComponent::RightBorder, ' ')
        .set_style(TableComponent::HorizontalLines, ' ')
        .set_style(TableComponent::VerticalLines, ' ')
        .set_style(TableComponent::BottomBorderIntersections, ' ')
        .set_style(TableComponent::LeftBorderIntersections, ' ')
        .set_style(TableComponent::RightBorderIntersections, ' ')
        .set_style(TableComponent::TopBorderIntersections, ' ')
        .set_style(TableComponent::MiddleIntersections, ' ')
        .set_style(TableComponent::RightHeaderIntersection, ' ')
        .set_style(TableComponent::LeftHeaderIntersection, ' ')
        .set_style(TableComponent::TopLeftCorner, ' ')
        .set_style(TableComponent::TopRightCorner, ' ')
        .set_style(TableComponent::BottomLeftCorner, ' ')
        .set_style(TableComponent::BottomRightCorner, ' ');

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
