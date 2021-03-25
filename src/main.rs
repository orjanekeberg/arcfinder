use std::collections;
use std::io::{self, BufRead};
use regex::Regex;

const PI: f64 = std::f64::consts::PI;

const EMIT_RADIUS: bool = true;
const MIN_MATCH: usize = 4;
const RMS_LIMIT: f64 = 0.01;
const ANGLE_LIMIT: f64 = 40.0 * PI/180.0;
const OFFSET_LIMIT: f64 = 0.5;

struct Move {
    x: f64,
    y: f64,
    e: f64,
}

struct State {
    current_x: f64,
    current_y: f64,
    storage: collections::VecDeque<Move>,
}


impl State {
    fn store_move(&mut self, x: f64, y:f64, e:f64) {
        self.storage.push_back(Move{x: x, y: y, e: e});
    }

    fn process_moves(&mut self) {
        if self.storage.is_empty() {
            // No stored moves to process
            return
        }

        // Copy the coordinates into a sliceable vector
        let mut points: Vec<Point> = Vec::new();
        for p in self.storage.iter() {
            points.push(Point{x: p.x, y: p.y});
        }

        let mut first: usize = 0;

        while first < points.len() { // Consume all stored points
            let mut found_candidate = false;
            let mut candidate = (false, 0.0, Point{x:0.0, y:0.0});
            let mut candidate_index = 0;
            let mut last = first + (MIN_MATCH-1) as usize;

            while last < points.len() {
                match find_best_arc(&Point{x: self.current_x, y: self.current_y},
                                    &points[last], &points[first..last]) {
                    Some(best) => {
                        candidate = best;
                        candidate_index = last;
                        found_candidate = true;
                        last += 1; // Try to include more points
                    },
                    None => {
                        break;
                    }
                }
            }
            
            if found_candidate {
                // Calculate the total extrusion
                let mut e_sum = 0.0;
                for i in first..candidate_index+1 {
                    e_sum += self.storage[i].e;
                }

                if EMIT_RADIUS {
                    println!("{} X{:5.3} Y{:5.3} R{:5.3} E{:.5}",
                             if candidate.0 {"G2"} else {"G3"},
                             points[candidate_index].x,
                             points[candidate_index].y,
                             candidate.1,
                             e_sum);
                } else {
                    println!("{} X{:5.3} Y{:5.3} I{:5.3} J{:5.3} E{:.5}",
                             if candidate.0 {"G2"} else {"G3"},
                             points[candidate_index].x,
                             points[candidate_index].y,
                             candidate.2.x - self.current_x,
                             candidate.2.y - self.current_y,
                             e_sum);
                }
                
                self.current_x = points[candidate_index].x;
                self.current_y = points[candidate_index].y;
                first = candidate_index+1;
            } else {
                self.current_x = points[first].x;
                self.current_y = points[first].y;
                println!("G1 X{:5.3} Y{:5.3} E{:.5}",
                         self.current_x, self.current_y, self.storage[first].e);
                first += 1;
            }
        }
        self.storage.clear();
    }
}


struct Point {
    x: f64,
    y: f64,
}


// Compute centre of circle passing three points
fn centre(p1: &Point, p2: &Point, p3: &Point) -> Point {
    let x12 = p1.x - p2.x;
    let x13 = p1.x - p3.x;

    let y12 = p1.y - p2.y;
    let y13 = p1.y - p3.y;

    let y31 = p3.y - p1.y;
    let y21 = p2.y - p1.y;

    let x31 = p3.x - p1.x;
    let x21 = p2.x - p1.x;

    // p1.x^2 - p3.x^2
    let sx13 = p1.x.powi(2) - p3.x.powi(2);

    // p1.y^2 - p3.y^2
    let sy13 = p1.y.powi(2) - p3.y.powi(2);

    let sx21 = p2.x.powi(2) - p1.x.powi(2);
    let sy21 = p2.y.powi(2) - p1.y.powi(2);

    let xden = 2.0 * (x31*y12 - x21*y13);
    let yden = 2.0 * (y31*x12 - y21*x13);

    let g = 
        if xden==0.0 {
            -p3.x
        } else {
            (sx13*y12 + sy13*y12 + sx21*y13 + sy21*y13) / xden
        };

    let f =
        if yden==0.0 {
            -p3.y
        } else {
            (sx13*x12 + sy13*x12 + sx21*x13 + sy21*x13) / yden
        };

    Point{x: -g, y: -f}
}


fn best_arc(a: &Point, b: &Point, points: &[Point]) -> (f64, Point, f64) {
    let mid = Point{x: (a.x+b.x)/2.0, y: (a.y+b.y)/2.0};

    let k = ((b.y-a.y).powi(2) + (a.x-b.x).powi(2)).sqrt();

    // Guard
    if k == 0.0 {
        return (0.0, Point{x:0.0, y:0.0}, 1000.0);
    }

    // Mismatch function
    let g = |d: f64| -> f64 {
        let mut sum = 0.0;
        for p in points.iter() {
            sum += (((mid.x + d/k*(b.y-a.y) - p.x).powi(2) +
                     (mid.y + d/k*(a.x-b.x) - p.y).powi(2)).sqrt() -
                    (d.powi(2) + k.powi(2)/4.0).sqrt()).powi(2);
        }
        sum
    };

    // Derivative of mismatch function
    let dg = |d: f64| -> f64 {
        let mut sum = 0.0;
        for p in points.iter() {
            sum += 2.0*(((mid.x + d/k*(b.y-a.y) - p.x).powi(2) +
                         (mid.y + d/k*(a.x-b.x) - p.y).powi(2)).sqrt() -
                        (d.powi(2) + k.powi(2)/4.0).sqrt()) *
                (1.0/((mid.x + d/k*(b.y-a.y) - p.x).powi(2) +
                      (mid.y + d/k*(a.x-b.x) - p.y).powi(2)).sqrt() *
                 ((mid.x + d/k*(b.y-a.y) - p.x) * (b.y-a.y)/k +
                  (mid.y + d/k*(a.x-b.x) - p.y) * (a.x-b.x)/k) -
                 d/(d.powi(2)+k.powi(2)/4.0).sqrt());
        }
        sum
    };

    // Estimate d from three points
    let f = &points[(points.len()-1)/2];
    let c_est = centre(&a, &b, f);

    let d_est = (c_est.x-mid.x)*(b.y-a.y)/k + (c_est.y-mid.y)*(a.x-b.x)/k;

    // Simpsons method
    let mut d_last = d_est - 0.1*k;
    let mut d_opt = d_est;
    let mut dg_last = dg(d_last);
    let mut dg_opt = dg(d_opt);

    for _iter in 0..10 {
        if dg_last == dg_opt {
            break;
        }
        {
            let tmp = d_opt;
            d_opt = d_opt+dg_opt*(d_opt-d_last)/(dg_last-dg_opt);
            d_last = tmp;
        }
        dg_last = dg_opt;
        dg_opt = dg(d_opt);
    }

    let rms = (g(d_opt) / (points.len() as f64)).sqrt();
    let c = Point{x: mid.x + d_opt/k*(b.y-a.y),
                  y: mid.y + d_opt/k*(a.x-b.x)};
    let r_opt = (d_opt.powi(2) + k.powi(2)/4.0).sqrt();

    return (r_opt, c, rms)
}


// Convert angle to -PI..PI range
fn unwind_angle(angle: f64) -> f64 {
    let mut a = angle;
    while a >= PI {
        a -= 2.0*PI;
    }
    while a < -PI {
        a += 2.0*PI;
    }
    return a
}


fn get_angles(a: &Point, b: &Point, c: &Point, points: &[Point], angles: &mut Vec<f64>) {
    let mut last_a = (a.y-c.y).atan2(a.x-c.x);

    // Compute angle difference and push to result vector
    let mut angle_diff = |p: &Point| {
        let a = (p.y-c.y).atan2(p.x-c.x);
        angles.push(unwind_angle(a - last_a));
        last_a = a;
    };

    for p in points.iter() {
        angle_diff(&p);
    }
    angle_diff(&b);
}


fn find_best_arc(a: &Point, b: &Point, points: &[Point]) -> Option<(bool, f64, Point)> {
    let (r, c, rms) = best_arc(a, b, points);

    if rms > RMS_LIMIT {
        return None
    }

    let mut angles: Vec<f64> = Vec::new();
    get_angles(a, b, &c, &points, &mut angles);

    let sign = angles[0].signum(); // Clockwise->-1, Counter-cw->1

    let mut max_angle = 0.0;
    for angle in angles {
        if sign*angle > max_angle {
            max_angle = sign*angle;
        }
        if sign*angle < 0.0 {
            // Mix of directions
            return None;
        }
    }
    if max_angle > ANGLE_LIMIT {
        return None;
    }
    if max_angle.powi(2)*r > OFFSET_LIMIT {
        return None;
    }

    return Some((sign<0.0,
                 if sign*((b.x-a.x)*(a.y-c.y)+(b.y-a.y)*(c.x-a.x)) > 0.0 {-r} else {r},
                 c))
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = State {current_x:0.0, current_y:0.0, storage:collections::VecDeque::<Move>::new()};
    let g1_pattern = Regex::new(r"^G1 X(\d+\.\d+) Y(\d+\.\d+) E(\d+\.\d+)")?;
    let g123_x_pattern = Regex::new(r"^G[123] .*X(\d+\.\d+)")?;
    let g123_y_pattern = Regex::new(r"^G[123] .*Y(\d+\.\d+)")?;
    
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = line?;
        match g1_pattern.captures(&line) {
            Some(cap) => state.store_move(cap[1].parse::<f64>()?,
                                          cap[2].parse::<f64>()?,
                                          cap[3].parse::<f64>()?),
            None => {
                state.process_moves();
                println!("{}", &line);
            },
        };

        match g123_x_pattern.captures(&line) {
            Some(cap) => state.current_x = cap[1].parse::<f64>()?,
            None => (),
        };

        match g123_y_pattern.captures(&line) {
            Some(cap) => state.current_y = cap[1].parse::<f64>()?,
            None => (),
        }
    }

    // Empty the queue if something is still there
    state.process_moves();
    
    Ok(())
}
