// Produce an image of an arbitrary-arity tree
// Michael Xilinas
// 2023-08-09

#![allow(dead_code)]

use draw::*;
use std::f32::INFINITY;

// DATA DEFINITIONS

/// interp. The bounding box of a drawing
/// - top is the smallest y value.
/// - left is the smallest x value.
pub struct Bounds {
    pub top: f32,
    pub bottom: f32,
    pub right: f32,
    pub left: f32,
}

/// interp. A node in an arbitrary-arity tree.
/// Each node can have an arbitrary number of sub-nodes.
pub struct Node {
    pub subs: Vec<Node>,
}

impl Clone for Node {
    /// Return a clone of this node.
    fn clone(&self) -> Node {
        fn fn_for_node(n: &Node) -> Node {
            return Node {
                subs: fn_for_lon(&n.subs),
            };
        }

        fn fn_for_lon(lon: &Vec<Node>) -> Vec<Node> {
            let mut subs = vec![];
            for n in lon {
                let result = fn_for_node(n);
                subs.push(result);
            }

            return subs;
        }

        fn_for_node(self)
    }
}

// FUNCTIONS

/// Return a black square at (0,0).
pub fn square(side_len: u32) -> Drawing {
    let width = side_len;
    let height = side_len;
    let shape = Shape::Rectangle { width, height };
    let style = Style::filled(Color::black());
    let rect = Drawing::new().with_shape(shape).with_style(style);

    return rect;
}

/// Move a drawing and its subs by some x and y.
pub fn move_with_subs(mut d: Drawing, x: f32, y: f32) -> Drawing {
    fn fn_for_d(d: &mut Drawing, x: f32, y: f32) {
        d.position.x += x;
        d.position.y += y;

        fn_for_lod(&mut d.display_list.drawings, x, y)
    }

    fn fn_for_lod(lod: &mut Vec<Drawing>, x: f32, y: f32) {
        for d in lod {
            fn_for_d(d, x, y)
        }
    }

    fn_for_d(&mut d, x, y);
    return d;
}

/// Produce the bounding box of a drawing and its subs.
pub fn get_bounds(d: &Drawing) -> Bounds {
    // d_wl is the primary worklist.
    // top is the smallest y so far.
    // bottom is the largest y so far.
    // right is the largest x so far.
    // left is the smallest x so far.
    fn fn_for_d<'a>(
        d: &'a Drawing,
        mut d_wl: Vec<&'a Drawing>,
        mut top: f32,
        mut bottom: f32,
        mut right: f32,
        mut left: f32,
    ) -> Bounds {
        let y = d.position.y;
        if y < top {
            top = y;
        }
        if y > bottom {
            bottom = y;
        }

        let x = d.position.x;
        if x > right {
            right = x;
        }
        if x < left {
            left = x;
        }

        // Update the worklist
        let subs = &d.display_list.drawings;
        for sub in subs {
            d_wl.push(sub);
        }

        return fn_for_lod(d_wl, top, bottom, right, left);
    }

    fn fn_for_lod(mut d_wl: Vec<&Drawing>, top: f32, bottom: f32, right: f32, left: f32) -> Bounds {
        if d_wl.is_empty() {
            return Bounds {
                top,
                bottom,
                right,
                left,
            };
        }

        let last = d_wl.pop().unwrap();
        return fn_for_d(last, d_wl, top, bottom, right, left);
    }

    return fn_for_d(d, vec![], INFINITY, -INFINITY, -INFINITY, INFINITY);
}

/// Move d1 above d0 and parent d1 to d0
/// depth is the depth of subs to reparent
pub fn above(mut d0: Drawing, mut d1: Drawing, offset: f32, depth: u8) -> Drawing {
    if let None = d1.shape {
        return d0;
    }
    if let None = d0.shape {
        return d1;
    }

    let top_d0 = get_bounds(&d0).top;
    let bottom_d1 = get_bounds(&d1).bottom;

    let center_x_d0 = get_center(&d0).0;
    let center_x_d1 = get_center(&d1).0;

    let target_x = center_x_d0 - center_x_d1;
    let target_y = bottom_d1 + top_d0 - offset;

    d1 = move_with_subs(d1, target_x, target_y);

    d1 = inherit(d1, &mut d0, depth);

    d1.display_list.add(d0);

    return d1;
}

/// Transfer all the subs of d1 to d0 and return d0.
fn inherit(mut d0: Drawing, d1: &mut Drawing, depth: u8) -> Drawing {
    if d1.display_list.drawings.is_empty() || depth < 1 {
        return d0;
    }

    let last = d1.display_list.drawings.pop().unwrap();
    d0.display_list.add(last);

    return inherit(d0, d1, depth - 1);
}

/// Move d1 beside d0 and parent d1 to d0
fn beside(mut d0: Drawing, mut d1: Drawing, offset: f32) -> Drawing {
    if let None = d1.shape {
        return d0;
    }
    if let None = d0.shape {
        return d1;
    }

    let right_d0 = get_bounds(&d0).right;
    let left_d1 = get_bounds(&d1).left;

    let center_y_d0 = get_center(&d0).1;
    let center_y_d1 = get_center(&d1).1;

    let target_x = right_d0 - left_d1 + offset;
    let target_y = center_y_d0 - center_y_d1;

    d1 = move_with_subs(d1, target_x, target_y);

    d0.display_list.add(d1);

    return d0;
}

/// Move d1 beside d0 and parent d1 to d0
pub fn beside_align_top(mut d0: Drawing, mut d1: Drawing, offset: f32) -> Drawing {
    if let None = d1.shape {
        return d0;
    }
    if let None = d0.shape {
        return d1;
    }

    let d0_bounds = get_bounds(&d0);
    let d1_bounds = get_bounds(&d1);

    let target_x = d0_bounds.right - d1_bounds.left + offset;
    let target_y = d0_bounds.top - d1_bounds.top;

    d1 = move_with_subs(d1, target_x, target_y);

    d0.display_list.add(d1);

    return d0;
}

/// Return the center (x,y) of a drawing.
fn get_center(d: &Drawing) -> (f32, f32) {
    let bounds = get_bounds(d);

    let cntr_x = (bounds.right + bounds.left) / 2.0;
    let cntr_y = (bounds.bottom + bounds.top) / 2.0;

    return (cntr_x, cntr_y);
}

/// Move a drawing to the center of the canvas.
pub fn center(mut d: Drawing, width: f32, height: f32, side_len: f32) -> Drawing {
    let cntr_x = width / 2.0 - side_len / 2.0;
    let cntr_y = height / 2.0 - side_len / 2.0;

    let d_center = get_center(&d);

    let target_x = cntr_x - d_center.0;
    let target_y = cntr_y - d_center.1;

    d = move_with_subs(d, target_x, target_y);

    return d;
}

/// Add a black line to the canvas from start to end.
pub fn draw_line(start: (f32, f32), end: (f32, f32)) -> Drawing {
    let mut lines = LineBuilder::new(start.0, start.1);
    lines = lines.line_to(end.0, end.1);
    let shape = lines.build();

    let d = Drawing::new()
        .with_style(Style::stroked(5, Color::black()))
        .with_shape(shape);

    return d;
}

/// Produce a drawing of the lines connecting the nodes on a given tree.
pub fn draw_lines(d: &Drawing, offset: f32) -> Drawing {
    // Draws a lines from the given node to each of its children
    fn fn_for_d(d: &Drawing, offset: f32) -> Drawing {
        let mut output = Drawing::new();
        for sub in &d.display_list.drawings {
            let start = (d.position.x + offset, d.position.y + offset * 2.0);
            let end = (sub.position.x + offset, sub.position.y);
            output.display_list.add(draw_line(start, end));
        }

        let result = fn_for_lod(&d.display_list.drawings, offset);
        output.display_list.add(result);

        return output;
    }

    // Returns the lines connecting a list of nodes and their children
    fn fn_for_lod(lod: &Vec<Drawing>, offset: f32) -> Drawing {
        let mut output = Drawing::new();

        for d in lod {
            let result = fn_for_d(d, offset);
            output.display_list.add(result);
        }

        return output;
    }

    return fn_for_d(&d, offset);
}

/// Produce an image of the given tree
pub fn draw_nodes(n: Node, side_len: u32, x_offset: f32, y_offset: f32) -> Drawing {
    // Takes a node and ruturns a drawing of its subs.
    fn fn_for_node(node: Node, side_len: u32, x_offset: f32, y_offset: f32) -> Drawing {
        let mut depth = node.subs.len() as u8;
        if node.subs.len() > 0 {
            depth -= 1
        }

        let mut rect = square(side_len);
        if node.subs.len() == 0 {
            rect.style = Style::filled(RGB::new(128, 0, 0));
        }

        let tree = fn_for_lon(node.subs, side_len, x_offset, y_offset);

        return above(tree, rect, y_offset, depth);
    }

    // Returns a drawing of a list of nodes' trees beside one another.
    fn fn_for_lon(lon: Vec<Node>, side_len: u32, x_offset: f32, y_offset: f32) -> Drawing {
        let mut output = Drawing::new();
        for node in lon {
            let result = fn_for_node(node, side_len, x_offset, y_offset);
            output = beside_align_top(output, result, x_offset);
        }

        return output;
    }

    return fn_for_node(n, side_len, x_offset, y_offset);
}
