use core::any::Any;
use super::EventType;

pub trait Event {
    fn as_any(&self) -> &dyn Any;
    fn event_type(&self) -> EventType;
}