use sealed::sealed;
use std::marker::PhantomData;
#[sealed]
pub trait DroneState {}

pub struct Drone<State>
where
    State: DroneState,
{
    x: f32,
    y: f32,
    state: PhantomData<State>,
}

pub struct Idle;
#[sealed]
impl DroneState for Idle {}

pub struct Hovering;
#[sealed]
impl DroneState for Hovering {}

pub struct Flying;
#[sealed]
impl DroneState for Flying {}

impl Drone<Idle> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: PhantomData,
            x: 0.0,
            y: 0.0,
        }
    }

    #[must_use]
    pub fn take_off(self) -> Drone<Hovering> {
        Drone::<Hovering>::from(self)
    }
}

impl Default for Drone<Idle> {
    fn default() -> Self {
        Self::new()
    }
}

impl Drone<Hovering> {
    #[must_use]
    fn land(self) -> Drone<Idle> {
        Drone::<Idle>::from(self)
    }

    #[must_use]
    fn move_to(self, x: f32, y: f32) -> Drone<Hovering> {
        let drone = Drone::<Flying>::from(self);
        drone.fly(x, y)
    }
}

impl Drone<Flying> {
    fn has_arrived(&self, x: f32, y: f32) -> bool {
        (self.x - x).abs() < f32::EPSILON && (self.y - y).abs() < f32::EPSILON
    }

    #[must_use]
    fn fly(mut self, x: f32, y: f32) -> Drone<Hovering> {
        while !self.has_arrived(x, y) {
            self.x = x;
            self.y = y;
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        Drone::<Hovering>::from(self)
    }
}

impl From<Drone<Idle>> for Drone<Hovering> {
    fn from(drone: Drone<Idle>) -> Self {
        Self {
            x: drone.x,
            y: drone.y,
            state: PhantomData,
        }
    }
}

impl From<Drone<Flying>> for Drone<Hovering> {
    fn from(drone: Drone<Flying>) -> Self {
        Self {
            x: drone.x,
            y: drone.y,
            state: PhantomData,
        }
    }
}

impl From<Drone<Hovering>> for Drone<Flying> {
    fn from(drone: Drone<Hovering>) -> Self {
        Self {
            x: drone.x,
            y: drone.y,
            state: PhantomData,
        }
    }
}

impl From<Drone<Hovering>> for Drone<Idle> {
    fn from(drone: Drone<Hovering>) -> Self {
        Self {
            x: drone.x,
            y: drone.y,
            state: PhantomData,
        }
    }
}

#[cfg(test)]
mod drone_test {
    use super::*;
    #[test]
    fn drone_spawns_idle() {
        let drone = Drone::<Idle>::new();
        assert!(drone.x.abs() < f32::EPSILON);
        assert!(drone.y.abs() < f32::EPSILON);
    }

    #[test]
    fn drone_takes_off_n_lands() {
        let drone = Drone::<Idle>::new();
        let drone = drone.take_off();
        assert!(drone.x.abs() < f32::EPSILON);
        assert!(drone.y.abs() < f32::EPSILON);
        drone.land();
    }

    #[test]
    fn drone_flies() {
        let drone = Drone::<Idle>::new().take_off().move_to(-5.0, -5.0).land();
        assert!((drone.x - -5.0).abs() < f32::EPSILON);
        assert!((drone.y - -5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn drone_does_not_fly_idle() {
        let drone = Drone::<Idle>::new();
        // drone.move_to(10.0, 10.0); // comptime error: "move_to" is not a member of type Idle
        assert!(drone.x.abs() < f32::EPSILON);
        assert!(drone.y.abs() < f32::EPSILON);
    }
}

fn main() {
    let _drone = Drone::<Idle>::new().take_off().move_to(-5.0, -5.0).land();
}
