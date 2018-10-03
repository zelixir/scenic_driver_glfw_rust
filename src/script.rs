use types::*;

pub fn run_scripts<'a>(
    window_data: &mut WindowData,
    script_id: u32,
    ctx: &mut Context,
    frame: &mut ::nanovg::Frame<'a>,
) {
    match window_data.scripts.get(&script_id) {
        Some(script) => {
            
        }
        _ => (),
    };
}
