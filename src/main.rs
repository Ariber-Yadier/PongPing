use macroquad::prelude::*;

fn window_config() -> Conf {
  Conf {
    window_title: "Pong Ping".to_string(),
    window_width: 640,
    window_height: 480,

    ..Default::default()
  }
}

enum State {
  Paused,
  Playing,
}

struct Paddle {
  id: u32,
  x: f32,
  y: f32,
}
impl Paddle {
  pub fn new(id: u32) -> Paddle {
    // Compare id and set x to the correct value
    let x = match id {
      1 => 0.0,
      2 => screen_width() - 16.0,

      _ => panic!("Not a valid id: {}", id),
    };
    // Set y to the middle
    let y = screen_height() / 2.0 - 32.0;

    Paddle { id, x, y }
  }

  pub fn control(&mut self) {
    // Match id to differentiate controls
    let up = match self.id {
      1 => is_key_down(KeyCode::W),
      2 => is_key_down(KeyCode::Up),

      _ => false,
    };
    let down = match self.id {
      1 => is_key_down(KeyCode::S),
      2 => is_key_down(KeyCode::Down),

      _ => false,
    };

    if up && self.y >= 0.0 {
      self.y -= 10.0;
    }
    if down && self.y <= screen_height() - 64.0 {
      self.y += 10.0;
    }
  }

  pub fn draw(&self) { draw_rectangle(self.x, self.y, 16.0, 64.0, WHITE); }
}

struct Ball {
  x: f32,
  y: f32,
  dir: (f32, f32),
}
impl Ball {
  pub fn new() -> Ball {
    // Generate a random number for direction
    let dir_x = match rand::gen_range(-1, 2) <= 0 {
      true => 1,
      false => -1,
    };
    let dir_y = match rand::gen_range(-1, 2) <= 0 {
      true => 1,
      false => -1,
    };

    Ball {
      x: screen_width() / 2.0 - 8.0,
      y: screen_height() / 2.0 - 8.0,
      dir: (dir_x as f32, dir_y as f32),
    }
  }

  pub fn collide(&mut self, player_1: &Paddle, player_2: &Paddle) -> (bool, bool) {
    // If the ball's x position is the same as the paddles position and the ball's y
    // position is in range of the paddles y and the paddles y plus it's size then
    // set dir to negative
    if self.x <= player_1.x + 16.0 && (self.y >= player_1.y && self.y <= player_1.y + 64.0) {
      self.dir.0 = 1.0;
    } else if self.x >= player_2.x && (self.y >= player_2.y && self.y <= player_2.y + 64.0) {
      self.dir.0 = -1.0;
    }

    if self.y <= 0.0 || self.y >= screen_height() - 16.0 {
      self.dir.1 = match self.dir.1 as i32 {
        -1 => 1.0,
        1 => -1.0,

        _ => 0.0,
      }
    }

    if self.x <= -16.0 {
      (true, false)
    } else if self.x >= screen_width() {
      (false, true)
    } else {
      (false, false)
    }
  }

  pub fn move_dir(&mut self) {
    self.x += self.dir.0 * 6.0;
    self.y += self.dir.1 * 6.0;
  }

  pub fn draw(&self) { draw_rectangle(self.x, self.y, 16.0, 16.0, WHITE); }
}

#[macroquad::main(window_config)]
async fn main() {
  let mut state = State::Paused;

  let mut player_1 = Paddle::new(1);
  let mut player_2 = Paddle::new(2);

  let mut ball = Ball::new();

  let mut p1_score = 0;
  let mut p2_score = 0;

  loop {
    clear_background(BLACK);

    match state {
      State::Paused => {
        let text = "Press space to play!";
        let text_size = measure_text(text, None, 44, 1.0);
        draw_text(
          text,
          screen_width() / 2.0 - text_size.width / 2.0,
          screen_height() / 2.0 - text_size.height / 2.0,
          44.0,
          WHITE,
        );
        if is_key_pressed(KeyCode::Space) {
          state = State::Playing;
        }
      },
      State::Playing => {
        player_1.control();
        player_2.control();

        ball.move_dir();
        match ball.collide(&player_1, &player_2) {
          (true, false) => {
            p2_score += 100;
            player_1 = Paddle::new(1);
            player_2 = Paddle::new(2);
            ball = Ball::new();
            state = State::Paused;
          },
          (false, true) => {
            p1_score += 100;
            player_1 = Paddle::new(1);
            player_2 = Paddle::new(2);
            ball = Ball::new();
            state = State::Paused;
          },

          _ => (),
        }
      },
    }

    player_1.draw();
    player_2.draw();
    ball.draw();

    draw_line(
      screen_width() / 2.0,
      0.0,
      screen_width() / 2.0,
      screen_height(),
      2.0,
      WHITE,
    );

    draw_text(
      format!("P1: {}, P2: {}", p1_score, p2_score).as_str(),
      10.0,
      10.0,
      15.0,
      WHITE,
    );

    next_frame().await;
  }
}
