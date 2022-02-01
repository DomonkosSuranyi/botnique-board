use bevy::prelude::{App, EventWriter, Res, ResMut};
use bevy::app::prelude::CoreStage;
use bevy::app::AppExit;
use bevy::ecs::system::Resource;
use bevy::ecs::schedule::{SystemDescriptor, IntoSystemDescriptor};

pub trait TestApp {
    fn send_events<T>(&mut self, events_to_send: Vec<Option<T>>) -> &mut Self
        where T: Resource;

    fn add_assert_system<P>(&mut self, system: impl IntoSystemDescriptor<P>) -> &mut Self;

    fn exit_after_nth_frame(&mut self, n: u8) -> &mut Self;
}

struct EventsToSend<T>(Vec<Option<T>>) where T: Resource;
struct CurrentFrameNumber(u8);
struct TargetFrameNumber(u8);

fn send_event_system<T>(mut writer: EventWriter<T>,
                        mut event_res: ResMut<EventsToSend<T>>)
where T: Resource {
    let maybe_event = event_res.0.pop().flatten();
    if let Some(event) = maybe_event {
        writer.send(event);
    }
}

impl TestApp for App {
    fn send_events<T>(&mut self, events_to_send: Vec<Option<T>>) -> &mut Self
    where T: Resource {
        self.add_event::<T>()
           .insert_resource(EventsToSend(events_to_send))
           .add_system_to_stage(
               CoreStage::PreUpdate, send_event_system::<T>)
    }

    fn add_assert_system<P>(&mut self, system: impl IntoSystemDescriptor<P>) -> &mut Self {
        self.add_system_to_stage(CoreStage::PostUpdate, system)
    }

    fn exit_after_nth_frame(&mut self, n: u8) -> &mut Self {
        self.insert_resource(CurrentFrameNumber(0u8))
            .insert_resource(TargetFrameNumber(n))
            .add_system_to_stage(CoreStage::Last,
             |mut exit: EventWriter<AppExit>,
              mut current_frame: ResMut<CurrentFrameNumber>,
              target_frame: Res<TargetFrameNumber>| {
                 if current_frame.0 >= target_frame.0 {
                     exit.send(AppExit);
                 }
                 current_frame.0 += 1u8;
             })
    }

}

pub mod assertion {
    use super::*;
    use bevy::prelude::EventReader;

    pub fn event_count<T>(cnt: u8) -> SystemDescriptor
    where T: Resource {
        let system = move |mut reader: EventReader<T>| {
            let mut event_cnt = 0u8;
            for _ in reader.iter() {
                event_cnt += 1;
            }
            assert_eq!(
                event_cnt,
                cnt,
                "\nEvent count assertion failed.
                   Event: {}
                   Expected: {}, Actual: {}\n",
                std::any::type_name::<T>(), cnt, event_cnt);
        };
        system.into_descriptor()
    }

    pub fn assert_event<T>(expected: T) -> SystemDescriptor
    where T: Resource
           + std::fmt::Debug
           + std::cmp::PartialEq {
        let system = move |mut reader: EventReader<T>| {
            let maybe_event = reader.iter().next();
            assert_eq!(maybe_event.is_some(), true, "Received no event {}", std::any::type_name::<T>());
            assert_eq!(*maybe_event.unwrap(), expected);
        };
        system.into_descriptor()
    }
}

#[macro_export]
macro_rules! assert_delta {
    ($x:expr, $y:expr, $delta:expr) => {
        assert!(($x -$y).abs() < $delta);
    }
}
