extern crate ggez;
extern crate rand;

use ggez::event;
use ggez::Context;
use ggez::GameResult;
use ggez::ContextBuilder;
use ggez::graphics;
use ggez::timer;

use ggez::conf::{WindowSetup, WindowMode};
use ggez::graphics::{Point2, Vector2};
use ggez::event::{Keycode, Mod};

use rand::{thread_rng, Rng};

const SECONDS: u32 = 20;

enum ObjectType {
    Player,
    Food,
    Shit,
}

struct Player {
    pos: Point2,
    id: ObjectType
}

impl Player {
    fn new() -> Player {
        Player {
            pos: Point2::new(0.0, 0.0),
            id: ObjectType::Player,
        }
    }
}

struct Food {
    pos: Point2,
    id: ObjectType,
    eaten: bool,
    score: u32,
    poison: bool,
    speed: f32,
}

impl Food {
    fn new() -> Food {
        Food {
            pos: Point2::new(0., 0.),
            id: ObjectType::Food,
            eaten: true,
            score: 0,
            poison: false,
            speed: 20.0,
        }
    }
}

struct Assets {
    player_image: graphics::Image,
    food_image: graphics::Image,
    shit_image: graphics::Image,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_image = graphics::Image::new(ctx, "/player.png")?;
        let food_image = graphics::Image::new(ctx, "/food.png")?;
        let shit_image = graphics::Image::new(ctx, "/shit.png")?;

        Ok(Assets {
            player_image,
            food_image,
            shit_image,
        })
    }
    fn image_bind(&mut self, id: &ObjectType) -> &mut graphics::Image {
        match id {
            ObjectType::Player => return &mut self.player_image,
            ObjectType::Food => return &mut self.food_image,
            ObjectType::Shit => return &mut self.shit_image,
        }
    }
}

struct KeyInput {
    x: i32,
    y: i32,
    start: bool,
}

impl KeyInput {
    fn new() -> KeyInput {
        KeyInput {
            x: 0,
            y: 0,
            start: false,
        }
    }
}

struct Mainstate {
    player: Player,
    food: Food,
    input: KeyInput,
    assets: Assets,
}

impl Mainstate {
    fn new(ctx: &mut Context) -> GameResult<Mainstate> {
        println!("W: Up");
        println!("S: Down");
        println!("Space: Start game and restart on death");
        
        let player = Player::new();
        let food = Food::new();        
        let input = KeyInput::new();
        let assets = Assets::new(ctx)?;

        Ok(Mainstate {
            player,
            food,
            input,
            assets,
        })
    }
}

fn move_player(input: &KeyInput, player: &mut Player) {
    match input {
        KeyInput {x: 0, y: 1, start: true} => player.pos.y += 1.0,
        KeyInput {x: 0, y: -1, start: true} => player.pos.y -= 1.0,
        _ => (),
    }
    
}

fn food_spawn(ctx: &mut Context, input: &mut KeyInput, food: &mut Food, player: &Player) {
    if food.eaten == true {
        let mut rng = thread_rng();
        food.pos.x = 800.0;
        food.pos.y = rng.gen_range(0, 7) as f32;
        food.pos.y *= 52.0;
        food.eaten = false;
        food.speed += 2.0;
    
        let poison = rng.gen_range(0, 7);
        if poison == 0 {
            food.poison = true;
            food.id = ObjectType::Shit;
        }
        else {
            food.poison = false;
            food.id = ObjectType::Food;
        }
    }
    

    food.pos.x -= food.speed;

    let food_comp_y = food.pos.y / 52.0;

    if food.poison == false && food_comp_y == player.pos.y && food.pos.x <= player.pos.x && food.pos.x >= -50.0 {
        food.eaten = true;
        food.score += 1;
        println!("{}", food.score);
    }
    else if food.poison == true && food.pos.x <= -50.0 {
        food.eaten = true;
        food.score += 1;
        println!("{}", food.score);
    }
    else if food.poison == true && food_comp_y == player.pos.y && food.pos.x <= player.pos.x && food.pos.x >= -50.0 {
        input.start = false;
    }
    else if food.poison == false && food.pos.x <= -50.0 {
        input.start = false;
    }
}

fn restart(input: &KeyInput, player: &mut Player, food: &mut Food) {
    if input.start == false {
        player.pos = Point2::new(0., 0.);
        food.pos = Point2::new(0., 0.);
        food.eaten = true;
        food.score = 0;
        food.speed = 20.0;
    }
}

fn render_background(ctx: &mut Context, assets: &mut Assets) -> GameResult<()> {
    let background = graphics::Image::new(ctx, "/background.png")?;
    let draw_param = graphics::DrawParam {
        ..Default::default()
    };
    graphics::draw_ex(ctx, &background, draw_param)
}

fn render_player(ctx: &mut Context, assets: &mut Assets, player: &Player) -> GameResult<()> {
    let image = assets.image_bind(&player.id);
    let draw_param = graphics::DrawParam {
        dest: player.pos * (52.0),
        ..Default::default()
    };
    graphics::draw_ex(ctx, image, draw_param)
}

fn render_food(ctx: &mut Context, assets: &mut Assets, food: &Food) -> GameResult<()> {
    let image = assets.image_bind(&food.id);
    let draw_param = graphics::DrawParam {
        dest: food.pos,
        ..Default::default()
    };
    graphics::draw_ex(ctx, image, draw_param)
}



impl event::EventHandler for Mainstate {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, SECONDS) {
            if self.input.start == true {
                move_player(&self.input, &mut self.player);
                food_spawn(ctx, &mut self.input, &mut self.food, &self.player);
            }
            restart(&self.input, &mut self.player, &mut self.food);
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_background_color(ctx, graphics::Color::new(0., 0.5, 0.5, 0.,));

        let assets = &mut self.assets;
        let player = &self.player;
        let food = &self.food;

        render_player(ctx, assets, player);
        render_food(ctx, assets, food);

        graphics::present(ctx);
        Ok(())
    }
    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::W => self.input.y = -1,
            Keycode::S => self.input.y = 1,
            _ => (),
        }
    }
    fn key_up_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::W => self.input.y = 0,
            Keycode::S => self.input.y = 0,
            Keycode::Space => self.input.start = true,
            _ => (),
        }
    }
}

fn main() {
    
    let cb = ContextBuilder::new("RPC", "Usummon") 
        .window_setup(WindowSetup::default().title("RPC"))
        .window_mode(WindowMode::default().dimensions(800, 396));

    let ctx = &mut cb.build().unwrap();
    let state = &mut Mainstate::new(ctx).unwrap();

    event::run(ctx, state); 

}
