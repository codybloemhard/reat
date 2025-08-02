# reat

Tool for ergonomic use of extended attributes of filesystem objects.

Reat stands for Rust Extended ATtribute.

- ergonomic cli
  - short hands
  - just words, no flags
  - multiple orders of arguments
- coloured output
- list attributes
- get attribute
- set attribute
- remove attribute
- add item to list attribute
- cut item from list attribute
- rename attributes
- replace item with another in list attribute 
- copy attributes from source file to destination file
- contains strings in attribute: or, and/all, not
- dump attribute data
- restore attribute data from dump
- clear attributes
- tags: slight special treatment
- read paths via stdin: chain reat with itself and others

todo:

- rank
- index
- workflows
- sort
- help

cli interface argument orders:

- reat get att file
- reat get att file file file
- reat get att att att - file file file
- reat file file file get att att att
- reat set att val file
- reat set att val file file file
- reat set att att att val - file file file
- reat file file file set att att att val

might implement:

- reat set m att val att val att val - file file file
- reat m att val att val att val set file file file

