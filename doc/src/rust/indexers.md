Custom Type Indexers
===================

{{#include ../links.md}}

A custom type can also expose an _indexer_ by registering an indexer function.

A custom type with an indexer function defined can use the bracket '`[]`' notation to get a property value.

Indexers are disabled when the [`no_index`] feature is used.

For efficiency reasons, indexers **cannot** be used to overload (i.e. override) built-in indexing operations for
[arrays] and [object maps].

```rust
#[derive(Clone)]
struct TestStruct {
    fields: Vec<i64>
}

impl TestStruct {
    fn get_field(&mut self, index: i64) -> i64 {
        self.fields[index as usize]
    }
    fn set_field(&mut self, index: i64, value: i64) {
        self.fields[index as usize] = value
    }

    fn new() -> Self {
        TestStruct { fields: vec![1, 2, 3, 4, 5] }
    }
}

let mut engine = Engine::new();

engine.register_type::<TestStruct>();

engine.register_fn("new_ts", TestStruct::new);

// Shorthand: engine.register_indexer_get_set(TestStruct::get_field, TestStruct::set_field);
engine.register_indexer_get(TestStruct::get_field);
engine.register_indexer_set(TestStruct::set_field);

let result = engine.eval::<i64>("let a = new_ts(); a[2] = 42; a[2]")?;

println!("Answer: {}", result);                     // prints 42
```
