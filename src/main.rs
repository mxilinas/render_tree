use draw::*;
use render_tree::*;

// CONSTANTS
const HEIGHT: u32 = 512;
const WIDTH: u32 = 512;
const SIDE_LEN: f32 = 20.0;
const X_OFFSET: f32 = 25.0;
const Y_OFFSET: f32 = 50.0;

// FUNCTIONS
fn main() {
    let t0 = Node {
        subs: vec![
            Node { subs: vec![] },
            Node {
                subs: vec![
                    Node { subs: vec![] },
                    Node {
                        subs: vec![
                            Node { subs: vec![] },
                            Node { subs: vec![] },
                            Node { subs: vec![] },
                        ],
                    },
                    Node { subs: vec![] },
                ],
            },
            Node {
                subs: vec![
                    Node {
                        subs: vec![Node {
                            subs: vec![
                                Node { subs: vec![] },
                                Node { subs: vec![] },
                                Node {
                                    subs: vec![
                                        Node {
                                            subs: vec![Node {
                                                subs: vec![
                                                    Node { subs: vec![] },
                                                    Node { subs: vec![] },
                                                    Node { subs: vec![] },
                                                    Node { subs: vec![] },
                                                ],
                                            }],
                                        },
                                        Node { subs: vec![] },
                                        Node { subs: vec![] },
                                        Node { subs: vec![] },
                                    ],
                                },
                            ],
                        }],
                    },
                    Node { subs: vec![] },
                    Node {
                        subs: vec![
                            Node { subs: vec![] },
                            Node { subs: vec![] },
                            Node { subs: vec![] },
                            Node { subs: vec![] },
                        ],
                    },
                ],
            },
        ],
    };

    let mut nodes = draw_nodes(t0, SIDE_LEN as u32, X_OFFSET, Y_OFFSET);
    nodes = center(nodes, WIDTH as f32, HEIGHT as f32, SIDE_LEN);

    let lines = draw_lines(&nodes, SIDE_LEN / 2.0);

    let mut canvas = Canvas::new(WIDTH as u32, HEIGHT as u32);

    canvas.display_list.add(lines);
    canvas.display_list.add(nodes);

    render::save(&canvas, "./output.svg", SvgRenderer::new()).expect("Failed to save");
}
