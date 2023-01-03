use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    color: Srgba,
    theta: f32,
    rate: f32,
    wh: Vec2
}

fn model(app: &App) -> Model {
    let color = srgba(60.0/255.0, 139.0/255.0, 108.0/255.0, 1.0);
    let theta = 0.0;
    let rate = 1.0;
    let wh = app.window_rect().wh();
    Model { color, theta, rate, wh }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.theta = app.time * model.rate;

    model.wh.x = (( model.theta.cos() + 1.0 ) * 0.5) * app.window_rect().w();
    model.wh.y = (( model.theta.sin() + 1.0 ) * 0.5) * app.window_rect().h();
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }
    draw.rect()
        .wh(model.wh * 0.5)
        /* .stroke_color(WHITE) */
        /* .stroke_weight(1.0) */
        .color(model.color);
    
    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,0.01));
    draw.to_frame(app, &frame).unwrap();
}