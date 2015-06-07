# actiondb

## Rust things

### Run only one test without muting stdout

```
cargo test -- --nocapture matcher::trie::node::node::given_empty_trie_when_literals_are_inserted_then_they_can_be_looked_up
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

### Cannot borrow mutable X multiple times, but there is only one active borrow
Check your lifetimes. If there is one on `self` it can cause problems. If a trait
doesn't need a litetime just one of its methods, then place the lifetime on the method
and not on the trait.

