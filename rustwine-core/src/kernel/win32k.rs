use anyhow::Result;
use parking_lot::Mutex;
use std::collections::HashMap;

pub struct Win32k {
    gdi_objects: Mutex<HashMap<u32, GdiObject>>,
}

#[derive(Debug)]
pub enum GdiObject {
    Bitmap,
    Brush,
    Font,
    Palette,
    Pen,
    Region,
}

impl Win32k {
    pub fn new() -> Self {
        Self {
            gdi_objects: Mutex::new(HashMap::new()),
        }
    }

    pub fn create_gdi_object(&self, handle: u32, object_type: GdiObject) {
        self.gdi_objects.lock().insert(handle, object_type);
    }

    pub fn delete_gdi_object(&self, handle: u32) -> Result<()> {
        if self.gdi_objects.lock().remove(&handle).is_some() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Invalid GDI object handle"))
        }
    }
} 