# What is xtagger?

xtagger is a console application that lets you manage and find tags on your files. While similar
applications like [TMSU](https://tmsu.org/) and [Tagsistant](https://www.tagsistant.net/) use
databases to manage the file/tag relationships, others like [TagSpaces](https://www.tagspaces.org/)
use sidecar files or encode tags in the filename. xtagger is different in that it uses the extended
attributes of supporting file systems on Linux and macOS (Windows alternate data streams are not
supported yet).

This fixes one problem common with most other solutions: You can move or rename your files with your
favourite tools without loosing the file/tag relationship managed elsewhere.

# General facts

xtagger has subcommands for different purposes. To get general help you can issue

	$ xtagger help

or

	$ xtagger help SUBCOMMAND

for help on a specific subcommand.

For handling multiple files in a single call, xtagger not only supports multiple files using normal
shell globs. For bigger file amounts it supports its own glob mechanism. Just put the glob in
quotation marks.

A tag can stand alone or can have a value associated. A tag or value can contain alphanumeric
characters along with the characters `:`, `_` and `-`.

# Adding tags to files

The subcommand `add` lets you add or change tags on files. Use a comma separated list to add more
than one tag. Tag/value pairs are separated with `=`.

	$ xtagger add "ARM,Samsung,ARMFamily=ARM9E,ARMArchitecture=ARMv5TEJ,ARMCore=ARM926EJ-S" "Samsung S3C2416.pdf"

Here we add the standalone tags `ARM` and `Samsung` as well as the tag/value pairs `ARMFamily`,
`ARMArchitecture` and `ARMCore` with there associated values `ARM9E`, `ARMv5TEJ` and `ARM926EJ`
respectively to the single given file.

If there already was a `ARMArchitecture` tag with or without an associated value, the new value
replaces the old one or creates a new value entry.

xtagger has no specific support for tag-groups or hierarchies. But you can give more meaning to your
tags for example by using `:` to separate levels.

# Removing tags from files

To remove specific tags from files you use the `remove` subcommand. You can give a comma separated
list of tags to be removed.

	$ xtagger remove "ARM" "*.pdf"

This removes the tag `ARM` of all PDFs in the actual folder, using xtaggers own glob mechanism,
regardless of having an associated value or not.

To delete all tags of given files you can use

	$ xtagger delete *.pdf

# Listing tags from files

To simply list tags of files you use the `list` subcommand.

	$ xtagger list *.pdf

This shows the filename along with its tags:

	Samsung S3C2416.pdf
	  ARMArchitecture=ARMv5TEJ
	  ARMCore=ARM926EJ-S
	  ARMFamily=ARM9E

You can also use general tools like `xattr` on macOS or `getfattr` on Linux to see the extended
attributes associated with a file.

The extended attribute is called `user.xtag`.

# Finding files

You can find files on their tags with the `find` subcommand.

## Logical operations

xtagger supports the natural logic operators `AND` `OR` `NOT` along with their symbolic pendants
`&&` `||` and `!` respectively. The natural variants are case insensitive and need spaces around them
while the symbolic variants can be used without. In difference to other implementations these
operators have equal precedence while being left associative here. You can use parentheses to
influence the precedence as needed.

	$ xtagger find "Samsung and ARMFamily" *.pdf

This finds PDFs in the actual folder that have both the `Samsung` and `ARMFamily` tags, not caring
if any of them has an associated value or not.

## Equality/inequality

Equality uses `==` and is tested using the string representation. Inequality uses `<`, `<=`, `>=`
and `>`, while the values are converted to signed integers for comparison.

	$ xtagger find "Samsung and ARMFamily == ARM9E and Year >= 2006" *.pdf

Finds PDFs that have the `Samsung` tag with or without a value associated, the `ARMFamily` tag with
the value `ARM9E` associated and have the `Year` tag with an integer value bigger or equal to 2006.

## Regular Expressions

You can use regular expressions to find tags or values on patterns. As inequality tests convert to
integers you can't use regular expressions with them. Regular expressions always match a full tag or
value.

	$ xtagger find "ARM(Family|Core) == .*J.*" *.pdf

Will return files which have a 'J' in either value associated with the `ARMFamily` or `ARMCore` tags
(Which indicates MCUs with Jazelle technology).

# Known issues

* Please be aware that not all filesystems might be able to use extended attributes or your Linux
kernel doesn't have support compiled in the kernel at all. Also older implementations of NFS for
example might not support them and/or you might have to use special configuration/mount options to
enable them. In all cases you might loose extended attributes while moving files between filesystems
without notice. Please check your systems components documentations regarding extended attribute
support.

* Extended attributes might have filesystem specific size limits. These might be further limited
by other mechanisms using them. SELinux for example uses extended attributes for storing the files
security context.

* As there is no central database, finding information is, like using the `find` command, a more
costly operation limited in speed compared to database accesses.

* There is no central repository so there currently is no way to get a list of all used tags.

* xtaggers own glob mechanism can't use the `~` shorthand for the home directory.
