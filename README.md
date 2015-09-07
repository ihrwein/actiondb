[![Coverage Status](https://coveralls.io/repos/ihrwein/actiondb/badge.svg?branch=master&service=github)](https://coveralls.io/github/ihrwein/actiondb?branch=master)

# actiondb

Actiondb is a library and its associated tools to efficiently extract information from unstructured data. It's a tool
to parse logs and extract key-value pairs with predefined patterns from them.

The patterns can be specified in a JSON serialized file.
The latter one allows you to give a name, a unique identifier (`UUID`) and to test messages to each pattern.

This library is intended to used with `syslog-ng`. You can find a Docker image which uses this library
as a parser:

[actiondb-docker](https://github.com/ihrwein/actiondb-docker)

## [Patterns](#patterns)

A pattern is composed of literals and parsers, like:

```
Jun %{INT:day} %{INT:hour}:%{INT:min}:%{INT:sec} server sshd[%{INT:pid}]: Accepted publickey for joe
```

It can be used to parse the following log message:

```
Jun 25 14:09:58 server sshd[26665]: Accepted publickey for joe
```

### JSON pattern files
These files contains patterns and their attributes. A JSON file looks like the following example

```json
{
  "patterns": [
    {
      "uuid": "c11c806a-766d-4a09-9f24-7de1fe02e51e",
      "name": "SSH_PUBKEY",
      "pattern": "Jun %{INT:day} %{INT:hour}:%{INT:min}:%{INT:sec} lobotomy sshd[%{INT:pid}]: Accepted publickey for zts from %{INT:oct0}.%{INT:oct1}.%{INT:oct2}.%{INT:oct3} port %{INT:port} ssh2"
    },
    {
      "name": "SSH_DISCONNECT",
      "uuid": "9a49c47d-29e9-4072-be84-3b76c6814743",
      "pattern": "Jun %{INT:day} %{INT:hour}:%{INT:min}:%{INT:sec} lobotomy sshd[%{INT:pid}]: Received disconnect from %{GREEDY:ipaddr}: %{INT:dunno}: disconnected by user"
    },
    {
      "uuid": "fa8bdbcb-e0fd-4da1-9fa4-15ecfec28ad2",
      "pattern": "Jun %{INT:day} %{INT:hour}:%{INT:min}:%{INT:sec} lobotomy sshd[%{INT:pid}]: pam_unix(sshd:session): session closed for user zts"
    }
  ]
}
```

It has the following structure:
* `patterns`: it's a top level array of pattern objects

A pattern object consists of the following key-value pairs:
* `uuid`: it's a required field and contains a UUID,
* `name`: it's an optional field and contains the name of the pattern. Currently there is no restriction about the valid character set.
* `pattern`: it's the same thing as defined in [Patterns](#patterns)
* `values`: it's an optional field and contains additional key-value pairs which should be added to the matching message
* `tags`: it's and optional array and contains tags which should be added to the matching message
* `test_messages`: it's an array of test messages which can be used to test the patters.

A test message object has the following key-value pairs:
* `message`: a string message which should be parsed,
* `values`: an object which defines the expected key-value pairs after the parsing. Every key and value
 must be strings.
* `tags`: the expected tags

An example test message object can be seen in the following example:

```json
{  
  "patterns":[  
    {  
      "uuid":"6d2cba0c-e241-464a-89c3-8035cac8f73e",
      "name":"LOGGEN",
      "pattern":"seq: %{INT:.loggen.seq}, thread: %{INT:.loggen.thread}, runid: %{INT:.loggen.runid}, stamp: %{GREEDY:.loggen.stamp} %{GREEDY:.loggen.padding}",
      "values": {
        "foo": "bar"
      },
      "tags": ["foo", "bar"],
      "test_messages":[  
        {  
          "message":"seq: 0000000001, thread: 0000, runid: 1437655178, stamp: 2015-07-23T14:39:38 PADDPADDPADDPADD",
          "values":{  
            ".loggen.seq":"0000000001",
            ".loggen.thread":"0000",
            ".loggen.runid":"1437655178",
            ".loggen.stamp":"2015-07-23T14:39:38",
            ".loggen.padding":"PADDPADDPADDPADD"
          }
        }
      ]
    }
  ]
}
```

### Parsers

Parsers can be used to extract data from unstructured text.

Every parser has the following syntax:

```
%{PARSER_TYPE(required_arg1, required_arg2, optional_arg1="value", optional_arg2=value):parser_instance_name}
```

If a parser doesn't have extra arguments its parameter list can be omitted:

```
%{PARSER_TYPE:parser_instance_name}
```

The `name` can be omitted too:

```
%{PARSER_TYPE}
```

You can use the `_`, `.`, `[0-9]`, `-` and `[a-zA-Z]` characters as parser names.

#### Available parsers

#### [SET](#set)

Parses only the characters which was given as its arguments. An optional
minimum and maximum length can be specified.

##### Example

```
%{SET("abcd",min_len=1,max_len=2):parsed_value_name}
```

It's identical to the `[abcd]{1,2}` regular expression (but faster).

#### INT

It reuses the `SET` parser with the character set of the numbers from `0` to
`9`. An optional minimum and maximum length can be specified as in [SET](#set).

#### GREEDY

It tries to fill in the gap between a parser and a literal or two literals. It will use
the next literal as an "end string" condition. If the `GREEDY` parser is the last parser
in the pattern it will consume the whole remaining message.

##### Example

Pattern:
```
from %{GREEDY:ipaddr}: %{INT:dunno}
```
Sample message:
```
from 1.2.3.4: 123
```
Extracted key-value pairs:
* `(ipaddr,1.2.3.4)`
* `(dunno,123)`

### adbtool

`adbtool` is a tool which can be used for the following purposes:
* validate patterns,
* parse texts.

It support the `validate` and `parse` subcommands. For more information check
it's `--help` option.

## Changelog
### Actiondb 0.3.1
* upgrade serde to 0.6

### Actiondb 0.3.0
* Plain pattern file support is removed
* CR characters can be used in patterns
* improved error messages
* upgrade to `serde 0.5`
* parsed messages can be tagged
* parsed messages can have additional key-value pairs
* the tests checks only the expected tags and values
* parser names are optional(like `%{GREEDY}`)

This release would not be possible without the help of Fabien Wernli. Thanks, Fabien!

### Actiondb 0.2.0
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
