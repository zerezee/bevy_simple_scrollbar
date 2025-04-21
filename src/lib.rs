#![doc = include_str!("../README.md")]

pub mod prelude {
    use bevy::prelude::*;
    use log::warn;
    
    #[derive(Default)]
    pub struct SimpleScrollbarPlugin {}
    impl Plugin for SimpleScrollbarPlugin {
        fn build(&self, app: &mut App) {
            app
            .add_observer(drag_scrollbar)
            .add_systems(Update, update_scrollbars);
        }
    }
    
    pub enum ScrollbarDirection {
        Vertical,
        Horizontal
    }
    
    /// Add this component to the thumb of your scrollbar
    #[derive(Component)]
    #[require(Node)]
    pub struct Scrollbar {
        pub direction: ScrollbarDirection,
        pub scroll_area: Entity,
        max_scroll: f32,
    }
    impl Scrollbar {
        pub fn new(direction: ScrollbarDirection, scroll_area: Entity) -> Self {
            Self { direction, scroll_area, max_scroll: 0.0}
        }
    }

    fn drag_scrollbar (
        drag: Trigger<Pointer<Drag>>,
        mut scrollbars: Query<(&mut Node, &ComputedNode, &Scrollbar, &Parent)>, 
        nodes: Query<(&ComputedNode, &Node), Without<Scrollbar>>,
        mut scrolled_nodes: Query<&mut ScrollPosition>,
        window: Query<&Window>, 
    ) {
        let (delta_x, delta_y) = (drag.delta.x, drag.delta.y);
        if let Ok((mut scrollbar_node, computed_scrollbar, scrollbar, parent)) = scrollbars.get_mut(drag.target) {
            let (computed_parent, _) = nodes.get(parent.get()).expect("Parent of the scrollbar must have the node component!");
            let mut scroll_area = scrolled_nodes.get_mut(scrollbar.scroll_area).expect("Scroll area of a scrollbar must be a valid, scrollable node!");

            match scrollbar.direction {
                ScrollbarDirection::Horizontal => {
                    let Val::Px(left) = &mut scrollbar_node.left else { warn!("Horizontal scrollbars must have a Px left value, otherwise they won't work!"); return };
                    *left += delta_x * computed_parent.inverse_scale_factor() * (computed_parent.size().x / window.single().width());

                    let scrollbar_width = computed_scrollbar.size().x;

                    let Val::Px(scrollbar_left) = &mut scrollbar_node.left else { warn!("Horizontal scrollbars must have a Px left value, otherwise they won't work!"); return };
                    *scrollbar_left = scrollbar_left.clamp(0.0, (computed_parent.size().x-scrollbar_width)*computed_parent.inverse_scale_factor()); 

                    scroll_area.offset_x = *scrollbar_left/(computed_parent.size().x*computed_parent.inverse_scale_factor() - computed_scrollbar.size().x*computed_scrollbar.inverse_scale_factor())*scrollbar.max_scroll;
                }
                ScrollbarDirection::Vertical => {
                    let Val::Px(top) = &mut scrollbar_node.top else { warn!("Vertical scrollbars must have a Px top value, otherwise they won't work!"); return };
                    *top += delta_y * computed_parent.inverse_scale_factor() * (computed_parent.size().y / window.single().height());

                    let scrollbar_height = computed_scrollbar.size().y;

                    let Val::Px(scrollbar_top) = &mut scrollbar_node.top else { warn!("Vertical scrollbars must have a Px top value, otherwise they won't work!"); return };
                    *scrollbar_top = scrollbar_top.clamp(0.0, (computed_parent.size().y-scrollbar_height)*computed_parent.inverse_scale_factor());

                    scroll_area.offset_y = *scrollbar_top/(computed_parent.size().y*computed_parent.inverse_scale_factor() - computed_scrollbar.size().y*computed_scrollbar.inverse_scale_factor())*scrollbar.max_scroll;
                }
            }
        }
    }

    fn update_scrollbars(
        mut scrollbars: Query<(&mut Scrollbar, &mut Node, &ComputedNode, &Parent)>, 
        nodes: Query<(&ComputedNode, &Node), Without<Scrollbar>>,
        scroll_areas: Query<(&ScrollPosition, &ComputedNode, &Children), Without<Scrollbar>>,
    ) {
        for (mut scrollbar, mut scrollbar_node, computed_scrollbar, parent) in &mut scrollbars {
            let (scroll_position, computed_scroll_area, children) = scroll_areas.get(scrollbar.scroll_area).expect("Scroll area of a scrollbar must be a valid, scrollable node!");

            let mut max_scroll = 0.0;
            match scrollbar.direction {
                ScrollbarDirection::Horizontal => {
                    for &child in children.iter() {
                        let Ok((computed_child, child_node)) = nodes.get(child) else { continue };

                        let child_margin_left = match child_node.margin.left {
                            Val::Px(value) => value/computed_child.inverse_scale_factor(),
                            _ => 0.0,
                        };
                        let child_margin_right = match child_node.margin.right {
                            Val::Px(value) => value/computed_child.inverse_scale_factor(),
                            _ => 0.0,
                        };
            
                        max_scroll += (computed_child.size().x + computed_child.padding().left + computed_child.padding().right + child_margin_left + child_margin_right)*computed_child.inverse_scale_factor();
                    }
                    max_scroll -= computed_scroll_area.size().x*computed_scroll_area.inverse_scale_factor();
                }
                ScrollbarDirection::Vertical => {
                    for &child in children.iter() {
                        let Ok((computed_child, child_node)) = nodes.get(child) else { continue };
        
                        let child_margin_top = match child_node.margin.top {
                            Val::Px(value) => value/computed_child.inverse_scale_factor(),
                            _ => 0.0,
                        };
                        let child_margin_bottom = match child_node.margin.bottom {
                            Val::Px(value) => value/computed_child.inverse_scale_factor(),
                            _ => 0.0,
                        };
            
                        max_scroll += (computed_child.size().y + computed_child.padding().top + computed_child.padding().bottom + child_margin_top + child_margin_bottom)*computed_child.inverse_scale_factor();
                    }
                    max_scroll -= computed_scroll_area.size().y*computed_scroll_area.inverse_scale_factor();
                }
            }
            scrollbar.max_scroll = max_scroll;

            let (computed_parent, _) = nodes.get(parent.get()).expect("Parent of the scrollbar must have the node component!");
            match scrollbar.direction {
                ScrollbarDirection::Horizontal => {
                    let thumb_size = (computed_scroll_area.size().x / (max_scroll + computed_scroll_area.size().x) * computed_scroll_area.inverse_scale_factor()).clamp(0.05, 1.0);
                    let Val::Percent(scrollbar_width) = &mut scrollbar_node.width else { warn!("Horizontal scrollbars must have a Percent width value, otherwise they won't work!"); return };
                    *scrollbar_width = thumb_size*100.0;

                    let scroll_percent = scroll_position.offset_x / max_scroll;
                    let scrollbar_width = computed_scrollbar.size().x;
                    let Val::Px(scrollbar_left) = &mut scrollbar_node.left else { warn!("Vertical scrollbars must have a Px top value, otherwise they won't work!"); return };
                    *scrollbar_left = (computed_parent.size().x-scrollbar_width)*computed_parent.inverse_scale_factor() * scroll_percent;
                }
                ScrollbarDirection::Vertical => {
                    let thumb_size = (computed_scroll_area.size().y / (max_scroll + computed_scroll_area.size().y) * computed_scroll_area.inverse_scale_factor()).clamp(0.05, 1.0);
                    let Val::Percent(scrollbar_height) = &mut scrollbar_node.height else { warn!("Vertical scrollbars must have a Percent height value, otherwise they won't work!"); return };
                    *scrollbar_height = thumb_size*100.0;

                    let scroll_percent = scroll_position.offset_y / max_scroll;
                    let scrollbar_height = computed_scrollbar.size().y;
                    let Val::Px(scrollbar_top) = &mut scrollbar_node.top else { warn!("Vertical scrollbars must have a Px top value, otherwise they won't work!"); return };
                    *scrollbar_top = (computed_parent.size().y-scrollbar_height)*computed_parent.inverse_scale_factor() * scroll_percent;
                }
            }
        }
    }
}
