pub struct Vbusin0EventMask;

impl Vbusin0EventMask {
    pub const VBUS_DETECTED: u8 = 1 << 0;
    pub const VBUS_REMOVED: u8 = 1 << 1;
    pub const OVRVOLT_DETECTED: u8 = 1 << 2;
    pub const OVRVOLT_REMOVED: u8 = 1 << 3;
    pub const UNDERVOLT_DETECTED: u8 = 1 << 4;
    pub const UNDERVOLT_REMOVED: u8 = 1 << 5;
}
