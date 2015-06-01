# actiondb

## Rust things

### Turn on debug logs while running tests
Set the `RUST_LOG` environment variable:

```
RUST_LOG=debug cargo test
```

### You need to move out a resource from &mut self

You can do this by destructoring it via a `let` binding. The destructoring
function (like `split()`) takes `self`, not a reference. Then it can destructor
it.

### Reference has a longer lifetime than the data it references

You can extend a lifetime with the following syntax:

```rust
struct LiteralLookupHit<'a, 'b: 'a, 'c>(&'a mut Node<'b>, &'c str);
```
