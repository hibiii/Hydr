use crate::state::State;

pub fn display_progress(state: &State) {
    println!(
        "Progress: {}/{} ({}%)",
        state.progress,
        state.goal,
        (state.progress as f32 * 100f32) / state.goal as f32
    )
}

pub fn parse_millis(string: Option<&String>) -> Result<u32, &'static str> {
    if string == None {
        return Err("expected number, none given");
    }
    string
        .unwrap()
        .parse()
        .map_err(|_| "expected number, could not parse")
}
