#[macro_use]
extern crate nom;
extern crate util;

named!(num<&str, i32>, map_res!(recognize!(preceded!(opt!(char!('-')), nom::digit)), str::parse));
named!(vec3<&str, Vector3>, delimited!(
        char!('<'),
        tuple!(num, preceded!(char!(','), num), preceded!(char!(','), num)),
        char!('>')
        ));
named!(particle<&str, Particle>, ws!(do_parse!(
            tag!("p=") >>
            p: vec3    >>
            char!(',') >>
            tag!("v=") >>
            v: vec3    >>
            char!(',') >>
            tag!("a=") >>
            a: vec3    >>
            (Particle { pos: p, vel: v, acc: a })
            )));
named!(parse_input<&str, Vec<Particle>>, complete!(many1!(particle)));

type Scalar = i32;
type Vector3 = (Scalar, Scalar, Scalar);

trait Vector<S> {
    fn vec_add(self, Self) -> Self;
    fn vec_diff(self, Self) -> Self;
    fn scal_mul(self, S) -> Self;
    fn dot(self, Self) -> S;
    fn norm(self) -> S;
    fn is_zero(self) -> bool;
}

impl Vector<Scalar> for Vector3 {
    fn vec_add(self, (x2, y2, z2): Vector3) -> Vector3 {
        (self.0 + x2, self.1 + y2, self.2 + z2)
    }

    fn vec_diff(self, (x2, y2, z2): Vector3) -> Vector3 {
        (self.0 - x2, self.1 - y2, self.2 - z2)
    }

    fn scal_mul(self, s: Scalar) -> Vector3 {
        (self.0 * s, self.1 * s, self.2 * s)
    }

    fn dot(self, (x2, y2, z2): Vector3) -> Scalar {
        self.0 * x2 + self.1 * y2 + self.2 * z2
    }

    fn norm(self) -> Scalar {
        self.0.abs() + self.1.abs() + self.2.abs()
    }

    fn is_zero(self) -> bool {
        self == (0, 0, 0)
    }
}

#[derive(Debug, Clone, Copy)]
struct Particle {
    pos: Vector3,
    vel: Vector3,
    acc: Vector3,
}

use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
enum Intersect {
    Roots(usize, [i32; 2]),
    Zero,
}
use Intersect::*;

impl Intersect {
    fn dne() -> Intersect {
        Roots(0, [0; 2])
    }
    fn one(root: i32) -> Intersect {
        Roots(1, [root, 0])
    }
    fn intersect(self, other: Intersect) -> Intersect {
        match (self, other) {
            (Roots(count1, roots1), Roots(count2, roots2)) => {
                let mut roots = [0; 2];
                let mut count = 0;
                for &r1 in roots1[..count1].iter() {
                    if roots2[..count2].iter().any(|&r2| r2 == r1) {
                        roots[count] = r1;
                        count += 1;
                    }
                }
                Roots(count, roots)
            }
            (Zero, i) | (i, Zero) => i,
        }
    }
}

impl Particle {
    fn vel_component(self) -> Vector3 {
        self.vel.scal_mul(2).vec_add(self.acc)
    }

    fn cmp(self, other: Particle) -> Ordering {
        let ac = self.acc.norm().cmp(&other.acc.norm());
        if ac != Ordering::Equal {
            return ac;
        }
        let vc = self.vel_component()
            .norm()
            .cmp(&other.vel_component().norm());
        if vc != Ordering::Equal {
            return vc;
        }
        self.pos.norm().cmp(&other.pos.norm())
    }

    fn pos(&self, t: i32) -> Vector3 {
        self.acc
            .scal_mul((t * (t + 1)) / 2)
            .vec_add(self.vel.scal_mul(t))
            .vec_add(self.pos)
    }

    fn collision_component((a, v, p): Vector3) -> Intersect {
        if a == 0 {
            if v == 0 {
                return if p == 0 { Zero } else { Intersect::dne() };
            }
            let t = -p / v;
            return if t >= 0 && (t * v == -p) {
                Intersect::one(t)
            } else {
                Intersect::dne()
            };
        }
        let disc = v * v - 4 * a * p;
        if disc < 0 {
            return Intersect::dne();
        }

        // https://people.csail.mit.edu/bkph/articles/Quadratics.pdf
        let (r1, r2) = {
            let d = (f64::from(disc)).sqrt();
            let a = f64::from(a);
            let b = f64::from(v);
            let c = f64::from(p);
            if v >= 0 {
                let r1 = ((-b - d) / (2.0 * a)).round() as i32;
                let r2 = ((2.0 * c) / (-b - d)).round() as i32;
                (r1, r2)
            } else {
                let r1 = ((2.0 * c) / (-b + d)).round() as i32;
                let r2 = ((-b + d) / (2.0 * a)).round() as i32;
                (r1, r2)
            }
        };
        let mut roots = [0; 2];
        let mut count = 0;
        if r1 >= 0 && (a * r1 * r1 + v * r1 + p) == 0 {
            roots[count] = r1;
            count += 1;
        }
        if r2 >= 0 && (a * r2 * r2 + v * r2 + p) == 0 {
            roots[count] = r2;
            count += 1;
        }
        Roots(count, roots)
    }

    /*
     * pt = p0 + t v0 + t (t + 1) / 2 a0
     *    = 1/2 (2 p0 + 2 t v0 + t (t + 1) a0)
     *    = 1/2 (2 p0 + 2 t v0 + t^2 a0 + t a0)
     *    = 1/2 (a0 t^2 + (2 v0 + a0) t + 2 p0)
     */
    fn collision(self, other: Particle) -> Option<i32> {
        let a = self.acc.vec_diff(other.acc);
        let v = self.vel_component().vec_diff(other.vel_component());
        let p = self.pos.vec_diff(other.pos).scal_mul(2);

        let c = Particle::collision_component((a.0, v.0, p.0))
            .intersect(Particle::collision_component((a.1, v.1, p.1)))
            .intersect(Particle::collision_component((a.2, v.2, p.2)));

        match c {
            Roots(0, _) => None,
            Roots(1, roots) => Some(roots[0]),
            Roots(2, roots) => Some(i32::min(roots[0], roots[1])),
            Zero => Some(0),
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_collision() {
    let p1 = Particle {
        pos: (1796, -8375, -1230),
        vel: (-81, 83, 44),
        acc: (1, 15, 0),
    };
    let p2 = Particle {
        pos: (1012, -8683, 5350),
        vel: (-53, 123, 12),
        acc: (1, 13, -14),
    };
    assert_eq!(Some(28), p1.collision(p2));
}

fn slowest_particle(particles: &[Particle]) -> usize {
    if particles.len() < 1 {
        return 0;
    }
    particles
        .iter()
        .cloned()
        .enumerate()
        .min_by(|&(_, x), &(_, y)| x.cmp(y))
        .unwrap()
        .0
}

fn free_particles(particles: &[Particle]) -> usize {
    use std::collections::BTreeMap;

    let mut collision = BTreeMap::<i32, Vec<(usize, usize)>>::new();
    let mut live = vec![true; particles.len()];
    for (i, pi) in particles.iter().cloned().enumerate() {
        for (j, pj) in particles[(i + 1)..].iter().cloned().enumerate() {
            if let Some(t) = pi.collision(pj) {
                assert_eq!(
                    pi.pos(t),
                    pj.pos(t),
                    "{:?} and {:?} claimed to collide at time {}, but they do not",
                    pi,
                    pj,
                    t
                );
                collision
                    .entry(t)
                    .or_insert_with(Vec::new)
                    .push((i, j + i + 1));
            }
        }
    }
    let mut removed = Vec::<usize>::new();
    for v in collision.values() {
        for &(i, j) in v.iter() {
            if live[i] && live[j] {
                removed.push(i);
                removed.push(j);
            }
        }
        for &i in &removed {
            live[i] = false;
        }
        removed.clear();
    }
    live.iter().cloned().filter(|&b| b).count()
}

#[allow(dead_code)]
fn last_collision(particles: &[Particle]) -> usize {
    let mut time = 0;
    for (i, pi) in particles.iter().cloned().enumerate() {
        for &pj in particles[(i + 1)..].iter() {
            if let Some(t) = pi.collision(pj) {
                time = usize::max(time, t as usize);
            }
        }
    }
    time
}

#[allow(dead_code)]
fn simulate(particles: &mut Vec<Particle>, steps: usize) {
    use std::collections::HashMap;

    let mut collision = HashMap::<Vector3, bool>::new();
    for _ in 0..steps {
        let mut len = particles.len();
        let mut i = 0;
        collision.clear();
        while i < len {
            let pos = particles[i].pos;
            if collision.contains_key(&pos) {
                collision.insert(pos, true);
                particles.swap_remove(i);
                len -= 1;
            } else {
                collision.insert(pos, false);
                i += 1;
            }
        }
        let mut i = 0;
        while i < len {
            if collision[&particles[i].pos] {
                particles.swap_remove(i);
                len -= 1;
            } else {
                i += 1;
            }
        }
        for p in particles.iter_mut() {
            p.vel = p.vel.vec_add(p.acc);
            p.pos = p.pos.vec_add(p.vel);
        }
    }
}

fn main() {
    let run = |input: &str| {
        let particles = match parse_input(input).to_result() {
            Ok(list) => list,
            Err(_) => {
                eprintln!("invalid input");
                return;
            }
        };
        println!("slowest particle:        {}", slowest_particle(&particles));
        println!("non-colliding particles: {}", free_particles(&particles));
    };
    util::run_multiline("enter particle list:", run);
}

#[test]
fn test_part_one() {
    let input = "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>";
    let particles = parse_input(input).to_result().unwrap();
    assert_eq!(0, slowest_particle(&particles));
}

#[test]
fn test_part_two() {
    let input = "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>";
    let mut particles = parse_input(input).to_result().unwrap();
    assert_eq!(1, free_particles(&mut particles));
}
