pub struct ViewerToken {
    _p: (),
}
pub struct TechToken {
    _p: (),
}
pub struct EngineerToken {
    _p: (),
}

pub trait CanView {}
pub trait CanDiag: CanView {}
pub trait CanFlash: CanDiag {}

impl CanView for ViewerToken {}
impl CanView for TechToken {}
impl CanView for EngineerToken {}
impl CanDiag for TechToken {}
impl CanDiag for EngineerToken {}
impl CanFlash for EngineerToken {}

pub fn read_status(_tok: &impl CanView) -> String {
    "status: OK".to_string()
}

pub fn run_quick_diag(_tok: &impl CanDiag) -> String {
    "diag: PASS".to_string()
}

pub fn flash_firmware(_tok: &impl CanFlash) {}

fn main() {}
