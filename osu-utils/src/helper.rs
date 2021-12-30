use osu_types::OsuPoint;

pub trait MintPointExt {
    type T;
    fn new(x: Self::T, y: Self::T) -> Self;
    fn sub(self, b: Self) -> Self;
    fn add(self, b: Self) -> Self;
    fn mul(self, b: Self) -> Self;
    fn div(self, b: Self) -> Self;
    fn length_squared(self) -> Self::T;
    fn length(self) -> Self::T;
    fn distance(self, b: Self) -> Self::T;
}

impl MintPointExt for mint::Point2<f32> {
    type T = f32;

    fn new(x: f32, y: f32) -> mint::Point2<f32> {
        mint::Point2 { x, y }
    }

    fn sub(self, b: mint::Point2<f32>) -> mint::Point2<f32> {
        mint::Point2 {
            x: self.x - b.x,
            y: self.y - b.y,
        }
    }

    fn add(self, b: mint::Point2<f32>) -> mint::Point2<f32> {
        mint::Point2 {
            x: self.x + b.x,
            y: self.y + b.y,
        }
    }

    fn mul(self, b: mint::Point2<f32>) -> mint::Point2<f32> {
        mint::Point2 {
            x: self.x * b.x,
            y: self.y * b.y,
        }
    }

    fn div(self, b: mint::Point2<f32>) -> mint::Point2<f32> {
        mint::Point2 {
            x: self.x / b.x,
            y: self.y / b.y,
        }
    }

    fn length_squared(self) -> f32 {
        self.x.powi(2) + self.y.powi(2)
    }

    fn length(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn distance(self, b: mint::Point2<f32>) -> f32 {
        ((self.x - b.x).abs().powi(2) + (self.y - b.y).abs().powi(2)).sqrt()
    }
}

pub trait MintPointFloatCast {
    fn to_float(self) -> mint::Point2<f32>;
}

impl MintPointFloatCast for OsuPoint {
    fn to_float(self) -> mint::Point2<f32> {
        mint::Point2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

/// Computes the circumcircle given 3 points.
pub fn circumcircle(
    p1: mint::Point2<f32>,
    p2: mint::Point2<f32>,
    p3: mint::Point2<f32>,
) -> (mint::Point2<f32>, f32) {
    let (x1, y1) = (p1.x, p1.y);
    let (x2, y2) = (p2.x, p2.y);
    let (x3, y3) = (p3.x, p3.y);

    let two = 2.0f32;
    let d = two.mul_add(x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2), 0.0);
    let ux = ((x1 * x1 + y1 * y1) * (y2 - y3)
        + (x2 * x2 + y2 * y2) * (y3 - y1)
        + (x3 * x3 + y3 * y3) * (y1 - y2))
        / d;
    let uy = ((x1 * x1 + y1 * y1) * (x3 - x2)
        + (x2 * x2 + y2 * y2) * (x1 - x3)
        + (x3 * x3 + y3 * y3) * (x2 - x1))
        / d;

    let center = mint::Point2::new(ux, uy);
    (center, center.distance(p1))
}

/// Get the point on the line segment on p1, p2 that ends after length
#[allow(clippy::many_single_char_names)]
pub fn point_on_line(a: mint::Point2<f32>, b: mint::Point2<f32>, len: f32) -> mint::Point2<f32> {
    let full = a.distance(b);
    let n = full - len;
    let x = (n * a.x + len * b.x) / full;
    let y = (n * a.y + len * b.y) / full;
    mint::Point2::new(x, y)
}

/// Checks if a, b, and c are all on the same line
pub fn is_line(a: mint::Point2<f32>, b: mint::Point2<f32>, c: mint::Point2<f32>) -> bool {
    ((b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)).abs() < 0.001
}
