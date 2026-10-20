# idea

The idea is to to have a simple ticket tool for CLI and with a webserver aswell.

Focus in keeping it simple, no org i have worked in has successfully used epics and initiatives in a way that lets them justify why both exist.

So here is the main things.

## Vocab

### Ticket repository
A folder with tickets and initiatives.


## Initiative
An initiative is a strategic business goals or high level planning unit, a dev or org defines this is something we wish to achive then create tasks from it.
I know jira lets you have multiple epics inside a initiative, and defines them as clear diffrent things; but i have never liked them and never been in an org that used them correctly so i have personally given up.

## Ticket
A ticket is something that can be worked on, or have time logged on it.


# Getting started
To get stated you install the tool and then create a `ticket repository` using the `init` command.
like this.
```bash
tk init
```

This will create a `.ticket` folder in the current folder, inside that folder there will be 2 sub-folders `tickets`, `initaitves` and a config file, `.config`; and later ther is going to be a `.cache` file aswell.

## Ticket types
* Task
* Bug
