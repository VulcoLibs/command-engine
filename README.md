# Command Engine
Transform string instructions into code execution. <br>

## Disclaimer
#### ToDo _(before `1.0.0-rc.1`)_:
- [x] ~~Support for escape characters in input.~~ **_[Added collector]_**
- [ ] Remove `--` from deserialized flags?

#### Versioning:
- `*.*.*` - Released.
- `*.*.*-rc.*` - Release Candidate.
- `*.*.*-dev` - Unreleased in production.
- `0.*.*` - Deprecated.

## Engine
This crate provides a default engine, that is present if default features are enabled.
It is simply a container for your Commands that can also execute them based on the input.

There are 2 versions of the default Engine:
- `sync` - Default.
- `async` - Enabled with `async` feature.

## Instruction
The given input is deserialized into a specific format defined in `Instruction` structure.

It allows for one Command `caller`, multiple positional `args`, additional `o_args` which act like flags 
that can contain `sub_args`.

Format:
```pseudo
<caller> <arg> <arg> --<o_arg> <sub_arg> <sub_arg> --<o_arg>
```

Example:
```pseudo
example argument1 argument2 --flag2 child1 child2 --flag3 --flag1
```

Deserializes to:
```rust
Instruction {
    caller: "example",
    args: vec!["argument1", "argument2"],
    o_args: {
        let mut map = HashMap::new();
        map.insert("--flag2", Some(vec!["child1", "child2"]));
        map.insert("--flag3", None);
        map.insert("--flag1", None);
        map
    },
    input: "example argument1 argument2 --flag2 child1 child2 --flag3 --flag1",
};
```

To add spaces in arguments use double quotes `"`:
```pseudo
example "argument 1" "--flag 2" "child 1"
```
If there are double quotes in the argument it's suggested to use collector `#`:
```pseudo
example #"argument "quotes" 1"#
```

There are no escape characters to avoid any heap allocations. Each argument is a string slice taken from the input.

## Example
Sync version:
```rust
use command_engine::*;

pub struct Example;

impl CommandInfo for Example {
    fn caller(&self) -> &'static str {
        "ex"
    }
}

impl Command for Example {
    type Output = String;

    fn on_execute(&self, ins: Instruction) -> Self::Output {
        format!("{:?}", ins)
    }
}

fn main() {
    let input = "ex arg --o_arg sub_arg";
    
    let mut engine = Engine::new();
    engine.insert(Example);
    
    // Will return formatted string of the Instruction.
    let output = engine.execute(input).unwrap();
    println!("{}", output);
}
```
