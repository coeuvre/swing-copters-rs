
use graphics::vecmath::Scalar;

pub enum Action {
    /// dt, x, y
    MoveTo(f64, Scalar, Scalar),
    /// dt, x, y
    MoveBy(f64, Scalar, Scalar),
    /// dt, deg
    RotateTo(f64, Scalar),
    /// dt, deg
    RotateBy(f64, Scalar),
    /// dt, sx, sy
    ScaleTo(f64, Scalar, Scalar),
    /// dt, sx, sy
    ScaleBy(f64, Scalar, Scalar),
    /// Flip sprite in x direction
    FlipX(bool),
    /// Flip sprite in y direction
    FlipY(bool),
    /// Set the sprite's visibility to true
    Show,
    /// Set the sprite's visibility to false
    Hide,
    /// Toggle the sprite's visibility
    ToggleVisibility,
    /// dt, times
    Blink(f64, uint),
    /// dt
    ///
    /// Fade in the sprite, set its opacity from 0 to 1 in `dt` seconds
    FadeIn(f64),
    /// dt
    ///
    /// Fade out the sprite, set its opacity from 1 to 0 in `dt` seconds
    FadeOut(f64),
    /// dt, opacity
    ///
    /// Set the sprite's opacity to specified value in `dt` seconds
    FadeTo(f64, f64),
}
