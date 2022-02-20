use bevy::prelude::*;
use bevy::utils::Duration;

#[derive(Component)]
pub struct Animation<T> {
    pub entity: Entity,
    pub start: T,
    pub end: T,
    pub timer: Timer
}

impl Animation<Style> {
    fn ease_pos(&self) -> Rect<Val> {
        let t = self.timer.percent();
        self.start.position.ease(&self.end.position, t)
    }
}

pub trait Ease {
    fn ease(&self, other: &Self, t: f32) -> Self;
}

impl Ease for Val {
    fn ease(&self, other: &Self, t:f32) -> Self {
        // println!("{:?} {:?}", self, other);
        use interpolation::*;
        if let Val::Px(a) = self {
            if let Val::Px(b) = other {
                return Val::Px(lerp(a, b, &t.quadratic_out()));
            }
        }
        if let Val::Percent(a) = self {
            if let Val::Percent(b) = other {
                let v = Val::Percent(lerp(a, b,&t.quadratic_out()));
                // println!("{:?}", v);
                return v;
            }
        }
        Val::Undefined
    }
}

impl Ease for Rect<Val> {
    fn ease(&self, other: &Self, t:f32) -> Self {
        Rect {
            left: self.left.ease(&other.left, t),
            right: self.right.ease(&other.right, t),
            top: self.top.ease(&other.top, t),
            bottom: self.bottom.ease(&other.bottom, t)
        }
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(style_animation_system);
    }
}

fn style_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut style_animation_query: Query<(Entity, &mut Animation<Style>)>,
    mut style_query: Query<&mut Style>
) {
    for (e, mut style_ani) in style_animation_query.iter_mut() {
        let mut style = style_query.get_mut(style_ani.entity).unwrap();
        style_ani.timer.tick(time.delta());
        style.position = style_ani.ease_pos();
        if style_ani.timer.finished() {
            commands.entity(e).despawn();
        }
    }
}
