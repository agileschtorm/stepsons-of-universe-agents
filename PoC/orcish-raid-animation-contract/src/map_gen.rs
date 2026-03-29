use step_combat::map_utils::pt;
use thin_walls::{LevelGrid, Walkability};
use thin_walls::seeds::ObstacleShape;

pub const WIDTH: f32 = 40.0;
pub const HEIGHT: f32 = 25.0;

fn bordered_walkability_grid(width: usize, height: usize) -> LevelGrid<Walkability> {
    let mut grid: LevelGrid<Walkability> = LevelGrid::new(width, height);

    for i in 0..width {
        grid.set_at(pt(i, 0), Walkability::n());
        grid.set_at(pt(i, height - 1), Walkability::s());
    }

    for i in 0..height {
        let p = pt(0, i);
        grid.set_at(p, grid.get_at(p) | Walkability::w());
        let p = pt(width - 1, i);
        grid.set_at(p, grid.get_at(p) | Walkability::e());
    }

    grid
}

pub fn demo_map(width: usize, height: usize) -> LevelGrid<Option<ObstacleShape>> {
    let grid = bordered_walkability_grid(width, height);
    let mut result_grid: LevelGrid<Option<ObstacleShape>> = grid
        .map(|w| if w == Walkability::CLEAR { None } else { Some(w.into()) });

    for p in [pt(7, 6), pt(9, 18), pt(31, 6), pt(33, 18), pt(18, 5), pt(28, 20)] {
        result_grid.set_at(p, Some(ObstacleShape::SmallCircle));
    }

    for p in [pt(5, 19), pt(34, 8)] {
        result_grid.set_at(p, Some(ObstacleShape::BigCircle));
    }

    result_grid
}
