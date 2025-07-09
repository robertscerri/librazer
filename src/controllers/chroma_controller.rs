use crate::razer_device::RazerDevice;

pub struct ChromaController<'a> {
    device: &'a RazerDevice<'a>,
}
