use druid::
    {
        AppDelegate,   
        Command,
        DelegateCtx,
        Env,
        Handled,
        Selector,
        Target,
    };

use crate::ApplicationState;
use crate::presentation::Scene;

pub struct Delegate;

pub const SWITCH: Selector<Scene> = Selector::new("dungeon.switch");

impl AppDelegate<ApplicationState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut ApplicationState,
        _env: &Env,
    ) -> Handled {
        if let Some(scene) = cmd.get(SWITCH) {
            data.current_scene = scene.clone();
            Handled::Yes
        } else {
            println!("Command forwarded {:?}", cmd);
            Handled::No
        }
    }
}
