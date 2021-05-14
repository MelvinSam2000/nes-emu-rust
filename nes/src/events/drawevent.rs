pub struct DrawEvent {
    pub position: (u8, u8),
    pub rgb: (u8, u8, u8),
}

impl Clone for DrawEvent {

    fn clone(&self) -> DrawEvent {
        return DrawEvent {
            position: self.position,
            rgb: self.rgb
        }
    }
}