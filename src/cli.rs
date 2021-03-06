use crate::preferences::Preferences;
use clap::{App, Arg};

pub const VERSION: &str = "0.4.2";

/// Builds the App with commands and defaults.
pub fn build_cli(preferences: &Preferences) -> App {
    let default_task_priority: &Vec<&str> = &preferences
        .default_task_priority
        .iter()
        .map(std::ops::Deref::deref)
        .collect();

    let default_limit = preferences.default_limit_str.as_str();
    let default_sort = preferences.default_sort.as_ref();

    App::new("Fab")
        .author("Shaishav <shaishavgandhi05@gmail.com>")
        .version(VERSION)
        .subcommand(
            App::new("diffs")
                .version(VERSION)
                .author("Shaishav <shaishavgandhi05@gmail.com>")
                .about("Commands related to your differential revisions")
                .arg(
                    Arg::with_name("needs-review")
                        .short('n')
                        .long("needs-review")
                        .help("Show diffs that need your review"),
                )
                .arg(
                    Arg::with_name("author")
                        .short('a')
                        .long("author")
                        .required(false)
                        .takes_value(true)
                        .help(
                            "Find diffs with given author name. This is the Phabricator username",
                        ),
                ),
        )
        .subcommand(
            App::new("tasks")
                .about("Commands related to maniphest tasks")
                .version(VERSION)
                .author("Shaishav <shaishavgandhi05@gmail.com>")
                .arg(
                    Arg::with_name("priority")
                        .short('p')
                        .long("priority")
                        .possible_values(&[
                            "unbreak-now",
                            "needs-triage",
                            "high",
                            "normal",
                            "low",
                            "wishlist",
                        ])
                        .help("Specify the priority of the task")
                        .default_values(default_task_priority)
                        .multiple(true),
                )
                .arg(
                    Arg::with_name("limit")
                        .short('l')
                        .long("limit")
                        .help("limit results by a value")
                        // .default_value(preferences.get_default_limit().clone().as_str())
                        .default_value(&default_limit),
                )
                .arg(
                    Arg::with_name("sort")
                        .short('s')
                        .long("sort")
                        .help("Sort results")
                        .possible_values(&["priority", "updated", "newest", "title"])
                        .default_value(default_sort),
                )
                .arg(
                    Arg::with_name("status")
                        .short('S')
                        .long("status")
                        .help("Filter tasks by status")
                        .possible_values(&["open", "resolved", "wontfix", "invalid", "duplicate"])
                        .default_value("open"),
                ),
        )
        .subcommand(
            App::new("summary")
                .about("Gives a snapshot of what is relevant to you in the moment")
                .version(VERSION)
                .author("Shaishav <shaishavgandhi05@gmail.com>"),
        )
        .subcommand(
            App::new("configure")
                .about("Configure settings")
                .arg(
                    Arg::with_name("reset")
                        .short('r')
                        .long("reset")
                        .help("Reset preferences to their default value"),
                )
                .version(VERSION)
                .author("Shaishav <shaishavgandhi05@gmail.com>"),
        )
        .subcommand(
            App::new("generate-shell-completions")
                .about("Generate the bash completion files for fab")
                .version(VERSION)
                .arg(
                    Arg::with_name("shell")
                        .short('s')
                        .long("shell")
                        .required(true)
                        .help("Pass the shell for which you want completions")
                        .possible_values(&["bash", "zsh", "fish", "elvish", "powershell"]),
                )
                .author("Shaishav <shaishavgandhi05@gmail.com>"),
        )
}
