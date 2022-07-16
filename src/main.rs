use nannou::prelude::*;
use nannou_osc as osc;

fn main() {
    nannou::app(model).simple_window(view).run();
}

struct Model {
    sender: osc::Sender<osc::Connected>,
}

fn model(_app: &App) -> Model {
    // The network port that data is being sent to
    let port = 1234;

    // The osc-sender expects a string in the format "address:port", for example "127.0.0.1:1234"
    // "127.0.0.1" is equivalent to your computers internal address.
    let target_addr = format!("{}:{}", "127.0.0.1", port);

    // This is the osc Sender which contains a couple of expectations in case something goes wrong.
    let sender = osc::sender()
        .expect("Could not bind to default socket")
        .connect(target_addr)
        .expect("Could not connect to socket at address");

    Model { sender }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Use app time to progress through a sine wave
    let sine = app.time.sin();
    let slowersine = (app.time / 2.0).sin();

    // Get boundary of the window (to constrain the movements of our circle)
    let boundary = app.window_rect();

    // Map the sine wave functions to ranges between the boundaries of the window
    let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    let y = map_range(slowersine, -1.0, 1.0, boundary.bottom(), boundary.top());

    // Send x-y coordinates as OSC
    let osc_addr = "/circle/position".to_string();
    let args = vec![osc::Type::Float(x), osc::Type::Float(y)];
    let packet = (osc_addr, args);

    model.sender.send(packet).ok();

    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to purple.
    draw.background().color(PLUM);

    // Draw a blue ellipse at the x/y coordinates 0.0, 0.0
    draw.ellipse().color(STEELBLUE).x_y(x, y);

    draw.to_frame(app, &frame).unwrap();
}