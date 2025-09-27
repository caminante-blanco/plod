use crate::colors::*;
use crate::dev_icons;
use egui::{Label, RichText, Ui};
use egui_ltreeview::{NodeBuilder, TreeView, TreeViewBuilder, CloserState};
use std::ops::{Deref, DerefMut};

pub struct DevIconTreeBuilder<'a, 'b> {
    builder: &'a mut TreeViewBuilder<'b, usize>,
    depth: usize,
}

impl<'a, 'b> Deref for DevIconTreeBuilder<'a, 'b> {
    type Target = TreeViewBuilder<'b, usize>;
    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}

impl<'a, 'b> DerefMut for DevIconTreeBuilder<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.builder
    }
}

impl<'a, 'b> DevIconTreeBuilder<'a, 'b> {
    pub fn leaf(&mut self, id: usize, label: &str) {
        let icon_info = dev_icons::get_icon_for_file(label);
        let node = NodeBuilder::leaf(id)
            .label(RichText::new(label).color(icon_info.color))
            .icon(|ui: &mut Ui| {
                ui.add(Label::new(RichText::new(icon_info.icon).color(icon_info.color)).selectable(false));
            });
        self.builder.node(node);
    }

    pub fn dir(&mut self, id: usize, label: &str) {
        let is_top_level = self.depth == 0;
        self.depth += 1;

        let node = NodeBuilder::dir(id)
            .default_open(is_top_level)
            .label(RichText::new(label).color(GREEN))
            .closer(|ui: &mut Ui, closer_state: CloserState| {
                let icon_info = dev_icons::get_icon_for_folder(closer_state.is_open);
                let closer_icon = if closer_state.is_open { " " } else { " " };
                ui.add(Label::new(RichText::new(format!("{closer_icon}{}", icon_info.icon)).color(icon_info.color)).selectable(false));
            });
        self.builder.node(node);
    }

    // close_dir is handled by DerefMut
}

pub fn show<F>(
    ui: &mut egui::Ui,
    id: egui::Id,
    add_contents: F,
)
where
    for<'a, 'b> F: FnOnce(&mut DevIconTreeBuilder<'a, 'b>),
{
    TreeView::new(id).show(ui, |builder| {
        let mut dev_icon_builder = DevIconTreeBuilder { builder, depth: 0 };
        add_contents(&mut dev_icon_builder);
    });
}

