#[test]
fn block_absolute_margin_auto_left_fix_right_child_bigger_than_parent_with_inset() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            position: taffy::style::Position::Absolute,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(72f32),
                height: taffy::style::Dimension::Length(72f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Auto,
                right: taffy::style::LengthPercentageAuto::Length(10f32),
                top: zero(),
                bottom: zero(),
            },
            inset: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(10f32),
                right: taffy::style::LengthPercentageAuto::Length(20f32),
                top: auto(),
                bottom: auto(),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Block,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(52f32),
                    height: taffy::style::Dimension::Length(52f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 52f32, "width of node {:?}. Expected {}. Actual {}", node, 52f32, size.width);
    assert_eq!(size.height, 52f32, "height of node {:?}. Expected {}. Actual {}", node, 52f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 72f32, "width of node {:?}. Expected {}. Actual {}", node0, 72f32, size.width);
    assert_eq!(size.height, 72f32, "height of node {:?}. Expected {}. Actual {}", node0, 72f32, size.height);
    assert_eq!(location.x, -50f32, "x of node {:?}. Expected {}. Actual {}", node0, -50f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
}
