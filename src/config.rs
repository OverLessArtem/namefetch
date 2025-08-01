pub const BLACK: &str = "\x1b[0;30m";
pub const LIGHT_GRAY: &str = "\x1b[0;37m";
pub const WHITE: &str = "\x1b[1;37m";
pub const LIGHT_BLUE: &str = "\x1b[1;34m";
pub const NC: &str = "\x1b[0m";

pub const GAP: usize = 4;
pub const MAX_KEY_LENGTH: usize = 22;

pub fn get_logo() -> Vec<String> {
    let accent = LIGHT_BLUE;
    let nc = NC;

    vec![
        format!("     {accent}███████{nc}"),
        format!("   {accent}██{nc}░░░░░░░░{accent}██{nc}"),
        format!("  {accent}██{nc}░░{accent}██{nc}░░{accent}██{nc}░░{accent}██{nc}"),
        format!("  {accent}██{nc}░░░░░░░░░░{accent}██{nc}"),
        format!("   {accent}██{nc}░░░░░░░░{accent}██{nc}"),
        format!("     {accent}████████{nc}"),
        format!("   {accent}██ ██{nc}"),
        format!("  {accent}█{nc}░░{accent}█ █{nc}░░{accent}█{nc}"),
        format!("  {accent}█{nc}░░{accent}█ █{nc}░░{accent}█{nc}"),
        format!("  {accent}█{nc}░░{accent}██████{nc}░░{accent}█{nc}"),
        format!("  {accent}█{nc}░░░░░░░░░░{accent}█{nc}"),
        format!("  {accent}█{nc}░░░░░░░░░░{accent}█{nc}"),
        format!("  {accent}██ ██{nc}"),
        format!("  {accent}██ ██{nc}"),
        format!("  {accent}██ ██{nc}"),
        format!("  {accent}██ ██{nc}"),
    ]
}
