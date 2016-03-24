[![Coverage Status](https://coveralls.io/repos/ihrwein/actiondb/badge.svg?branch=master&service=github)](https://coveralls.io/github/ihrwein/actiondb?branch=master)

# actiondb

Actiondb is a library and its associated tools to efficiently extract information from unstructured data. It's a tool
to parse logs and extract key-value pairs with predefined patterns from them.

The patterns can be specified in a JSON or YAML serialized file. Their schema is the same, only
the format is different.
The format allows you to give a name, a unique identifier (`UUID`) to each pattern and to test message parsing with example messages.

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

An example test message object can be seen in the following example (in YAML):

```yaml
patterns:
  -
    uuid: "6d2cba0c-e241-464a-89c3-8035cac8f73e"
    name: "LOGGEN"
    pattern: "seq: %{INT:.loggen.seq}, thread: %{INT:.loggen.thread}, runid: %{INT:.loggen.runid}, stamp: %{GREEDY:.loggen.stamp} %{GREEDY:.loggen.padding}"
    values:
      foo: "bar"
    tags:
      - "foo"
      - "bar"
    test_messages:
      -
        message: "seq: 0000000001, thread: 0000, runid: 1437655178, stamp: 2015-07-23T14:39:38 PADDPADDPADDPADD"
        values:
          .loggen.seq: "0000000001"
          .loggen.thread: "0000"
          .loggen.runid: "1437655178"
          .loggen.stamp: "2015-07-23T14:39:38"
          .loggen.padding: "PADDPADDPADDPADD"
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
* parse text files.

It support the `validate` and `parse` subcommands. For more information check
it's `--help` option.

## [Changelog](CHANGELOG.md)
