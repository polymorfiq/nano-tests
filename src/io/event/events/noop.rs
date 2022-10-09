use super::super::EventType;

pub struct NoopEvent {}

impl super::super::Event for NoopEvent {
    fn event_type(&self) -> EventType { EventType::None }
    fn as_any(&self) -> &dyn core::any::Any { self }
}