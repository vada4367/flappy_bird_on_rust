use macroquad::prelude::*;
use macroquad::prelude::KeyCode::Enter;


fn window_conf() -> Conf {
    Conf {
        window_title: "Flappy Bird :)".to_owned(),
        window_width: 550,
        window_height: 700,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]

async fn main() {
    let bird_texture = Texture2D::from_file_with_format(
            include_bytes!("../resources/bird.png"),
            None,
    );
    let wallaper_texture = Texture2D::from_file_with_format(
            include_bytes!("../resources/wallaper.png"),
            None,
    );
    let mut bird_player :bird::Bird = bird::init();

    let mut obstacle2 :wall::Wall = wall::init(screen_width() / 2.0, (rand::rand() as i32) % 100 + 160);
    let mut obstacle1 :wall::Wall = wall::init(screen_width(), (rand::rand() as i32) % 100 + 160);
    loop {
        draw_texture(wallaper_texture, 0.0, 0.0, WHITE);

        wall::draw_obstacle(&obstacle1);
        wall::draw_obstacle(&obstacle2);

        bird::phys(&mut bird_player);

        let maxrotate :f32 = 7.0;
        let mut rotate :f32 = bird_player.y_speed / 7.0;
        if bird_player.y_speed > maxrotate { rotate = maxrotate / 7.0; }

        draw_texture_ex(
                bird_texture,
                (bird_player.x - 30) as f32,
                (bird_player.y - 35) as f32,
                WHITE,
            DrawTextureParams {
                    dest_size: Some(Vec2::new(90.0, 90.0)),
                    source: None,
                    rotation: rotate,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
            }
        );

        if bird_player.died {
            let mut died_text = "You are died. You score is ".to_string();
            died_text.push_str(&bird_player.score.to_string());
            died_text.push_str(&" Press enter to restart a game.".to_string());
            draw_text(&died_text,
                      screen_width() / 2.0 - 350.0,
                      screen_height() / 5.0, 40.0, WHITE);
            if is_key_pressed(Enter) {
                obstacle2 = wall::init(screen_width() / 2.0, (rand::rand() as i32) % 100 + 160);
                obstacle1 = wall::init(screen_width(), (rand::rand() as i32) % 100 + 160);
                bird_player = bird::init()
            }
        }
        else {
            draw_text(&bird_player.score.to_string(), screen_width() / 2.0 - 50.0, screen_height() / 6.0, 60.0, WHITE);
            if !((obstacle1.x + obstacle1.x_s + 10 >= bird_player.x &&
                  obstacle1.x + obstacle1.x_s <= bird_player.x) ||
                 (obstacle2.x + obstacle2.x_s + 10 >= bird_player.x &&
                  obstacle2.x + obstacle2.x_s <= bird_player.x)) &&
               bird_player.zarplata == 1
            {
                bird_player.score += 1;
                bird_player.zarplata = 0;
            } else if (obstacle1.x + obstacle1.x_s + 10 >= bird_player.x && obstacle1.x + obstacle1.x_s <= bird_player.x) || (obstacle2.x + obstacle2.x_s + 10 >= bird_player.x && obstacle2.x + obstacle2.x_s <= bird_player.x) { bird_player.zarplata = 1; }


            wall::move_obstacle(&mut obstacle1);
            wall::move_obstacle(&mut obstacle2);

            if is_key_down(Enter) && !(bird_player.died) { bird_player.do_jump = true; }
            else { bird_player.do_jump = false; }

            bird::jump(&mut bird_player);


            if bird::check_died(&mut bird_player, &mut obstacle2) ||
               bird::check_died(&mut bird_player, &mut obstacle1) { bird_player.died = true; }
        }

        next_frame().await
    }
}


mod wall {
    pub struct Wall {
        pub x : i32,
        pub y : i32,
        pub speed : i32,
        pub x_s : i32,
        pub y_s : i32,
    }

pub fn init(wd :f32, y_s :i32) -> Wall
{
    let wall = Wall {x : ((wd as i32)), y : y_s, speed : 2, x_s : 100, y_s : 140};
    wall
}

pub fn move_obstacle(wall :&mut Wall) {
        if wall.x > -100 {
            wall.x = wall.x - wall.speed;
        } else {
            use macroquad::prelude::*;

            wall.x = screen_width() as i32;
            wall.y = ((rand::rand() as f32) % (screen_height() - 200.0)) as i32;
        }
    }

pub fn draw_obstacle(wall :&Wall) {
        use macroquad::prelude::*;

        draw_rectangle(wall.x as f32,
        0.0,
        100.0,
        wall.y as f32,
        Color {r : 0.2, g : 1.0, b : 0.0, a : 1.0});
        draw_rectangle(wall.x as f32,
        wall.y as f32 + 140.0,
        100.0,
        screen_height(),
        Color {r : 0.2, g : 1.0, b : 0.0, a : 1.0});
    }
}

mod bird {
    pub struct Bird {
        pub x: i32,
        pub y: i32,
        pub died: bool,
        pub do_jump: bool,
        pub grav: i32,
        pub y_speed: f32,
        pub x_s: i32,
        pub y_s: i32,
        pub score: i32,
        pub zarplata: i32,
    }

pub fn init() -> Bird
{
    let bird = Bird {x : 70,
        y : 150,
        died : false,
        do_jump : false,
        grav : 2,
        y_speed : 0.0,
        x_s : 40,
        y_s : 30,
        score : 0,
        zarplata : 0,
    };
    bird
}

pub fn phys(bird :&mut Bird) {
        use macroquad::prelude::*;

        bird.y_speed = bird.y_speed + (bird.grav as f32) / 7.0;
        bird.y = bird.y + (bird.y_speed as i32);
        if (bird.y + bird.y_s) as f32 > screen_height() {
            bird.y_speed = 0.0;
            bird.y = screen_height() as i32 - 50;
        }
    }

    pub fn jump(bird :&mut Bird) {
        if bird.do_jump { bird.y_speed = -6.0; }
    }

    use crate::wall::Wall;
    pub fn check_died(bird :&mut Bird, wall :&mut Wall) -> bool
    {
        if  !(bird.died) && ((bird.x > wall.x && bird.x < wall.x + wall.x_s) ||
            (bird.x_s + bird.x > wall.x && bird.x_s + bird.x < wall.x + wall.x_s)) {
            if bird.y < wall.y || bird.y + bird.y_s > wall.y + wall.y_s
            {
                return true;
            }
        }
        false
    }
}
