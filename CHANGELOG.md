# Changelog
## Actiondb 0.4.0
* include pattern UUIDs in error messages
* check the pattern's uuid when testing its test messages
* make grammar use the ParserFactory to create parsers
* add MatcherFactory trait
* do not call pattern().unwrap() on an internal LiteralNode (too short match)
* add `-i` parameter to `adbtool validate` so all error messages will be
  printed out, not just the first one
* add `-d` parameter to `adbtool` to enable debug logs
* store `Node`s directly in `ParserNode` and `LiteralNode`
* add `pdb2adb` script to convert `PatternDB` patterns to `ActionDB` format
* add `MatcherSuite` trait
* count the parsed lines in `adbtool`
* other smaller refactors and improvements

Contributors: faxm0dem, ihrwein, lbudai

## Actiondb 0.3.1
* upgrade serde to 0.6

## Actiondb 0.3.0
* Plain pattern file support is removed
* CR characters can be used in patterns
* improved error messages
* upgrade to `serde 0.5`
* parsed messages can be tagged
* parsed messages can have additional key-value pairs
* the tests checks only the expected tags and values
* parser names are optional(like `%{GREEDY}`)

This release would not be possible without the help of Fabien Wernli. Thanks, Fabien!

## Actiondb 0.2.0
User visible changes:

* support JSON pattern files
* nicer and more precise error messages

Internal changes:
* `Matcher` becomes a trait and `ParserTrie` implements it
* the pattern reading and trie building code is extracted into a `Builder` struct
* `Builder` is able to populate any `Matcher` instance from any type which implements the `PatternSource` trait
 * `BuildResult = Result<Pattern, BuildError>`
 * `BuildError` contains all possible `Error` types (IO, pattern parse errors, etc.)
 * `PatternSource` is automatically implemented for every type which implements `Iterator<Item=BuildResult>`
 * this makes possible to generalize the `Matcher` building logic:
  * `BuildResult`s are being read from a `PatternSource` and if they are `Ok()` then they are added to the `Matcher`
  * in case of an `Err()` value the building process stops and the error is returned
* `Factory` is introduced to create `Matcher` instances from files (JSON)
 * `Factory::form_file()` is file extension agnostic and creates a `Matcher` instance from the given file
* the big modules are split into smaller submodules
* allow `.` character in `Parser` names
* the `JSON` files can contain test messages. They are tested when their pattern is added to the `Matcher`.
* `Coveralls.io` checks every modifications
* new unit tests are added
