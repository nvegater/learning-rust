// -- Structs --
// TS equivalent: class Point { constructor(public x: f64, public y: f64) {} }
// Rust: no constructor keyword, no `this`, no class inheritance
struct Point {
    x: f64,
    y: f64,
}

// Methods go in `impl` blocks (separated from data — unlike TS classes)
impl Point {
    // "Associated function" (like a static method) — no `self`
    // TS: static new(x: number, y: number): Point
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    // Method — takes `&self` (immutable borrow)
    // TS: distanceTo(other: Point): number
    fn distance_to(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

// -- Enums --
// TS equivalent:
//   type Shape =
//     | { kind: "circle"; radius: number }
//     | { kind: "rectangle"; width: number; height: number }
//     | { kind: "triangle"; base: number; height: number }
//
// Rust enums are MORE powerful: each variant holds typed data directly.
// No "kind" field needed — the compiler tracks which variant it is.
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
    Pentagon { side: f64 },
}

// -- Traits --
// TS equivalent: interface Describable { describe(): string; }
// Traits define shared behavior. Any type can implement them.
trait Describable {
    fn describe(&self) -> String;
}

// -- Implement methods on the enum --
impl Shape {
    fn area(&self) -> f64 {
        // `match` = TS switch on discriminated union, but EXHAUSTIVE
        // Compiler forces you to handle every variant (try commenting one out!)
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
            Shape::Pentagon { side } => {
                (side * side / 4.0) * (5.0 * (5.0 + 2.0 * 5.0_f64.sqrt())).sqrt()
            }
        }
    }
}

// -- Implement a trait for a type --
// TS: class Shape implements Describable { describe() { ... } }
// Rust: impl blocks are separate — you can implement traits for types you don't own
impl Describable for Shape {
    fn describe(&self) -> String {
        match self {
            Shape::Circle { radius } => format!("Circle with radius {radius}"),
            Shape::Rectangle { width, height } => {
                format!("Rectangle {width}x{height}")
            }
            Shape::Triangle { base, height } => {
                format!("Triangle base={base} height={height}")
            }
            Shape::Pentagon { side } => {
                format!("Pentagon side={side}")
            }
        }
    }
}

// You can also implement traits for structs
impl Describable for Point {
    fn describe(&self) -> String {
        format!("Point({}, {})", self.x, self.y)
    }
}

// -- Trait as a function parameter --
// TS: function printDescription(item: Describable): void
// Rust: `impl Trait` means "any type that implements this trait"
fn print_description(item: &impl Describable) -> String {
    format!("Description: {}", item.describe())
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Struct tests ---

    #[test]
    fn point_creation() {
        let p = Point::new(3.0, 4.0);
        assert_eq!(p.x, 3.0);
        assert_eq!(p.y, 4.0);
    }

    #[test]
    fn point_distance() {
        let origin = Point::new(0.0, 0.0);
        let p = Point::new(3.0, 4.0);
        // Classic 3-4-5 triangle
        // So origin would be self.
        // and p would be other.
        assert!((origin.distance_to(&p) - 5.0).abs() < f64::EPSILON);
    }

    // --- Enum tests ---

    #[test]
    fn circle_area() {
        let c = Shape::Circle { radius: 1.0 };
        assert!((c.area() - std::f64::consts::PI).abs() < f64::EPSILON);
    }

    #[test]
    fn rectangle_area() {
        let r = Shape::Rectangle {
            width: 3.0,
            height: 4.0,
        };
        assert!((r.area() - 12.0).abs() < f64::EPSILON);
    }

    #[test]
    fn triangle_area() {
        let t = Shape::Triangle {
            base: 6.0,
            height: 4.0,
        };
        assert!((t.area() - 12.0).abs() < f64::EPSILON);
    }

    // --- Trait tests ---

    #[test]
    fn shape_describe() {
        let c = Shape::Circle { radius: 2.5 };
        assert_eq!(c.describe(), "Circle with radius 2.5");
    }

    #[test]
    fn point_describe() {
        let p = Point::new(1.0, 2.0);
        assert_eq!(p.describe(), "Point(1, 2)");
    }

    #[test]
    fn trait_as_parameter() {
        // Both Shape and Point implement Describable — same function works for both
        let shape = Shape::Rectangle {
            width: 5.0,
            height: 3.0,
        };
        let point = Point::new(1.0, 2.0);

        assert_eq!(print_description(&shape), "Description: Rectangle 5x3");
        assert_eq!(print_description(&point), "Description: Point(1, 2)");
    }

    #[test]
    fn pentagon_area() {
        let p = Shape::Pentagon { side: 5.0 };
        // A = (s² / 4) * √(5(5 + 2√5))
        let expected = (25.0 / 4.0) * (5.0 * (5.0 + 2.0 * 5.0_f64.sqrt())).sqrt();
        assert!((p.area() - expected).abs() < f64::EPSILON);
    }

    // --- Pattern matching exhaustiveness ---
    // Try adding a new variant to Shape (e.g., Pentagon) without updating
    // area() or describe() — the compiler will refuse to compile.
    // This is like TS's `never` exhaustiveness check, but enforced at compile time.
}
