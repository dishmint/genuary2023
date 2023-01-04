use nannou::prelude::*;
use rand::{Rng, thread_rng};
use std::cmp::Ordering;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(400,400)
        .run();
}

enum Channel {
    Red,
    Green,
    Blue
}

struct Model {
    data: Vec<(Point2, Srgb)>,
    channel: Channel,
    chunks: Vec<Vec<(Point2, Srgb)>>
}

fn model(app: &App) -> Model {
    let win = app.window_rect();
    let data_size = (win.w() * win.h()) as i32;
    let mut data: Vec<(Point2, Srgb)> = Vec::new();

    let mut y = 0;
    for i in 1..=data_size {
        let color: Srgb = srgb(thread_rng().gen_range(0.0..=1.0),thread_rng().gen_range(0.0..=1.0),thread_rng().gen_range(0.0..=1.0));
        let x = i as i32 % win.w() as i32;
        if i > win.w() as i32 && x == 0 {
            y += 1;
        };
        let x_map = map_range(x, 0, win.w() as i32, win.left(), win.right());
        let y_map = map_range(y, 0, win.h() as i32, win.top(), win.bottom());
        data.push((pt2(x_map, y_map), color));
    }

    let mut chunks:Vec<Vec<(Point2, Srgb)>> = Vec::new();

    for column in 0..app.window_rect().w() as usize {
        let mut chunk = Vec::new();
        for row in 0..app.window_rect().h() as usize {
            let ix = (column as i32 + row as i32 * app.window_rect().w() as i32) as usize;
            let ix = ix.clamp(0, data.len());
            chunk.push(data[ix]);
        }
        chunks.push(chunk);
    };

    let chunks = chunks;

    let channel = Channel::Red;

    Model { data, channel, chunks }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for slice in model.chunks.iter_mut() {
        match model.channel {
            Channel::Red => slice.sort_by(|a,b| if a.1.red.gt(&b.1.red) {Ordering::Greater} else {Ordering::Less}),
            Channel::Green => slice.sort_by(|a,b| if a.1.green.gt(&b.1.green) {Ordering::Greater} else {Ordering::Less}),
            Channel::Blue => slice.sort_by(|a,b| if a.1.blue.gt(&b.1.blue) {Ordering::Greater} else {Ordering::Less}),
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }
    let verts = (0..model.data.len())
        .map(|i| {
            model.data[i]
        });
    draw.polyline()
        .points_colored(verts);

    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,0.01));
    draw.to_frame(app, &frame).unwrap();
}