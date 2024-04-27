use crate::{Command, CommandInfo, Engine, Instruction};

#[cfg(feature = "async")]
use crate::{OutputFuture, IntoOutputFuture};

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )+
            map
        }
     };
);

#[test]
fn instruction() {
    let test_instruction = |input: &'static str, mut instruction: Instruction| {
        instruction.input = input;
        let inst_new = Instruction::new(input).expect("Failed to create a valid Instruction!");
        assert_eq!(inst_new, instruction, "Provided instruction is not the same as one crafted from the input!");
    };

    test_instruction("caller", Instruction {
        caller: "caller",
        .. Default::default()
    });

    test_instruction("caller arg1 arg2", Instruction {
        caller: "caller",
        args: vec!["arg1", "arg2"],
        .. Default::default()
    });

    test_instruction("caller arg1 arg2 --o_arg1", Instruction {
        caller: "caller",
        args: vec!["arg1", "arg2"],
        o_args: map!("--o_arg1" => None),
        .. Default::default()
    });

    test_instruction("caller arg1 arg2 --o_arg1 sub_arg11 sub_arg12 --o_arg2 sub_arg21", Instruction {
        caller: "caller",
        args: vec!["arg1", "arg2"],
        o_args: map!(
            "--o_arg1" => Some(vec!["sub_arg11", "sub_arg12"]),
            "--o_arg2" => Some(vec!["sub_arg21"])
        ),
        .. Default::default()
    });

    test_instruction(r#"caller "arg 1""#, Instruction {
        caller: "caller",
        args: vec!["arg 1"],
        .. Default::default()
    });

    test_instruction(r#"caller --o_arg1 "sub arg""#, Instruction {
        caller: "caller",
        o_args: map!(
            "--o_arg1" => Some(vec!["sub arg"])
        ),
        .. Default::default()
    });

    test_instruction(r#"caller "--o_arg 1""#, Instruction {
        caller: "caller",
        o_args: map!(
            "--o_arg 1" => None
        ),
        .. Default::default()
    });

    test_instruction(r#"caller "arg 1"arg2"#, Instruction {
        caller: "caller",
        args: vec!["arg 1", "arg2"],
        .. Default::default()
    });

    test_instruction("caller #\"arg 1\"# arg2", Instruction {
        caller: "caller",
        args: vec!["arg 1", "arg2"],
        .. Default::default()
    });

    test_instruction("caller #\"arg#1\"# arg2", Instruction {
        caller: "caller",
        args: vec!["arg#1", "arg2"],
        .. Default::default()
    });

    test_instruction("caller #\"arg\" # \" 1\"# \"arg 2\"", Instruction {
        caller: "caller",
        args: vec!["arg\" # \" 1", "arg 2"],
        .. Default::default()
    });

    test_instruction("caller #\"arg 1\"#arg2", Instruction {
        caller: "caller",
        args: vec!["arg 1", "arg2"],
        .. Default::default()
    });
}

#[test]
fn command() {
    const CALLER: &str = "test";

    struct TestCommand;

    impl CommandInfo for TestCommand {
        fn caller(&self) -> &'static str {
            CALLER
        }
    }

    impl Command for TestCommand {
        type Output = String;

        #[cfg(feature = "async")]
        fn on_execute<'a>(&self, ins: Instruction<'a>) -> OutputFuture<'a, Self::Output> {
            async move {
                ins.caller.to_string()
            }.output_future()
        }

        #[cfg(not(feature = "async"))]
        fn on_execute<'a>(&self, ins: Instruction<'a>) -> Self::Output {
            ins.caller.to_string()
        }
    }

    let cmd = TestCommand;
    let ins = Instruction {
        caller: CALLER,
        .. Default::default()
    };

    #[cfg(feature = "async")]
    let output = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(cmd.on_execute(ins));

    #[cfg(not(feature = "async"))]
    let output = cmd.on_execute(ins);

    assert_eq!(CALLER, cmd.caller(), "TestCommand has different caller!");
    assert_eq!(CALLER, output, "TestCommand returned different caller as output!");
}

#[test]
fn engine() {
    const EXPECTED_OUTPUT: &str = r#"Instruction { caller: "fmt", args: ["arg"], o_args: {"--o_arg": Some(["sub_arg"])}, input: "fmt arg --o_arg sub_arg" }"#;

    struct FormatCommand;

    impl CommandInfo for FormatCommand {
        fn caller(&self) -> &'static str {
            "fmt"
        }
    }

    impl Command for FormatCommand {
        type Output = String;

        #[cfg(feature = "async")]
        fn on_execute<'a>(&self, ins: Instruction<'a>) -> OutputFuture<'a, Self::Output> {
            async move {
                format!("{:?}", ins)
            }.output_future()
        }

        #[cfg(not(feature = "async"))]
        fn on_execute<'a>(&self, ins: Instruction<'a>) -> Self::Output {
            format!("{:?}", ins)
        }
    }

    #[cfg(feature = "async")]
    let runtime = tokio::runtime::Runtime::new().expect("Tokio failed to create a runtime!");

    let mut engine = Engine::new();
    assert!(engine.is_empty(), "Engine should be empty if no commands were added!");

    engine.insert(FormatCommand);
    assert!(!engine.is_empty(), "Engine shouldn't be empty if commands were added!");

    #[cfg(feature = "async")]
    let output = runtime.block_on(
        engine.execute("fmt arg --o_arg sub_arg")
    ).expect("Valid instruction should execute without error!");

    #[cfg(not(feature = "async"))]
    let output = engine.execute("fmt arg --o_arg sub_arg").expect("Valid instruction should execute without error!");

    assert_eq!(EXPECTED_OUTPUT, output);
}
