#[test]
fn leaf_overflow_scrollbars_take_up_space_x_axis() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node = taffy
        .new_leaf_with_measure(
            taffy::style::Style {
                overflow: taffy::geometry::Point {
                    x: taffy::style::Overflow::Scroll,
                    y: taffy::style::Overflow::Visible,
                },
                scrollbar_width: 15f32,
                ..Default::default()
            },
            taffy::tree::MeasureFunc::Raw(|known_dimensions, available_space| {
                const TEXT: &str = "HH";
                super::measure_standard_text(
                    known_dimensions,
                    available_space,
                    TEXT,
                    super::WritingMode::Horizontal,
                    None,
                )
            }),
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node, 20f32, size.width);
    assert_eq!(size.height, 25f32, "height of node {:?}. Expected {}. Actual {}", node, 25f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
}