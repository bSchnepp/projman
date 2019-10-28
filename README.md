# Project Manager
## ProjMan
An application written in Rust to handle issue management, issue
tracking, and other notes about a particular project.

It's intention is just for me to use Rust some more without injecting it into
anything too serious, while also building something that might be very useful
after some polish, and should be pretty simple to get started using.


## Building
You'll need the 2018 edition of the Rust language.
In addition, you should have sqlite3 installed in a
place where it can be linked to. For example, on
an Ubuntu machine, you'll need the sqlite3-0 package.

You should then be able to run cargo build --release


## Cleaning
The database is stored by default in the file ~/.projman/core.db, wherever
it is already located. You'll need to remove this file if there's a signifigant
change to the project, but in the future I'll add some sort of migration tool.


## How is this used?
In the future, I'm looking into creating a client in GTK or something similar
to query the server with some information. It will use a custom TCP-based protocol,
and will probably be text-based.

In mind is something like
"REGISTER ISSUE_CLOSE (PROJECT "Feral", ID 83) WITH MESSAGE ("Resolved use-after-free bug in commit 123456");"

