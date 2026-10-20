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
A ticket uses the numbers 0123456789ABCDEF as valid numbers; yes these are numbers in the hexadecimal system https://en.wikipedia.org/wiki/Hexadecimal

# Getting started
To get stated you install the tool and then create a `ticket repository` using the `init` command.
like this.
```bash
tk init
```

then to create tickets you use one the following commands.
```bash
tk create ticket "this is the ticket title"
tk c t "this is the ticket title"
tk c t "this is a bug, with a start and end date" --start 2010-10-10 --end 2026-10-10 --type bug
tk c t "this is a bug, with a start and end date" -s 2010-10-10 -e 2026-10-10 -t bug
tk c t "Creates the ticket and opens it in your editor" -o
tk c t "Creates the ticket and opens it in your editor" --open
```
dates are always in the format `[year]-[month]-[day]`

then to create initiative you use one the following commands.
```bash
tk create initiative "this is the initiative title"
tk c i "this is the initiative title"
tk c i "this is the initiative title, with a start and end date" --start 2010-12-30 --end 2026-12-30
tk c i "this is the initiative title, with a start and end date" -s 2010-12-30 -e 2026-10-30
tk c i "Creates the initiative and opens it in your editor" -o
tk c i "Creates the initiative and opens it in your editor" --open
```

This will create a `.ticket` folder in the current folder, inside that folder there will be 2 sub-folders `tickets`, `initaitves` and a config file, `conf.yaml`; and later ther is going to be a `.cache` file aswell.

## Ticket types
* Task
* Bug
