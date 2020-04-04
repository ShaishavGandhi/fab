use clap::ArgMatches;
use crate::structs::FabConfig;
use crate::diffs::{get_needs_review_diffs, render_diffs, get_authored_diffs};
use crate::tasks::{get_tasks, Priority, render_tasks};
use console::style;
use crate::preferences::Preferences;

pub fn process_summary(_matches: &ArgMatches, config: &FabConfig, preferences: &Preferences) {
    let needs_review_diffs = get_needs_review_diffs(config).unwrap();
    let authored_diffs = get_authored_diffs(config).unwrap();

    let priorities: Vec<i32> = preferences.summary_task_priority.iter()
        .map(|priority| Priority::get_value_for_name(&priority).unwrap())
        .collect();

    let tasks = get_tasks(preferences.default_limit.to_string().as_str(),
        &priorities, config).expect("Couldn't fetch tasks");

    println!("{}", style("Diffs that need your review").bold().underlined());
    println!();

    render_diffs(config, &needs_review_diffs);
    println!();

    println!("{}", style("Your open diffs").bold().underlined());
    println!();

    render_diffs(config, &authored_diffs);
    println!();

    println!("{}", style("Tasks that need your attention").bold().underlined());
    println!();
    render_tasks(&tasks, config);
}
