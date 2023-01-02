use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    title: String,
    length: f32,
    rate: f32,
}

fn model(app: &App) -> Model {
    let day = 1;
    let description = String::from("A Perfect Loop");
    let title = format!("{day}: {description}");

    let win = app.window_rect();
    let length = win.w() * 0.33;
    let rate = 0.25;

    Model {
        title,
        length,
        rate
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.rate = app.time * 0.5;
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();

    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }
    
    draw.line()
        .start(win.xy() - vec2(model.length,0.0))
        .end(  win.xy() + vec2(model.length, 0.0))
        .stroke_weight(0.5)
        .color(WHITE)
        .roll(model.rate)
        .pitch(model.rate);
    
    draw.ellipse()
        .no_fill()
        .radius(model.length)
        .stroke_weight(1.0)
        .stroke_color(BLUE)
        .yaw(model.rate)
        .pitch(model.rate);

    draw.text(&model.title)
        .xy(win.mid_bottom()+vec2(0.0,14.0))
        .color(WHITE)
        .font_size(14);

    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,0.05));
    draw.to_frame(app, &frame).unwrap();
}