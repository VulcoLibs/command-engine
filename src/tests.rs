use crate::Instruction;

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
}
