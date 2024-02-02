use render_tree::*;
use draw::*;

// CONSTANTS
const WIDTH: u32 = 1500;
const HEIGHT: u32 = 1500;
const SIDE_LEN: f32 = 50.0;

// FUNCTIONS
fn main() {
    let t0 = Node {
        subs: vec!(
            Node { subs: vec!() },
            Node { subs: vec!(
                Node { subs: vec!() },
                Node { subs: vec!(
                    Node { subs: vec!() },
                    Node { subs: vec!() },
                    Node { subs: vec!() },
                ) },
                Node { subs: vec!() },
            ) },
            Node { subs: vec!(
                Node { subs: vec!() },
                Node { subs: vec!() },
                Node { subs: vec!(
                    Node { subs: vec!() },
                    Node { subs: vec!() },
                    Node { subs: vec!() },
                    Node { subs: vec!() },
                ) },
            ) },
        )
    };

    let mut nodes = draw_nodes(t0, SIDE_LEN as u32, 100.0, 100.0);
    nodes = center(nodes, WIDTH as f32, HEIGHT as f32);

    let lines = draw_lines(&nodes, SIDE_LEN / 2.0);

    let mut canvas = Canvas::new(WIDTH as u32, HEIGHT as u32); 

    canvas.display_list.add(lines);
    canvas.display_list.add(nodes);

    render::save(&canvas, "./output.svg", SvgRenderer::new())
        .expect("Failed to save");
}


