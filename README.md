[![Coverage Status](https://coveralls.io/repos/ihrwein/actiondb/badge.svg?branch=master&service=github)](https://coveralls.io/github/ihrwein/actiondb?branch=master)

# actiondb

Actiondb is a library and its associated tools to efficiently extract information from unstructured data. It's a tool
to parse logs and extract key-value pairs with predefined patterns from them.

## Patterns

A pattern is composed of literals and parsers, like:

```
Jun %{INT:day} %{INT:hour}:%{INT:min}:%{INT:sec} server sshd[%{INT:pid}]: Accepted publickey for joe
```

It can be used to parse the following log message:

```
Jun 25 14:09:58 server sshd[26665]: Accepted publickey for joe
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

#### Available parsers

#### SET

Parses only the characters which was given as its arguments. An optional
minimum and maximum length can be specified.

##### Example

```
%{SET("abcd",min_len=1,max_len=2):parsed_value_name}
```

It's identical to the `[abcd]{1,2}` regular expression.

#### INT

It reuses the `SET` parser with the character set of the numbers from `0` to
`9`. An optional minimum and maximum length can be specified.

#### GREEDY

It tries to fill in the gap between a parser and a literal or two literals. It will use
the following literal as an "end string" condition.

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

`adbtool` is a program which can be used for the following purposes:
* validate patterns
* parse unstructured texts
