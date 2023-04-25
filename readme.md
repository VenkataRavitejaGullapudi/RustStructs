# Structs in Rust
- A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group.
- It is similar to object's data attributes in a oop paradigm.
- Structs are similar to tuples which can hold multiple related values of different types. Along with it, you can name each piece of data which can be more clear. Also you don’t have to rely on the order of the data to specify or access the values of an instance.
- Methods and associated functions are defined in implementation blocks. Struct allows us to define multiple implementation blocks.
- In Rust, struct methods and data can be accessed by dot notation(.). And this applies for even the referenced structs as rust performs auto referencing and auto dereferencing. So you can access the data of struct pointers/references which are pointing to a struct just using the dot notation.
## Methods in struct
- Similar to functions.
- Unlike the functions, the methods are defined in the context of structs (or enum or traits) and whose first parameter is always self which represents the instance of struct the method is being called.

## Associated functions in struct
- Unlike methods, they are not tied to an instance and so they don't get self as first param.
- These associated functions are namespaced by struct.
- The associated functions in struct and namespaces in packages can be accessed with "::" operator instead of dot notation.