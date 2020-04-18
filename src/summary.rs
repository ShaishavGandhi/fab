use crate::diffs::{get_authored_diffs, get_needs_review_diffs, render_diffs};
use crate::preferences::Preferences;
use crate::structs::FabConfig;
use crate::tasks::{get_tasks, render_tasks, Priority};
use clap::ArgMatches;
use console::style;
use failure::Error;
use futures::future::join3;

pub fn process_summary(
    _matches: &ArgMatches,
    config: &FabConfig,
    preferences: &Preferences,
) -> Result<(), Error> {
    let priorities: Vec<i32> = preferences
        .summary_task_priority
        .iter()
        .map(|priority| Priority::get_value_for_name(&priority).unwrap())
        .collect();

    let status = "open";

    let result = tokio::runtime::Runtime::new()?.block_on(join3(
        get_needs_review_diffs(config),
        get_authored_diffs(config),
        get_tasks(
            preferences.default_limit.to_string().as_str(),
            &priorities,
            &preferences.default_sort,
            &status,
            config,
        ),
    ));

    println!(
        "{}",
        style("Diffs that need your review").bold().underlined()
    );
    println!();

    render_diffs(config, &result.0?);
    println!();

    println!("{}", style("Your open diffs").bold().underlined());
    println!();

    render_diffs(config, &result.1?);
    println!();

    println!(
        "{}",
        style("Tasks that need your attention").bold().underlined()
    );
    println!();
    render_tasks(&result.2?, config);
    Ok(())
}
