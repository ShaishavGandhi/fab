use crate::diffs::{get_authored_diffs, get_needs_review_diffs, render_diffs};
use crate::preferences::Preferences;
use crate::structs::FabConfig;
use crate::tasks::{get_tasks, render_tasks, Priority};
use clap::ArgMatches;
use console::style;
use failure::Error;

pub fn process_summary(
    _matches: &ArgMatches,
    config: &FabConfig,
    preferences: &Preferences,
) -> Result<(), Error> {
    let needs_review_diffs = get_needs_review_diffs(config)?;
    let authored_diffs = get_authored_diffs(config)?;

    let priorities: Vec<i32> = preferences
        .summary_task_priority
        .iter()
        .map(|priority| Priority::get_value_for_name(&priority).unwrap())
        .collect();

    let tasks = get_tasks(
        preferences.default_limit.to_string().as_str(),
        &priorities,
        config,
    )?;

    println!(
        "{}",
        style("Diffs that need your review").bold().underlined()
    );
    println!();

    render_diffs(config, &needs_review_diffs);
    println!();

    println!("{}", style("Your open diffs").bold().underlined());
    println!();

    render_diffs(config, &authored_diffs);
    println!();

    println!(
        "{}",
        style("Tasks that need your attention").bold().underlined()
    );
    println!();
    render_tasks(&tasks, config);
    Ok(())
}
