#![forbid(unsafe_code)]

use super::*;

/// Rectangle struct, origin at the upper left.
///
/// Naturally, having the origin at the upper left is a terrible and heretical
/// coordinate system to use, but that's what SDL2 does so that's what we're
/// stuck with.
///
/// **Note:** _This type has been rewritten in Rust._
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[allow(missing_docs)]
#[repr(C)]
pub struct Rect {
  pub x: i32,
  pub y: i32,
  pub w: i32,
  pub h: i32,
}
impl From<fermium::SDL_Rect> for Rect {
  fn from(other: fermium::SDL_Rect) -> Self {
    Self {
      x: other.x,
      y: other.y,
      w: other.w,
      h: other.h,
    }
  }
}
impl From<Rect> for fermium::SDL_Rect {
  fn from(other: Rect) -> Self {
    Self {
      x: other.x,
      y: other.y,
      w: other.w,
      h: other.h,
    }
  }
}
impl Rect {
  /// Returns true if the point resides in this rect.
  ///
  /// A rect is _inclusive_ with the upper and right sides, and _exclusive_ with
  /// the lower and left sides.
  pub fn contains_point(&self, px: i32, py: i32) -> bool {
    px >= self.x && px < (self.x + self.w) && py >= self.y && py < (self.y + self.h)
  }

  /// Returns true if the rectangle has no area.
  pub fn is_empty(&self) -> bool {
    self.w > 0 && self.h > 0
  }

  /// Gives a `Rect` that's the intersection between this and the other rect.
  ///
  /// * If either Rect in the intersection is empty you get a default Rect.
  pub fn intersect(&self, other: &Rect) -> Rect {
    let mut out = Rect::default();
    if self.is_empty() || other.is_empty() {
      return out;
    }

    // horizontal intersection
    let mut ha_min = self.x;
    let mut ha_max = ha_min + self.w;
    let hb_min = other.x;
    let hb_max = hb_min + other.w;
    if hb_min > ha_min {
      ha_min = hb_min;
    }
    out.x = ha_min;
    if hb_max < ha_max {
      ha_max = hb_max;
    }
    out.w = ha_max - ha_min;

    // vertical intersection
    let mut va_min = self.y;
    let mut va_max = va_min + self.h;
    let vb_min = other.y;
    let vb_max = vb_min + other.h;
    if vb_min > va_min {
      va_min = vb_min;
    }
    out.y = va_min;
    if vb_max < va_max {
      va_max = vb_max;
    }
    out.h = va_max - va_min;

    out
  }

  /// Gives a `Rect` that's a union between this and the other rect.
  ///
  /// * If self is empty, you just get the other rect.
  /// * If the other rect is empty, you just get self.
  /// * If _both_ are empty then you get a default rect.
  pub fn union(&self, other: &Rect) -> Rect {
    match (self.is_empty(), other.is_empty()) {
      (true, true) => Rect::default(),
      (true, false) => *other,
      (false, true) => *self,
      (false, false) => {
        let mut out = Rect::default();

        // horizontal intersection
        let mut ha_min = self.x;
        let mut ha_max = ha_min + self.w;
        let hb_min = other.x;
        let hb_max = hb_min + other.w;
        if hb_min < ha_min {
          ha_min = hb_min;
        }
        out.x = ha_min;
        if hb_max > ha_max {
          ha_max = hb_max;
        }
        out.w = ha_max - ha_min;

        // vertical intersection
        let mut va_min = self.y;
        let mut va_max = va_min + self.h;
        let vb_min = other.y;
        let vb_max = vb_min + other.h;
        if vb_min < va_min {
          va_min = vb_min;
        }
        out.y = va_min;
        if vb_max > va_max {
          va_max = vb_max;
        }
        out.h = va_max - va_min;

        out
      }
    }
  }

  // TODO: SDL_EnclosePoints

  // TODO: SDL_IntersectRectAndLine

  // TODO: SDL_GetSpanEnclosingRect
}
