pub fn parse_input(input: &str) -> Vec<Shape> {
    let mut shapes = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.trim().chars().enumerate() {
            if char == 'O' {
                shapes.push(Shape {
                    shape: Shapes::Round,
                    x,
                    y,
                });
            } else if char == '#' {
                shapes.push(Shape {
                    shape: Shapes::Cube,
                    x,
                    y,
                });
            }
        }
    }

    shapes
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Shape {
    pub shape: Shapes,
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Shapes {
    Round,
    Cube,
}

#[derive(PartialEq)]
pub enum Dir {
    North,
    West,
    South,
    East,
}
