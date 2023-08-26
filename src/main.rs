extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::mouse::MouseButton;
use std::time::{SystemTime, UNIX_EPOCH};

const WORLD_SIZE: usize = 32;
const CELL_SIZE: u32 = 25;

type WorldType = [[bool; WORLD_SIZE]; WORLD_SIZE];

#[inline]
fn next_cell_state(current_state: bool, number_of_alive_neighbours: u32) -> bool {
    if current_state { // if alive
        return number_of_alive_neighbours == 2 || number_of_alive_neighbours == 3
    } else { // if dead
        return number_of_alive_neighbours == 3
    }
}

#[inline]
fn bound_check(x: i32, y: i32, length: usize) -> bool {
    return 0 <= x && x < length as i32 && 0 <= y && y < length as i32;
}

#[inline]
fn get_neighbor_count(world: WorldType, x: i32, y: i32) -> u32 { // FIXME: this needs a fresh rewrite (fourth time)
    let mut alive_neighbour_count = 0;
    if bound_check(x-1, y-1, WORLD_SIZE) { // <^
        if world[y as usize - 1][x as usize-1] {
            alive_neighbour_count += 1;
        }
    }
    if bound_check(x-1, y, WORLD_SIZE) { // ^
        if world[y as usize][x as usize-1] {
            alive_neighbour_count += 1;
        }
    }
    if bound_check(x-1, y+1, WORLD_SIZE) { // ^>
        if world[y as usize+1][x as usize-1] {
            alive_neighbour_count += 1;
        }
    }
    if bound_check(x, y-1, WORLD_SIZE) { // <
        if world[y as usize-1][x as usize] {
            alive_neighbour_count += 1;
        }
    }
    if bound_check(x, y+1, WORLD_SIZE) { // > 
        if world[y as usize+1][x as usize] {
            alive_neighbour_count += 1;
        }
    }
    if bound_check(x+1, y-1, WORLD_SIZE) { // <v
        if world[y as usize-1][x as usize+1] {
            alive_neighbour_count += 1;
        }
    }
    if bound_check(x+1, y, WORLD_SIZE) { // v
        if world[y as usize][x as usize+1] {
            alive_neighbour_count += 1;
        }
    }
    if bound_check(x+1, y+1, WORLD_SIZE) { // v>
        if world[y as usize+1][x as usize+1] {
            alive_neighbour_count += 1;
        }
    }
    return alive_neighbour_count;
}

fn next_state(world: &mut WorldType) {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut new_world: WorldType = world.clone();
    while y < WORLD_SIZE {
        while x < WORLD_SIZE {
            let number_of_alive_neighbours: u32 = get_neighbor_count(*world, x as i32, y as i32);
            new_world[y][x] = next_cell_state(world[y][x], number_of_alive_neighbours);
            x += 1;
        }
        x = 0;
        y += 1;
    }
    world.clone_from(&new_world);
}

#[inline]
fn draw_world(canvas: &mut Canvas<Window>, world: &WorldType, alive_color: Color) {
    canvas.set_draw_color(alive_color);
    let mut x = 0;
    let mut y = 0;
    while y < WORLD_SIZE {
        while x < WORLD_SIZE {
            /*canvas.set_draw_color(Color::RGB((amount_of_neighbours[y][x] * 30) as u8, 0, 0));
            if world[y][x] {
                canvas.set_draw_color(Color::RGB(0, (amount_of_neighbours[y][x] * 30) as u8+10, 20));
            }*/
            if world[y][x] {
                canvas.fill_rect(Rect::new((x*CELL_SIZE as usize) as i32, (y*CELL_SIZE as usize) as i32,CELL_SIZE,CELL_SIZE)).expect("Cannot fill Rect");
            }
            x += 1;
        }
        y += 1;
        x = 0;
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 CGOL", CELL_SIZE*WORLD_SIZE as u32, CELL_SIZE*WORLD_SIZE as u32)
        .position_centered()
        .build()
        .unwrap();

    // Canvas setup
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // Initialize world
    let mut world: WorldType = [[false; WORLD_SIZE]; WORLD_SIZE];//vec![vec![false; WORLD_SIZE]; WORLD_SIZE];
    let mut pause: bool = false;
    let mut step_speed: u64 = 3; // greater the value slower the program gets
    let mut step: i32 = 0;
    let mut cycles: u64 = 0;
    let mut start = SystemTime::now();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                Event::MouseButtonUp { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
                    if mouse_btn == MouseButton::Left {
                        let row = x / (CELL_SIZE as i32);
                        let column = y / (CELL_SIZE as i32);
                        world[column as usize][row as usize] = true;
                    } else if mouse_btn == MouseButton::Right {
                        let row = x / (CELL_SIZE as i32);
                        let column = y / (CELL_SIZE as i32);
                        world[column as usize][row as usize] = false;
                    }
                },
                Event::KeyUp { timestamp, window_id, keycode, scancode, keymod, repeat } => {
                    match keycode.unwrap() {
                        Keycode::S => {
                            next_state(&mut world);
                            step += 1;
                            println!("{step} steps");
                        },
                        Keycode::Space => {
                            pause = !pause;
                        },
                        Keycode::Up => {
                            step_speed += 1;
                        },
                        Keycode::Down => {
                            if step_speed != 1 {
                                step_speed -= 1;
                            }
                        },
                        Keycode::R => {
                            world.clone_from(&mut [[false; WORLD_SIZE]; WORLD_SIZE]);
                            cycles = 0;
                            step_speed = 3;
                            pause = false;
                            step = 0;
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        if !pause && cycles % step_speed == 0 {
            next_state(&mut world);
            step += 1;
        }
        cycles += 1;
        
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        draw_world(&mut canvas, &world, Color::RGB(255, 255, 255));
        canvas.present();
        

        // The rest of the game loop goes here...
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}



#[cfg(test)]
mod cgol {
    use super::*; // import outer names

    fn test_world() -> WorldType {
        let mut world: WorldType = [[false; WORLD_SIZE]; WORLD_SIZE];
        world[1][1] = true;
        world[1][2] = true;
        world[2][1] = true;
        world[2][2] = true;
        return world;
    }

    #[test]
    fn test_next_state() {
        let mut world = test_world();
        let expected: WorldType = world.clone();
        next_state(&mut world);
        assert_eq!(world, expected);
    }

    #[test]
    fn test_get_neighbor_count() {
        let world = test_world();
        assert_eq!(get_neighbor_count(world, 0, 0), 1);
        assert_eq!(get_neighbor_count(world, 1, 1), 3);
        assert_eq!(get_neighbor_count(world, 2, 1), 3);
        assert_eq!(get_neighbor_count(world, 1, 2), 3);
        assert_eq!(get_neighbor_count(world, 2, 2), 3);
        assert_eq!(get_neighbor_count(world, 2, 3), 2);
        assert_eq!(get_neighbor_count(world, 1, 3), 2);
    }

    #[test]
    fn test_next_cell_state() {
        assert_eq!(next_cell_state(true, 0), false);
        assert_eq!(next_cell_state(true, 1), false);
        assert_eq!(next_cell_state(true, 2), true);
        assert_eq!(next_cell_state(true, 3), true);
        assert_eq!(next_cell_state(true, 4), false);
        assert_eq!(next_cell_state(false, 3), true);
        assert_eq!(next_cell_state(false, 1), false);
    }
}