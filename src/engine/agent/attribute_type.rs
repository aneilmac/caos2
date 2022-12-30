pub struct AttributeType(pub i32);

impl AttributeType {
    pub const CARRYABLE: AttributeType = AttributeType(1);
    pub const MOUSEABLE: AttributeType = AttributeType(2);
    pub const ACTIVATABLE: AttributeType = AttributeType(4);
    pub const GREEDY_CABIN: AttributeType = AttributeType(8);
    pub const INVISIBLE: AttributeType = AttributeType(16);
    pub const FLOATABLE: AttributeType = AttributeType(32);
    pub const SUFFER_COLLISIONS: AttributeType = AttributeType(64);
    pub const SUFFER_PHYSICS: AttributeType = AttributeType(128);
    pub const CAMERA_SHY: AttributeType = AttributeType(256);
    pub const OPEN_AIR_CABIN: AttributeType = AttributeType(512);
}
