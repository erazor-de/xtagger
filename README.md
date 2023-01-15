# xtagger - find files using tags

xtagger is a console application that lets you manage and find tags on your files. While similar
applications like [TMSU](https://tmsu.org/) and [Tagsistant](https://www.tagsistant.net/) use
databases to manage the file/tag relationships, others like [TagSpaces](https://www.tagspaces.org/)
use sidecar files or encode tags in the filename. xtagger is different in that it uses the extended
attributes of supporting file systems on Linux and macOS (Windows alternate data streams are not
supported yet).

This fixes one problem common with most other solutions: You can move or rename your files with your
favourite tools without loosing the file/tag relationship managed elsewhere[^1].

## Usage

Install this console application with

```console
$ cargo install xtagger
```
## General

To get general help about xtagger and its command line interface you can issue

```console
$ xtagger --help
```
xtagger supports Perl-style regular expressions but has no look-around support.

The order of actions xtagger executes is fixed and given as follows.
- filter
- copy
- rename
- remove
- add
- output
- delete

## Tags

A tag in xtagger can stand alone or can have a value associated. A tag or value can contain
alphanumeric characters along with the characters `:`, `_` and `-`. Tags and values are always
handled case sensitive.

xtagger stores all tags in an extended attribute with the name `user.xtag`. This means you can use
general tools which list extended attributes associated with a file like `xattr` on macOS or
`getfattr` on Linux to see if a file has xtag attributes.

The order of manipulations is rename, remove and add. All manipulations are done before output
and the delete action is done afterwards. You can always use the `--dry_run` option to check what
the command would do but nothing gets changed.

### Adding tags to files

The option `--add` lets you add or change tags on files. Use a comma separated list to add more
than one tag. Tag/value pairs are separated with `=`.

```console
$ xtagger --add "ARM,Samsung,ARMFamily=ARM9E,ARMArchitecture=ARMv5TEJ,ARMCore=ARM926EJ-S" "Samsung S3C2416.pdf"
```
Here we add the standalone tags `ARM` and `Samsung` as well as the tag/value pairs `ARMFamily`,
`ARMArchitecture` and `ARMCore` with there associated values `ARM9E`, `ARMv5TEJ` and `ARM926EJ`
respectively to the single given file.

If there already is an `ARMArchitecture` tag with or without an associated value, the new value
replaces the old one or creates a new value entry.

xtagger has no specific support for tag-groups or hierarchies. But you can give more meaning to your
tags for example by using `:` to separate levels.

### Removing tags from files

To remove specific tags from files you use the `--remove` option. You can give a comma separated
list of tags to be removed.

```console
$ xtagger --remove "ARM" "*.pdf"
```
This removes the tag `ARM` of all PDFs in the actual folder, using xtaggers own glob mechanism,
regardless of having an associated value or not.

To delete all tags along with the whole extended attribute of given files you can use

```console
$ xtagger --delete *.pdf
```
### Renaming tags

The `--find` and `--replace` options let you rename existing tags also with regular expression
replacements.

```console
$ xtagger --find "ARM" --replace "Risc-V" *.pdf
```
Simply replaces the `ARM` tag with `Risc-V` in the given files, keeping any associated value.

```console
$ xtagger --find "ARM(.*)" --replace "Risc-V\$1" *.pdf
```
Uses a capture group to rename tags like `ARMFamily` to `Risc-VFamily`. Please note the escaped $
sign in the replace pattern. This is needed to keep the shell from replacing this with environment
variables. You can alternatively use single quotes.

This rename mechanism also supports named capture groups.

### Copy tags

The `--copy` option lets you copy the tags from the first handled file to all other files.

## Selecting files

To define the files xtagger should work on you can use multiple normal shell globs. For larger file
amounts xtagger supports its own glob mechanism. Just put the glob in quotation marks[^2]. Should you
need special selection criteria you might use xtagger as exec target of the find command for example.

## Filter files

You can filter the files given on their tags with the `--filter` or `--bookmark` option. As there is
no central database, finding information is a more time consuming search operation depending on file
system speed.

### Conditional operations on tags

xtagger supports the natural conditional operators `AND` `OR` `NOT` along with their symbolic pendants
`&&` `||` and `!` respectively. The natural variants are case insensitive and need spaces around them
while the symbolic variants can be used without. The `AND` operator has higher precedence than `OR`.
You can use parentheses to influence the precedence as needed.
The `AND` and `OR` operators are left associative and use short-circuit evaluation.

```console
$ xtagger --filter "Samsung and ARMFamily" --list *.pdf
```
This finds PDFs in the actual folder that have both the `Samsung` and `ARMFamily` tags, not caring
if any of them has an associated value or not.

### Equality/inequality/relational operations on values

Equality and inequality use `==` and `!=` respectively and are tested using the string representation
of the values. Relations use `<`, `<=`, `>=` and `>`, while the values are converted to signed
integers for comparison.

```console
$ xtagger --filter "Samsung and ARMFamily == ARM9E and Year >= 2006" --list *.pdf
```
Finds PDFs that have the `Samsung` tag with or without a value associated, the `ARMFamily` tag with
the value `ARM9E` associated and have the `Year` tag with an integer value bigger or equal to 2006.

If you use regular expressions for tags then `==` matches if at least one tag has a matching value.
The `!=` operator matches if not a single value matches.

### Regular Expressions

You can use regular expressions to find tags or values on patterns. As inequality tests convert to
integers you can't use regular expressions with them. Regular expressions always match a full tag or
value.

```console
$ xtagger --filter "ARM(Family|Core) == .*J.*" --list *.pdf
```
Will return files which have a 'J' in either value associated with the `ARMFamily` or `ARMCore` tags.

## Bookmarks

Bookmarks in xtaggers sense are just symbolic links in the filesystem. They can be used to store
recurring searches or to provide synonyms.

### Creating Bookmarks

xtagger has no specific support to create bookmarks. Use standard tools to create one. For example
```console
$ ln -s "ARM(Family|Core) == .*J.*" Jazelle
```
creates a symbolic link with the name Jazelle with the filter term as target.
```console
$ ln -s "DarkGreen or ForestGreen or Green" Green
creates synonyms for the color green.

```
With a command like
```console
$ ls -l *
```
you can list your bookmarks along with their filter terms.

### Using Bookmarks

Bookmarks can be easily used standalone like this:
```console
$ xtagger --bookmark Jazelle --list *.pdf
```
instead of directly using a filter term, or you can add them to your search term.
```console
$ xtagger --filter "{Green} and Round" --list *.pdf
```
This way the bookmark is included with implicit parentheses.

## Output options

The files that match xtaggers filter can be viewed in multiple ways. You can for example always add
`--list` to get a list of files that have been manipulated in this call.

If you are working interactively with a graphical user interface using a terminal that supports
hyperlinks, you can add the `-h` option. Then xtagger will create hyperlinks for
the listed files, so you can open them with a mouseclick.

### Only list files

The option `--list` will simply list all matching files, one per line.

### Show files with tags

To list files along with their tags you use the `--show` option.

```console
$ xtagger --show -h *.pdf
```
This shows the filenames with hyperlink support along with its tags and values, one per line and
indented:

```
Samsung S3C2416.pdf
  ARMArchitecture=ARMv5TEJ
  ARMCore=ARM926EJ-S
  ARMFamily=ARM9E
```
### List used tags

To just list all used tags in a set of files you use the `--tags` option.

```console
$ xtagger --tags *.pdf
```
will list the tags in alphabetical order, one per line.

## Platform support

Works on Linux and macOS.

## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).

## Footnotes

[^1]: Please be aware that not all filesystems might be able to use extended attributes or your Linux
	kernel doesn't have support compiled in the kernel at all. Also older implementations of NFS for
	example might not support them and/or you might have to use special configuration/mount options
	to enable them. In all cases you might loose extended attributes while moving files between
	filesystems without notice. Please check your systems components documentations regarding
	extended attribute support.

	Also extended attributes might have filesystem specific size limits. These might be further
	limited by other mechanisms using them. SELinux for example uses extended attributes for storing
	the files security context.
[^2]: xtaggers own glob mechanism can't use the `~` shorthand for the home directory.
