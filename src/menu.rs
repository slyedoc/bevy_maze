use bevy::prelude::*;
use bevy::prelude::Vec2 as Vec2;
use bevy::app::AppExit;
use bevy_egui::*;
use bevy_egui::egui::*;

use crate::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            //.add_startup_system_to_stage(StartupStage::PreStartup, spawn_cells.system())
            .init_resource::<MenuUIData>()
            .add_system_set(
                SystemSet::on_update(AppState::Menu).with_system(draw_menu.system())
            );

        // ACTION HANDLING
        // .add_system_set(
        //     SystemSet::new()
        //         .after(CommonLabels::Action)
        //         .with_system(actions::color_selected.system())
        //         .with_system(actions::update_cell_numbers.system())
        //         .with_system(actions::style_numbers.system()),
        // );
    }
}

#[derive(Default)]
struct MenuUIData {
    label: String,
    value: f32,
    settings: bool,
    inspection: bool,
    style: bool,

}

fn draw_menu(egui_ctx: Res<EguiContext>,
    mut data: ResMut<MenuUIData>,
    mut exit: EventWriter<AppExit>
) {

    egui::TopPanel::top("top_panel").show(egui_ctx.ctx(), |ui| {
        // The top panel is often a good place for a menu bar:
        egui::menu::bar(ui, |ui| {
            egui::menu::menu(ui, "File", |ui| {
                if ui.button("Quit").clicked() {
                    exit.send(AppExit);
                }
            });
        });
    });


    egui::SidePanel::left("side_panel", 200.0).show(egui_ctx.ctx(), |ui| {

        ui.add_space(50.0);

        
        ui.heading("Bevy Maze");

        ui.with_layout(egui::Layout::top_down_justified(Align::Center), |ui| {
            if ui.button("New Game").clicked() {
                data.value += 1.0;
            }
            ui.add(egui::Slider::new(&mut data.value, 0.0..=10.0).text("value"));

        });

        ui.add(egui::Slider::new(&mut data.value, 0.0..=10.0).text("value"));
        if ui.button("Increment").clicked() {
            data.value += 1.0;
        }

        ui.checkbox(&mut data.settings, "🔧 Settings");
        ui.checkbox(&mut data.inspection, "🔧 Inspection");

        // CollapsingHeader::new("Resize")
        // .default_open(false)
        // .show(ui, |ui| {
        //     Resize::default().default_height(100.0).show(ui, |ui| {
        //         ui.label("This ui can be resized!");
        //         ui.label("Just pull the handle on the bottom right");
        //     });
        // });

        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {

            ui.add(
                egui::Hyperlink::new("https://github.com/emilk/egui/").text("powered by egui"),
            );
        });




    });


    egui::Window::new("🔧 Settings")
    .open(&mut data.settings)
    .scroll(true)
    .show(egui_ctx.ctx(), |ui| {
        egui_ctx.ctx().settings_ui(ui);
    });


    egui::Window::new("🔧 Inspection")
    .open(&mut data.inspection)
    .scroll(true)
    .show(egui_ctx.ctx(), |ui| {
        egui_ctx.ctx().inspection_ui(ui);
    });

}
