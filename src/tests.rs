use crate::*;
use shared::{Instruction, Help, Output, SubArg};

struct Test;
impl Command for Test {
    fn name(&self) -> &str {
        "test"
    }

    fn on_help(&self, _: &Instruction) -> String {
        Help::new(&self.name(), "testing sync command")
            .add_arg(".", "unused argument")
            .add_oarg("-.", "unused o-argument", None)
            .add_oarg("-..", "unused o-argument\nwith unused sub-argument", Some(SubArg::new("unused", true)))
            .format_compact()
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

#[tokio::test]
#[cfg(feature = "async")]
#[cfg(feature = "c-fs")]
async fn engine_async() {
    use shared::commands::fs::Cwd;

    let mut engine = AsyncEngine::new()
        .add(Cwd::new());

    assert!(engine.execute("cwd").await.result.is_ok())
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
fn engine_sync_duplicated_name() {
    let mut engine = Engine::new().add(Test);

    assert_eq!(
        engine.add_separated(Test).is_err(),
        true,
    );
}
