mod color;
mod julia;
mod window;

const HEIGHT: usize = 800;
const WIDTH: usize = 800;

fn main() {
    let mut julia = julia::Julia::new(-1.6, -1.7, 10, 250.0);
    let mut window = window::Window::new(WIDTH, HEIGHT).unwrap();

    // init window
    julia.compute(&mut window);
    window.update();

    while window.handle_event(&mut julia) {
        julia.compute(&mut window);

        window.update();
    }
}
