use std::io::{BufWriter, Write};
use std::{
    thread,
    time::{Duration, Instant},
};
use term_size;

const CUBE_INIT_STATE: [[f32; 3]; 8] = [
    [1.0, 1.0, 1.0],
    [1.0, -1.0, 1.0],
    [-1.0, -1.0, 1.0],
    [-1.0, 1.0, 1.0],
    [-1.0, 1.0, -1.0],
    [-1.0, -1.0, -1.0],
    [1.0, -1.0, -1.0],
    [1.0, 1.0, -1.0],
];

struct Cube {
    vertices: [[f32; 3]; 8],
}

struct FrameSize {
    x: usize,
    y: usize,
}

fn draai(x: &f32, y: &f32, z: &f32, a: &f32, b: &f32, c: &f32) -> (f32, f32, f32) {
    let a_sin = a.sin();
    let b_sin = b.sin();
    let c_sin = c.sin();

    let a_cos = a.cos();
    let b_cos = b.cos();
    let c_cos = c.cos();

    let new_x = y * a_sin * b_sin * c_cos - z * a_cos * b_sin * c_cos
        + y * a_cos * c_sin
        + z * a_sin * c_sin
        + x * b_cos * c_cos;

    let new_y = y * a_cos * c_cos + z * a_sin * c_cos - y * a_sin * b_sin * c_sin
        + z * a_cos * b_sin * c_sin
        - x * b_cos * c_sin;

    let new_z = z * a_cos * b_cos - y * a_sin * b_cos + x * b_sin;

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
        let pos_y = ((corner[1] * size as f32 * 0.4 * 0.55) + frame_size.y as f32 * 0.5) as usize;
        // frame[pos_y][pos_x] = '#';
        if let Some((p_x, p_y)) = prev_corner {
            for (x, y) in bresenham_line(*&pos_x as i32, *&pos_y as i32, *&p_x as i32, *&p_y as i32)
            {
                frame[y][x] = '#';
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

    let mut frame;
    // let wait_time = Duration::from_millis(17);
    let wait_time = Duration::from_secs_f32(1.0 / 60.0);

    loop {
        let start = Instant::now();

        let (term_w, term_h) = term_size::dimensions().unwrap();
        frame_size.x = term_w;
        frame_size.y = term_h - 2;
        update_state(&mut cube);
        frame = render_cube(&cube, &frame_size);

        let stdout = std::io::stdout();
        let lock = stdout.lock();
        let mut buffer = BufWriter::with_capacity(frame_size.x * (frame_size.y + 2), lock); // er is ergens een buffer size limit of zo ma idk

        writeln!(buffer, "{esc}[{n}A", esc = 27 as char, n = term_h); // Moves n lines up
        for line in &frame {
            writeln!(buffer, "{}", line.iter().collect::<String>()).unwrap();
        }

        let runtime = start.elapsed();

        if let Some(remaining) = wait_time.checked_sub(runtime) {
            write!(buffer, "frametime: {:?}", runtime);
            buffer.flush().unwrap();
            thread::sleep(remaining);
        } else {
            buffer.flush().unwrap();
        }
    }
}
