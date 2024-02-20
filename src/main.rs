#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: f64,
    y: f64,
    name: String,
}

impl Point {
    fn calc_euclidean_distance(&self, other_point: &Point) -> f64 {
        return ((other_point.x - self.x).powf(2.0) + (other_point.y - self.y).powf(2.0)).sqrt();
    }

    fn calc_distances_for_point(&self, points: &Vec<Point>) {
        for p in points.iter() {
            println!(
                "{}:({}, {}) -> {}:({}, {}) = {}",
                self.name,
                self.x,
                self.y,
                p.name,
                p.x,
                p.y,
                self.calc_euclidean_distance(p)
            );
        }
    }

    fn calc_distances(points: &Vec<Point>) {
        for p in points.iter() {
            for other in points.iter() {
                if !(p == other) {
                    println!(
                        "{}:({}, {}) -> {}:({}, {}) = {}",
                        p.name,
                        p.x,
                        p.y,
                        other.name,
                        other.x,
                        other.y,
                        p.calc_euclidean_distance(other)
                    );
                }
            }
        }
    }
}

fn main() {
    let p1 = Point {
        x: 2.0,
        y: 5.0,
        name: String::from("p1"),
    };

    let p2 = Point {
        x: 10.0,
        y: 7.0,
        name: String::from("p2"),
    };

    let p3 = Point {
        x: -3.0,
        y: 13.0,
        name: String::from("p3"),
    };

    println!("{:?}", p1.calc_euclidean_distance(&p2));

    let points = vec![p1, p2, p3];

    Point::calc_distances(&points);
}
