#[test]
fn block_margin_y_first_child_collapse_positive_and_negative() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node000 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
            ..Default::default()
        })
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Block,
                margin: taffy::geometry::Rect {
                    left: zero(),
                    right: zero(),
                    top: taffy::style::LengthPercentageAuto::Length(10f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node000],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Block,
                margin: taffy::geometry::Rect {
                    left: zero(),
                    right: zero(),
                    top: taffy::style::LengthPercentageAuto::Length(-10f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node100 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
            ..Default::default()
        })
        .unwrap();
    let node10 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Block,
                margin: taffy::geometry::Rect {
                    left: zero(),
                    right: zero(),
                    top: taffy::style::LengthPercentageAuto::Length(5f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node100],
        )
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Block,
                margin: taffy::geometry::Rect {
                    left: zero(),
                    right: zero(),
                    top: taffy::style::LengthPercentageAuto::Length(-10f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node10],
        )
        .unwrap();
    let node200 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
            ..Default::default()
        })
        .unwrap();
    let node20 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Block,
                margin: taffy::geometry::Rect {
                    left: zero(),
                    right: zero(),
                    top: taffy::style::LengthPercentageAuto::Length(10f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node200],
        )
        .unwrap();
    let node2 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Block,
                margin: taffy::geometry::Rect {
                    left: zero(),
                    right: zero(),
                    top: taffy::style::LengthPercentageAuto::Length(-5f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            &[node20],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Block,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Length(50f32), height: auto() },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node, 50f32, size.width);
    assert_eq!(size.height, 30f32, "height of node {:?}. Expected {}. Actual {}", node, 30f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node0, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node00, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node00, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node000).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node000, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node000, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node000, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node000, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node1, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node1, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node1, 5f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node10).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node10, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node10, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node10, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node10, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node100).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node100, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node100, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node100, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node100, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node2, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node2, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2, 0f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node2, 20f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node20).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node20, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node20, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node20, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node20, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node200).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node200, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node200, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node200, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node200, 0f32, location.y);
}
