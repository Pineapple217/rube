use std::io::{self, Write};
use term_size;

const CUBE_INIT_STATE: [[f32; 3]; 8] = [
    // Front face
    [-1.0, -1.0, 1.0],
    [1.0, -1.0, 1.0],
    [1.0, 1.0, 1.0],
    [-1.0, 1.0, 1.0],
    // Back face
    [-1.0, -1.0, -1.0],
    [1.0, -1.0, -1.0],
    [1.0, 1.0, -1.0],
    [-1.0, 1.0, -1.0],
];

struct Cube {
    vertices: [[f32; 3]; 8],
}

struct FrameSize {
    x: usize,
    y: usize,
}

fn draai(x: &f32, y: &f32, z: &f32, a: &f32, b: &f32, c: &f32) -> (f32, f32, f32) {
    let new_x = y * a.sin() * b.sin() * c.cos() - z * a.cos() * b.sin() * c.cos()
        + y * a.cos() * c.sin()
        + z * a.sin() * c.sin()
        + x * b.cos() * c.cos();

    let new_y = y * a.cos() * c.cos() + z * a.sin() * c.cos() - y * a.sin() * b.sin() * c.sin()
        + z * a.cos() * b.sin() * c.sin()
        - x * b.cos() * c.sin();

    let new_z = z * a.cos() * b.cos() - y * a.sin() * b.cos() + x * b.sin();

    (new_x, new_y, new_z)
}

fn render_cube(cube: &Cube, frame_size: &FrameSize) -> Vec<Vec<char>> {
    let mut frame = vec![vec![' '; frame_size.x]; frame_size.y];
    let size = if frame_size.x > frame_size.y {
        frame_size.y
    } else {
        frame_size.x
    };
    let mut prev_corner: Option<(usize, usize)> = None;
    for corner in &cube.vertices {
        let pos_x = ((corner[0] * size as f32 * 0.4) + frame_size.x as f32 * 0.5) as usize;
        let pos_y = ((corner[1] * size as f32 * 0.4 * 0.6) + frame_size.y as f32 * 0.5) as usize;
        // frame[pos_y][pos_x] = '#';
        if let Some((p_x, p_y)) = prev_corner {
            for (x, y) in bresenham_line(*&pos_x as i32, *&pos_y as i32, *&p_x as i32, *&p_y as i32)
            {
                frame[y][x] = '#'
            }
        }

        prev_corner = Some((pos_x, pos_y));
    }
    frame
}

fn update_state(cube: &mut Cube) {
    for corner in &mut cube.vertices {
        let (x, y, z) = draai(&corner[0], &corner[1], &corner[2], &-0.02, &0.03, &-0.015);
        corner[0] = x;
        corner[1] = y;
        corner[2] = z;
    }
}

fn bresenham_line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(usize, usize)> {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut x = x0;
    let mut y = y0;
    let mut err = dx - dy;
    let mut pixels = vec![];

    while x != x1 || y != y1 {
        pixels.push((
            (x as usize).try_into().unwrap(),
            (y as usize).try_into().unwrap(),
        ));
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }

    pixels.push((
        (x1 as usize).try_into().unwrap(),
        (y1 as usize).try_into().unwrap(),
    ));
    pixels
    // let mut pixels = vec![];
    // let dx = x1 as f32 - x0 as f32;
    // let dy = y1 as f32 - y0 as f32;
    // let slope = dy.abs() / dx.abs();
    // let (mut x, mut y) = (x0 as f32, y0 as f32);
    // let (xstep, ystep) = if dx > 0.0 {
    //     (1, slope as i32)
    // } else {
    //     (-1, -(slope as i32))
    // };

    // pixels.push((x as usize, y as usize));

    // for _ in 0..dx.abs() as i32 {
    //     x += xstep as f32;
    //     y += ystep as f32;
    //     pixels.push((x as usize, y as usize));
    // }

    // pixels
}

fn main() {
    let (term_w, term_h) = term_size::dimensions().unwrap();
    let mut frame_size = FrameSize {
        x: term_w,
        y: term_h - 1,
    };

    let mut cube = Cube {
        vertices: CUBE_INIT_STATE,
    };

    for corner in &mut cube.vertices {
        let (x, y, z) = draai(&corner[0], &corner[1], &corner[2], &0.0, &0.0, &5.0);
        corner[0] = x;
        corner[1] = y;
        corner[2] = z;
    }

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    print!("{esc}[{n}A", esc = 27 as char, n = term_h); // Moves n lines up

    let mut frame;

    loop {
        let (term_w, term_h) = term_size::dimensions().unwrap();
        frame_size.x = term_w;
        frame_size.y = term_h - 1;
        update_state(&mut cube);
        frame = render_cube(&cube, &frame_size);
        for line in &frame {
            print!("{}\n", line.iter().collect::<String>())
        }

        print!("{esc}[{n}A", esc = 27 as char, n = term_h); // Moves n lines up
        io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(17));
    }
}
