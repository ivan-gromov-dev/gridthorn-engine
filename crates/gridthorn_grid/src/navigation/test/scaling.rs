use std::hash::{Hash, Hasher};
use std::{hint::black_box, time::Instant};

use crate::{GridCell, NavigationBounds, PathSearch, PathStatus, search_path};

/// Manual synchronous search probe with deterministic terrain and owned diagnostics.
#[test]
#[ignore = "manual navigation scaling probe; run alone with --release"]
fn measure_navigation_scaling() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!(
        "navigation,kind,side,budget,batch,elapsed_ns,visited,frontier,path,diagnostic_capacity_bytes"
    );
    for side in [32_i32, 128, 512] {
        for kind in ["open", "weighted", "wall", "budget"] {
            let bounds =
                NavigationBounds::new(GridCell::default(), GridCell::new(side - 1, side - 1))
                    .unwrap();
            let goal = GridCell::new(side - 1, side - 1);
            let cells = usize::try_from(side * side).unwrap();
            let budget = if kind == "budget" {
                1024.min(cells / 4)
            } else {
                cells
            };
            for batch in 0..22 {
                let start = Instant::now();
                let result = search_path(bounds, GridCell::default(), goal, budget, |cell| {
                    terrain(kind, side, black_box(cell))
                })
                .unwrap();
                let elapsed = start.elapsed().as_nanos();
                validate(&result, kind, side, budget, goal);
                if batch == 0 {
                    let mut fingerprint = std::collections::hash_map::DefaultHasher::new();
                    format!("{result:?}").hash(&mut fingerprint);
                    println!(
                        "navigation_fingerprint,{kind},{side},{:016x}",
                        fingerprint.finish()
                    );
                }
                if batch >= 2 {
                    let bytes = result.visited.capacity() * std::mem::size_of::<(GridCell, u64)>()
                        + result.frontier.capacity() * std::mem::size_of::<(GridCell, u64)>()
                        + result.path.capacity() * std::mem::size_of::<GridCell>();
                    println!(
                        "navigation,{kind},{side},{budget},{batch},{elapsed},{},{},{},{bytes}",
                        result.visited.len(),
                        result.frontier.len(),
                        result.path.len()
                    );
                }
            }
        }
    }
    if std::env::var_os("GRIDTHORN_NAVIGATION_MEMORY").is_some() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn terrain(kind: &str, side: i32, cell: GridCell) -> Option<u32> {
    if kind == "wall" && cell.column == side / 2 {
        None
    } else if kind == "weighted" {
        Some(1 + u32::try_from((cell.column * 17 + cell.row * 31) % 9).unwrap())
    } else {
        Some(1)
    }
}

fn validate(result: &PathSearch, kind: &str, side: i32, budget: usize, goal: GridCell) {
    assert!(result.visited.len() <= budget);
    assert!(
        result
            .frontier
            .windows(2)
            .all(|pair| (pair[0].1, pair[0].0) < (pair[1].1, pair[1].0))
    );
    match kind {
        "wall" => {
            assert_eq!(result.status, PathStatus::Unreachable);
            assert_eq!(
                result.visited.len(),
                usize::try_from(side * (side / 2)).unwrap()
            );
            assert_eq!(result.frontier, []);
            assert_eq!(result.path, []);
            assert_eq!(result.cost, None);
        }
        "budget" => {
            assert_eq!(result.status, PathStatus::BudgetExceeded);
            assert_eq!(result.visited.len(), budget);
            assert_ne!(result.frontier, []);
            assert_eq!(result.path, []);
            assert_eq!(result.cost, None);
        }
        _ => {
            assert_eq!(result.status, PathStatus::Found);
            assert_eq!(result.path.first(), Some(&GridCell::default()));
            assert_eq!(result.path.last(), Some(&goal));
            let cost = result
                .path
                .iter()
                .skip(1)
                .map(|cell| u64::from(terrain(kind, side, *cell).unwrap()))
                .sum::<u64>();
            assert_eq!(result.cost, Some(cost));
            assert!(
                result
                    .path
                    .windows(2)
                    .all(|pair| (pair[0].column - pair[1].column).abs()
                        + (pair[0].row - pair[1].row).abs()
                        == 1)
            );
            if kind == "open" {
                assert_eq!(cost, u64::try_from(2 * (side - 1)).unwrap());
                assert_eq!(result.visited.len(), usize::try_from(side * side).unwrap());
            }
        }
    }
}
