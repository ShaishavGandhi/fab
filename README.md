# fab

Opening up a browser to do a spot check on your differentials and tasks is time consuming. Since you're on the command-line anyway, Fab makes it much faster to check on your tasks, differentials and everything else by cutting down all the fat and showing you only the information you care about. 

## Getting Started
### Installation
Using `brew`
```
brew tap ShaishavGandhi/fab
brew install fab
```

### Initialization
Get started by getting a summary of what you should be focusing on.
```
fab summary
```
When you execute this the first time, Fab will ask you for two main things. 
1. The URL for where your Phabricator instance is hosted.
2. An API token it can work with. 

Once you have that, you're good to go!

## Usage

Fab aims to help you focus on the things that require your attention. You can get a quick overview of things that need your attention by doing:
```
fab summary
```
This will provide a list of diffs that require your review, your authored diffs and tasks that you should be working on.

### Differentials

You can check on your authored diffs:
```
fab diffs
```

You can also check on diffs that need your review:
```
fab diffs --needs-review
```
### Tasks

You can check on high priority tasks that are assigned to you:
```
fab tasks --priority high 
```

You can also give multiple values for the priority:
```
fab tasks --priority high normal
```

If the results become too overwhelming, you can limit them:
```
fab tasks --priority high normal --limit 10
```

Fab will show open tasks by default but you can toggle that behavior:
```
fab tasks --status=resolved/wontfix/invalid/duplicate
```

You can also specify a sorting order:
```
fab tasks --sort=priority/updated/newest/title
```

### Configuration

Everyone has different workflows. Fab aims to make most functionality configurable. Just type:
```
fab configure
```
which will take you to an interactive shell where you can configure:
* Priority of tasks that show up in `fab summary`
* Default limits for results
* Default sort order

You can also reset to default preferences by doing
```
fab configure --reset
```

### Shell Completion

Fab will output shell completions scripts for your favorite shell that you can add to your rc files. 
```
fab generate-shell-completions --shell=zsh/bash/fish/elvish/powershell
```

## Contributing

Contributions are highly welcome. This is a project still in early phases so feature requests + bug reports are greatly appreciated! 

## License

```
Copyright 2020 Shaishav Gandhi.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
