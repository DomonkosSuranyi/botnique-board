use bevy::prelude::*;
use westiny_common::components::{Input, InputFlags, Velocity};
use westiny_common::metric_dimension::{MeterPerSec, MeterPerSecVec2};
use westiny_common::utilities::rotate_toward_point;
use westiny_common::MoveDirection;

pub fn apply_input(mut query: Query<(&GlobalTransform, &mut Transform, &mut Velocity, &Input)>) {
    for (global_transform, mut transform, mut velocity, input) in query.iter_mut() {
        rotate_toward_point(&mut transform, &input.cursor.into_pixel_vec());

        let move_inputs = move_directions_from_input(input);
        *velocity = get_velocity(&global_transform.to_scale_rotation_translation().1, &move_inputs);
    }
}

fn move_directions_from_input(input: &Input) -> Vec<MoveDirection> {
    let mut directions = Vec::new();
    if input.flags.intersects(InputFlags::FORWARD) {
        if input.flags.intersects(InputFlags::RUN) {
            directions.push(MoveDirection::RunForward);
        } else {
            directions.push(MoveDirection::Forward);
        }
    }
    if input.flags.intersects(InputFlags::BACKWARD) {
        directions.push(MoveDirection::Backward);
    }
    if input.flags.intersects(InputFlags::LEFT) {
        directions.push(MoveDirection::StrafeLeft);
    }
    if input.flags.intersects(InputFlags::RIGHT) {
        directions.push(MoveDirection::StrafeRight);
    }
    directions
}

const PLAYER_MAX_WALK_SPEED: MeterPerSec = MeterPerSec(4.0);

fn get_velocity<'a, I>(rotation: &Quat, move_inputs: I) -> Velocity
where
    I: IntoIterator<Item = &'a MoveDirection>,
{
    let (cnt, sum) = move_inputs
        .into_iter()
        .map(|dir| as_vector2(*dir))
        .enumerate()
        .fold(
            (1, MeterPerSecVec2::from_raw(0.0, 0.0)),
            |(_, acc), (num, dir)| (num + 1, acc + dir),
        );

    let avg_vec = sum / (cnt as f32);
    Velocity(avg_vec.rotate(rotation))
}

fn as_vector2(move_dir: MoveDirection) -> MeterPerSecVec2 {
    match move_dir {
        MoveDirection::Forward => MeterPerSecVec2 {
            x: MeterPerSec(0.0),
            y: -PLAYER_MAX_WALK_SPEED,
        },
        MoveDirection::Backward => MeterPerSecVec2 {
            x: MeterPerSec(0.0),
            y: PLAYER_MAX_WALK_SPEED / 2.0,
        },
        MoveDirection::StrafeLeft => MeterPerSecVec2 {
            x: PLAYER_MAX_WALK_SPEED / 2.0,
            y: MeterPerSec(0.0),
        },
        MoveDirection::StrafeRight => MeterPerSecVec2 {
            x: -PLAYER_MAX_WALK_SPEED / 2.0,
            y: MeterPerSec(0.0),
        },
        MoveDirection::RunForward => MeterPerSecVec2 {
            x: MeterPerSec(0.0),
            y: PLAYER_MAX_WALK_SPEED * - 2.0,
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bevy::prelude::Transform;
    use std::f32::consts::PI;
    use westiny_test::assert_delta;

    const FACING_UP: f32 = PI;
    const FACING_DOWN: f32 = 0.0;
    const FACING_LEFT: f32 = 3.0 * PI / 2.0;
    const FACING_RIGHT: f32 = PI / 2.0;

    mod test_get_velocity {
        use super::*;

        macro_rules! test_get_velocity {
            ($($name:ident: $player_rotation:expr, $move_dirs:expr, $expected:expr,)*) => {$(
                #[test]
                fn $name() {
                    let mut transform: Transform = Transform::default();
                    transform.rotation = Quat::from_rotation_z($player_rotation);

                    let inputs = $move_dirs;
                    let (exp_x, exp_y) = $expected;

                    let velocity = get_velocity(&transform.rotation, &inputs);

                    assert_delta!(exp_x.0, velocity.0.x.0, 0.00001);
                    assert_delta!(exp_y.0, velocity.0.y.0, 0.00001);
                }
            )*}
        }

        use MoveDirection::*;

        test_get_velocity! {
            // forward
            fwd_up: FACING_UP, vec!{Forward}, (MeterPerSec(0.0), PLAYER_MAX_WALK_SPEED),
            fwd_down: FACING_DOWN, vec!{Forward}, (MeterPerSec(0.0), -PLAYER_MAX_WALK_SPEED),
            fwd_left: FACING_LEFT, vec!{Forward}, (-PLAYER_MAX_WALK_SPEED, MeterPerSec(0.0)),
            fwd_right: FACING_RIGHT, vec!{Forward}, (PLAYER_MAX_WALK_SPEED, MeterPerSec(0.0)),
            run_fwd_up: FACING_UP, vec!{RunForward}, (MeterPerSec(0.0), PLAYER_MAX_WALK_SPEED * 2.0),
            run_fwd_down: FACING_DOWN, vec!{RunForward}, (MeterPerSec(0.0), -PLAYER_MAX_WALK_SPEED * 2.0),
            run_fwd_left: FACING_LEFT, vec!{RunForward}, (-PLAYER_MAX_WALK_SPEED * 2.0, MeterPerSec(0.0)),
            run_fwd_right: FACING_RIGHT, vec!{RunForward}, (PLAYER_MAX_WALK_SPEED * 2.0, MeterPerSec(0.0)),
        }
    }
}
