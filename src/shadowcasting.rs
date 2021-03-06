// Copyright 2014-2017 bluss
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Smooth out the vision circle by fuzzing the radius a bit
const RADIUS_FUDGE: f32 = 0.33;

/// Squares marked not visible act as obstacles.
const NONVIS_OCCLUDE: bool = true;

/// You may choose to have include or exclude the end points here,
/// `<` is more permissive than `<=`.
fn angle_contained_in(angle: f32, start: f32, end: f32) -> bool {
    start < angle && angle < end
}

/// Decide visibility of the square based on center, near, and far corner visibility
fn visible_when(center: bool, near: bool, far: bool) -> bool {
    center && (near || far)
}

/// Restrictive Precise Angle Shadowcasting.
///
/// A port of https://github.com/MoyTW/roguebasin_rpas to Rust.
///
/// **RPAShadowcasting\<F\>** is an iterator that iterates a circular
/// region. It always uses coordinates centered on **(0, 0)** and yields
/// tuples **(i32, i32, bool)** representing the offset relative to the origin,
/// and a boolean for the square's visibility.
///
/// All squares inside the radius are yielded. The origin is always yielded first
/// with visibility **true**.
pub struct RPAShadowcasting<F> {
    obstruct: F,
    iter: CircleIter<()>,
}

impl<F> RPAShadowcasting<F>
where
    F: FnMut(i32, i32) -> bool,
{
    /// Create a new **RPAShadowcasting**.
    ///
    /// The function **obstruct(x, y)** should return **true**
    /// if the relative coordinates **x, y** are obstructing vision.
    pub fn new(radius: i32, obstruct: F) -> Self {
        RPAShadowcasting {
            obstruct: obstruct,
            iter: CircleIter::new(radius),
        }
    }
}

impl<F> Iterator for RPAShadowcasting<F>
where
    F: FnMut(i32, i32) -> bool,
{
    /// The iterator element represents **(x, y, visible)** in coordinates
    /// relative to the center.
    type Item = (i32, i32, bool);

    /// Algorithm in very brief summary:
    ///
    /// Treat each octant wedge completely separately.
    /// List occlusions as (start, end) angle for the current octant wedge.
    ///
    /// Visit squares by radial coordinate r in 1...radius and transversal
    /// coordinate x in 1...r.
    ///
    /// ```ignore
    ///  |  .
    ///  |  12
    ///  v  345     1,2,.. is visit order
    ///  r  6789
    ///     -->x
    /// ```
    ///
    /// Compute angles to the square's near, center
    /// and far edge and compare with all previous recorded occlusions.
    fn next(&mut self) -> Option<(i32, i32, bool)> {
        let (a, b, (near, center, far)) = match self.iter.next() {
            None => return None,
            Some(x) => x,
        };

        // check visibility vs prev obstructions.
        let mut visible = true;
        let mut near_vis = true;
        let mut center_vis = true;
        let mut far_vis = true;
        for &(near_obs, far_obs, _) in self.iter.obstructions.iter() {
            near_vis = near_vis && !angle_contained_in(near, near_obs, far_obs);
            center_vis = center_vis && !angle_contained_in(center, near_obs, far_obs);
            far_vis = far_vis && !angle_contained_in(far, near_obs, far_obs);

            visible = visible_when(center_vis, near_vis, far_vis);
            if !visible {
                break;
            }
        }
        if (NONVIS_OCCLUDE && !visible) || (self.obstruct)(a, b) {
            self.iter.obstructions.push((near, far, ()));
        }
        Some((a, b, visible))
    }
}

/// **RPAPartialShadowcasting\<F\>** is an iterator that iterates a circular
/// region. It always uses coordinates centered on **(0, 0)** and yields
/// tuples **(i32, i32, f32)** representing the offset relative to the origin,
/// and a float for the opacity of that square.
///
/// All squares inside the radius are yielded. The origin is always yielded first
/// with visibility **true**.
///
/// Works very similarly to **RPAShadowcasting**, but it allows visibility
/// to be limited partially and by adding up obstacles in the line of sight.
pub struct RPAPartialShadowcasting<F> {
    obstruct: F,
    /// begin angle, end angle, opacity in 0. to 1.
    iter: CircleIter<f32>,
}

impl<F> RPAPartialShadowcasting<F>
where
    F: FnMut(i32, i32) -> f32,
{
    /// Create a new **RPAPartialShadowcasting**.
    ///
    /// The function **obstruct(x, y)** should return a float indicating
    /// the opacity in 0. to 1. of the square at coordinates **x, y**.
    #[allow(dead_code)]
    pub fn new(radius: i32, obstruct: F) -> Self {
        RPAPartialShadowcasting {
            obstruct: obstruct,
            iter: CircleIter::new(radius),
        }
    }
}

impl<F> Iterator for RPAPartialShadowcasting<F>
where
    F: FnMut(i32, i32) -> f32,
{
    /// The iterator element represents **(x, y, opacity)** in coordinates
    /// relative to the center, opacity from 0. to 1. (1. means no visibility).
    type Item = (i32, i32, f32);

    fn next(&mut self) -> Option<(i32, i32, f32)> {
        let (a, b, (near, center, far)) = match self.iter.next() {
            None => return None,
            Some(x) => x,
        };

        // check visibility vs prev obstructions.
        // preserve the same visible_when rule as in the discrete case
        let mut opacity: f32 = 0.;
        let mut near_vis = true;
        let mut center_vis = true;
        let mut far_vis = true;
        for &(near_obs, far_obs, opc_obs) in self.iter.obstructions.iter() {
            near_vis = near_vis && !angle_contained_in(near, near_obs, far_obs);
            center_vis = center_vis && !angle_contained_in(center, near_obs, far_obs);
            far_vis = far_vis && !angle_contained_in(far, near_obs, far_obs);
            if !center_vis {
                opacity = opacity.max(0.5 * opc_obs);
            }
            if !visible_when(center_vis, near_vis, far_vis) {
                opacity = opacity.max(opc_obs);
            }

            if opacity >= 1. {
                break;
            }
        }
        let mut opc_here = (self.obstruct)(a, b);
        opc_here = opacity + opc_here;
        if opc_here > 0. {
            self.iter.obstructions.push((near, far, opc_here));
        }
        opacity = opacity.min(1.);
        Some((a, b, opacity))
    }
}

/// Visit all squares in a circle around (0, 0) up until the specified radius.
/// The iterator computes the near, center, far angles for each square.
///
/// Note: May visit a few coordinates twice.
#[derive(Clone, Debug)]
struct CircleIter<T> {
    /// This is cleared every time we switch octant.
    pub obstructions: Vec<(f32, f32, T)>,
    /// identify x-y quadrant and top/bottom half of quadrant.
    /// cycle (1, 1) -> (1, -1) -> (-1, -1) -> (-1, 1)
    octant: (i32, i32, bool),
    /// size of the circle
    radius: i32,
    /// radial coordinate
    r: i32,
    /// transversal coordinate
    x: i32,
}

/// x, y, (near, center, far).
type CircleItem = (i32, i32, (f32, f32, f32));

impl<T> CircleIter<T> {
    /// Create a new **CircleIter**.
    pub fn new(radius: i32) -> Self {
        CircleIter {
            obstructions: Vec::new(),
            octant: (1, 1, true),
            radius: radius,
            r: 0,
            x: 0,
        }
    }

    fn next_octant(&mut self) {
        let (ref mut x, ref mut y, ref mut vert) = self.octant;
        *vert = !*vert;
        if !*vert {
            return;
        }
        if *x > 0 && *y > 0 {
            *y = -1;
        } else if *x > 0 && *y < 0 {
            *x = -1;
        } else if *x < 0 && *y < 0 {
            *y = 1;
        } else {
            *x = 1;
        }
    }
}

impl<T> Iterator for CircleIter<T> {
    type Item = CircleItem;

    fn next(&mut self) -> Option<CircleItem> {
        if self.r == 0 {
            self.r += 1;
            return Some((0, 0, (0., 0., 0.)));
        }
        if self.x > self.r {
            self.x = 0;
            self.r += 1;
        }

        // Skip to next octant when we reach the radial limit.
        if self.r > self.radius {
            self.next_octant();
            self.x = 0;
            self.r = 1;
            self.obstructions.clear();
            if self.octant == (1, 1, true) {
                // back at the original octant and done.
                return None;
            } else {
                return self.next();
            }
        }

        let (qx, qy, vert) = self.octant;

        let (a, b) = if vert {
            (self.x * qx, self.r * qy)
        } else {
            (self.r * qx, self.x * qy)
        };

        if (a as f32).hypot(b as f32) >= RADIUS_FUDGE + self.radius as f32 {
            self.x += 1;
            return self.next();
        }
        // compute square's angles.
        let angle_alloc = 1. / ((self.r + 1) as f32);
        let near = (self.x as f32) * angle_alloc;
        let center = near + 0.5 * angle_alloc;
        let far = near + angle_alloc;

        self.x += 1;
        Some((a, b, (near, center, far)))
    }
}
