# Advanced Scripting Language (ASL)

An extention to my previous language project

## Requirements

- All features from SSL
- Removal of statements
- Strict Typing
- Compiled Using LLVM
- Primatives and Structs
- Loops
- Functions
- Arrays

## Example

```rust
struct Person {
  name: String
}


let names = [
  Person{name: "dan"},
  Person{name: "dave"},
  Person{name: "daniel"},
]


fn say_hello(name: Person) -> String {
  return "Hello, " + name
}

let hellos = for name in names {
  say_hello(name)
}

print(hellos)
```
