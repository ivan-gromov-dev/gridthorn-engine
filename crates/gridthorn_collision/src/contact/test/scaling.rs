use std::{hint::black_box, time::Instant};

use super::{aabb, circle, contact, overlaps};

/// Demonstrates caller-owned quadratic candidate enumeration without a broad phase.
#[test]
#[ignore = "manual all-pairs collision probe; run alone with --release"]
fn measure_collision_all_pairs() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!("collision_all_pairs,kind,objects,pairs,batch,elapsed_ns,hits");
    for kind in ["sparse", "dense"] {
        for count in [64_usize, 256, 1024] {
            let shapes = (0..count)
                .map(|index| {
                    circle(
                        [
                            if kind == "dense" {
                                0.0
                            } else {
                                f32::from(u16::try_from(index).unwrap()) * 4.0
                            },
                            0.0,
                        ],
                        1.0,
                    )
                })
                .collect::<Vec<_>>();
            let pairs = count * (count - 1) / 2;
            for batch in 0..22 {
                let start = Instant::now();
                let mut hits = 0;
                for (index, first) in shapes.iter().enumerate() {
                    for second in &shapes[index + 1..] {
                        hits += usize::from(overlaps(black_box(*first), black_box(*second)));
                    }
                }
                let elapsed = start.elapsed().as_nanos();
                assert_eq!(hits, if kind == "dense" { pairs } else { 0 });
                if batch >= 2 {
                    println!("collision_all_pairs,{kind},{count},{pairs},{batch},{elapsed},{hits}");
                }
            }
        }
    }
}

/// Manual narrow-phase batch probe; candidate generation is deliberately excluded.
#[test]
#[ignore = "manual collision scaling probe; run alone with --release"]
fn measure_collision_scaling() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!("collision,kind,pairs,batch,phase,elapsed_ns,hits");
    for kind in ["aabb", "circle", "mixed"] {
        for count in [1024, 16_384, 262_144] {
            let pairs = (0..count)
                .map(|index| {
                    let x = match index % 3 {
                        0 => 1.0,
                        1 => 2.0,
                        _ => 4.0,
                    };
                    match kind {
                        "aabb" => (aabb([0.0, 0.0], [1.0, 1.0]), aabb([x, 0.0], [1.0, 1.0])),
                        "circle" => (circle([0.0, 0.0], 1.0), circle([x, 0.0], 1.0)),
                        _ => (circle([x, 0.0], 1.0), aabb([0.0, 0.0], [1.0, 1.0])),
                    }
                })
                .collect::<Vec<_>>();
            let expected = count - count / 3;
            for batch in 0..22 {
                let start = Instant::now();
                let hits = pairs
                    .iter()
                    .filter(|(first, second)| {
                        black_box(overlaps(black_box(*first), black_box(*second)))
                    })
                    .count();
                let overlap = start.elapsed().as_nanos();
                assert_eq!(hits, expected);
                let start = Instant::now();
                let contacts = pairs
                    .iter()
                    .filter(|(first, second)| {
                        black_box(contact(black_box(*first), black_box(*second))).is_some()
                    })
                    .count();
                let contact_time = start.elapsed().as_nanos();
                assert_eq!(contacts, expected);
                if batch >= 2 {
                    for (phase, elapsed) in [("overlaps", overlap), ("contact", contact_time)] {
                        println!("collision,{kind},{count},{batch},{phase},{elapsed},{hits}");
                    }
                }
            }
        }
    }
}
