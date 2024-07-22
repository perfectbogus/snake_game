use std::collections::HashSet;
use super::snake::{Snake, Direction};
use termion::event::Key;
use termion::input::TermRead;

fn get_user_input(stdin: &mut impl TermRead) -> Option<Direction> {
    let key = match stdin.read_line() {
        Ok(key) => key,
        Err(_) => return None
    };
}

fn move_snake(snake: &mut Snake) {
    let mut head = snake.body[0];
    match snake.direction {
        Direction::Up => head.1 -= 1,
        Direction::Down => head.1 += 1,
        Direction::Left => head.0 -= 1,
        Direction::Right => head.0 += 1
    }

    snake.body.insert(0, head);
    snake.body.pop();
}

fn check_collision(snake: &Snake, board_width: i32, board_height: i32) -> bool {
    let head = snake.body[0];
    head.0 < 0 || head.0 >= board_width || head.1 < 0 || head.1 >= board_height || snake.body.iter().skip(1).any(|segmento| segmento == &head)
}