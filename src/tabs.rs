use egui_dock::{NodeIndex, Tree};
use crate::tab::Tab;

pub struct Tabs {
    pub tree: Tree<Tab>
}

impl Tabs {
    pub fn new() -> Self {
        let tree = Tree::new(vec![Tab::blank()]);

        Self { tree }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if self.tree.is_empty() {
            self.tree.push_to_focused_leaf(Tab::blank());
        }

        let style = egui_dock::StyleBuilder::from_egui(ui.style().as_ref())
            .show_add_buttons(true).expand_tabs(true);

        let mut added_nodes = Vec::new();

        egui_dock::DockArea::new(&mut self.tree)
            .style(style.build())
            .show_inside(ui, &mut TabViewer {
                added_nodes: &mut added_nodes,
            });

        added_nodes.drain(..).for_each(|node| {
            self.tree.set_focused_node(node);
            self.tree.push_to_focused_leaf(Tab::blank());
        });
    }
}
struct TabViewer<'a> {
    added_nodes: &'a mut Vec<NodeIndex>
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = Tab;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.ui(ui);
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.title().into()
    }

    fn on_add(&mut self, node: NodeIndex) {
        self.added_nodes.push(node);
    }
}
