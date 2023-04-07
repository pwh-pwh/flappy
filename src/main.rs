use std::thread::sleep;
use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION:f32 = 75.0;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    player:Player,
    frame_time:f32,
    mode: GameMode,
    obstacle:Obstacle,
    score: i32,
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x:i32,y:i32) -> Self {
        Self {
            x,y,
            velocity: 0.0
        }
    }
    fn render(&mut self,ctx:&mut BTerm) {
        ctx.set(0,self.y,YELLOW,BLACK,to_cp437('☺'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity<2.0 {
            self.velocity +=  0.2;
        }

        self.y += self.velocity as i32;
        self.x +=1;
        if self.y<0 {
            self.y = 0;
        }
    }
    fn flap(&mut self) {
        self.velocity = -2.0;
    }

}

impl State {
    fn new() -> Self {
        Self {
            player: Player::new(5,25),
            frame_time: 0.0,
            mode: GameMode::Menu,
            obstacle:Obstacle::new(SCREEN_WIDTH,0),
            score: 0,
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5,25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.obstacle = Obstacle::new(SCREEN_WIDTH,0);
        self.score = 0;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "welcome to my game");
        ctx.print_centered(8, "(P) play game");
        ctx.print_centered(9, "(Q) exit game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time>FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0,0,"press space to flap");
        ctx.print(0,1,&format!("Score:{}",self.score));

        self.obstacle.render(ctx,self.player.x);
        if self.player.x > self.obstacle.x {
            self.score+=1;
            self.obstacle = Obstacle::new(self.player.x+SCREEN_WIDTH,self.score);
        }
        if self.player.y > SCREEN_HEIGHT || (self.obstacle.hit_obstacle(&self.player)) {
            self.mode = GameMode::End;
        }

    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "you are dead");
        ctx.print_centered(6,&format!("you earned {} points",self.score));
        ctx.print_centered(8, "(P) play game");
        ctx.print_centered(9, "(Q) exit game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

struct Obstacle {
    x:i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x:i32,score: i32) -> Self {
        let mut generator = RandomNumberGenerator::new();
        Self {
            x,
            gap_y: generator.range(10,40),
            size: i32::max(2,20-score),
        }
    }

    fn render(&mut self,ctx:&mut BTerm,player_x:i32) {
        let screen_x = self.x - player_x; // 屏幕上的空间
        let half_size = self.size / 2;

        // 上边的
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
        // 下边的
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'))
        }
    }

    fn hit_obstacle(&self,player:&Player) ->bool {
        let half_size = self.size/2;
        let does_x_match = self.x == player.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y> self.gap_y+half_size;
        does_x_match && (player_above_gap||player_below_gap)
    }

}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("my game")
        .build()?;
    println!("Hello, world!");
    main_loop(context, State::new())
}
