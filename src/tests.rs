use crate::*;
use crate::shared::*;

struct Test;
impl Command for Test {
    fn name(&self) -> &str {
        "test"
    }

    fn on_help(&self) -> Help {
        Help::new(&self.name(), "testing sync command")
            .add_arg(".", "unused argument")
            .add_oarg("-.", "unused o-argument", None)
            .add_oarg("-..", "unused o-argument\nwith unused sub-argument", Some(SubArg::new("unused", true)))
            .set_display_compact()
    }

    fn on_execute(&mut self, ins: &Instruction) -> Output {
        Output::new_ok(
            0,
            Some(format!(
                "name[{}]args[{}]oargs[{}]",
                ins.value,
                ins.args.len(),
                ins.oargs.len()
            ))
        )
    }
}


#[test]
fn engine_sync_execution() {
    let mut engine = Engine::new().add(Test);
    let output = engine.execute("test . -. -.. -.. a");

    assert_eq!(
        output.message,
        String::from("name[test]args[1]oargs[2]"),
    );
}

#[test]
fn engine_sync_help() {
    let mut engine = Engine::new().add(Test);
    let output = engine.execute("test help");

    assert_eq!(
        output.message,
        "test (.) |-.| |-.. [unused]|",
    );
}
