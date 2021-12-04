# Command Engine
An engine to create your own command-line interpreter. <br>
- Fully async
- Commands as user-defined structures
- Case-sensitive commands
- Positional arguments
- Optional non-positional arguments
- Partial "departure" from the conventions
- Built-in "*help*" support for commands

## Disclaimer
Versions `0.x.y` are not release ready, and might contain a lot of bugs. <br>

## Example
```rust
use command_engine::*;


pub struct Example;

impl CommandInfo for Example {
    (...)
}

#[async_trait]
impl Command for Example {
    (...)
}


fn main() {
    // Raw instruction in String 
    let raw = String::from("example help");
    
    // Creating a global Engine instance and returning the communicator
    let engine = EngineBuilder::new().build().unwrap();
    
    // Executing the instruction
    let out = engine.execute(&raw).await.unwrap();
    
    println!("StatusCode: '{}'\n{}", out.result, out.message)
}
```

## Output / Result
Every command should return an `Output` which indicates in the success or failure of the command execution.

`Output::new_ok` - Creates new successful output. First argument is a prime value indicating kind of the output (every status code should be documented in `on_help`). The second one is `Option` containing a message. 

`Output::new_ok` - Creates new failure output. Same arguments as for `new_ok`. 

Status code is a value with 2 detail information. First one is success or failure of the command and the second one is just a prime value.<br>
If you format the status code to HEX, you will see that successful outputs looks like: `0xA000` and the failed ones: `0xF000`.

Max value of prime value is `4095` as it would interfere with the status code "prefix". <br>
For example, successful command with prime value `4095` will look like: `0xAFFF` (`45_055`)

**Restricted Outputs:**
- `Error(0)` - Engine failure
- `Ok(0)` - Help has been called

## Help
Every command has a built-in argument "*help*" which will call `on_help` function that has to return a `String`. <br>
If `on_help` wasn't implemented for a command, it will always return the name of command along with a message like "*help is not implemented for this command*".

Help has a 4 different formatting options.

__**Structural:**__ <br>
Divided by 3 sections. If the command doesn't accept any arguments or o-arguments, the specific section won't be visible. <br>
Which section contains an argument name inside `[]`, but o-arguments can accept a sub argument. Optional ones will be in `[]` and non optional in `()`.
```
NAME:
	[name] - description

ARGS:
	[arg1] - arg1 description
	[arg2] - arg2 description

OARGS:
	[-oarg1] - oarg1 description
	[-oarg2 (sub_arg)] - oarg2 with sub_arg description
	[-oarg3 [sub_arg_optional]] - oarg3 with sub_arg_optional description
```

__**Compact:**__ <br>
One line help without description showing only the proper usage.<br>
first we have a command's name without any brackets, then there are positional arguments inside `()` and in the end o-arguments in `||`.<br>
Optional and non optional sub arguments are shown the same way as for the structural format. 
```
name (arg1) (arg2) |-oarg1| |-oarg2 (sub_arg)| |-oarg3 [sub_arg_optional]|
```

__**Descriptive:**__ *(WIP)* <br>
Not finished yet. Might change a little.
```
0.name => description
1.arg1 => arg1 description
2.arg2 => arg2 description
X.oarg1 => oarg1 description
X.oarg2 (sub_arg) => oarg2 with sub_arg description
X.oarg3 [sub_arg_optional] => oarg3 with sub_arg_optional description
```
