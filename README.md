This is a toy implementation for `serde_json`, which implements serialization and deserialization using proc macros.

# Implementation

Declarative macros:

`serialize_trait!()` and `deserialize_trait!()`are two boilerplate macros that writes the trait definition and impl for primitives and enums (for this toy impl just call these at top). Importantly this also contains the `.parse_json()` (you have to invoke this before deserializing) method that converts any serialized json string to a `JsonValue`.

For Serialization:

This works by using `Serialize` proc macro to implement the `Serializer` trait which includes the `.to_str()` method which further includes a method body which is constructed at compile time and walks through the fields of the `TokenStream` (struct's AST) input fields and matches the data type (all 14 primitives + String + Vec + HashSet + HashMap) to appends valid json in the result, for a field item that's another struct's instance it expect that nested/child struct to also be serialized hence the parent instance calling the `.to_str()` returns a valid json string which it can further append to it's own json string.

For generics we check if it includes generic params and stores the generic types, then add a simple generic token + where clause to the macro.

For Deserialization:

This works by implementing the visitor pattern. The `Deserialize` proc macro walks the struct's fields and generates a `__{struct_name}Visitor` type and a `Visitor` trait impl for it. It uses `.visit_map()` method (`.visit_seq()` for unnamed) for named fields (top-level json is basically just a kv map) which extracts all fields name and calls `<FieldType as Deserialize>::deserialize()` on it recursively, so child/nested structs work as long as they have `Deserialize` on thme. The `Deserialize` impl for the struct just matches the `JsonValue` variant and calls the visitor. Lastly this impls `Deserializer`'s `.derserialize()` trait on the struct so we can invoke it.

For generics the visitor have a `PhantomData<(T, U, ...)>` to satisfy the compiler cus the type
params aren't actually stored, and the impl gets the same generic tokens + where clause as the Serializer + Deserializer.

For this flow its kinda like this: json string -> `.parse_json()` -> `JsonValue` -> `T::deserialize()` -> `T`

Parser is just a simple recursive descent parser.

hope it was was somewhat helpful. :/
