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
Get started by checking on your differentials
```
fab diffs
```
When you execute this the first time, Fab will ask you for two main things. 
1. The URL for where your Phabricator instance is hosted.
2. An API token it can work with. 

Once you have that, you're good to go!

## Usage

You can check on your authored diffs.
```
fab diffs
```
You can also check on diffs that need your review
```
fab diffs --needs-review
```

You can check high priority tasks that are assigned to you
```
fab tasks --priority high 
```
You can also give multiple values for the priority
```
fab tasks --priority high normal
```
If the results become too overwhelming, you can limit them
```
fab tasks --priority high normal --limit 10
```

You can get a snapshot summary of what requires your attention
```
fab summary
```
This will output the diffs that require your attention, your authored diffs and tasks you should be working on. 

By default, the `summary` command will display high priority tasks. You can configure that and much more by doing
```
fab configure
```
which will take you to an interactive shell where you can select which priority tasks show up for your commands. 

You can also reset to default preferences by doing
```
fab configure --reset
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
