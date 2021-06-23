use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::egui::*;
use bevy_egui::*;

use crate::maze::MazeSize;
use crate::grid::GridState;
use crate::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            //.add_startup_system_to_stage(StartupStage::PreStartup, spawn_cells.system())
            .init_resource::<MenuUIData>()
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(draw_menu.system()));

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
    settings: bool,
    inspection: bool,
}

fn draw_menu(
    mut commands: Commands,
    egui_ctx: Res<EguiContext>,
    mut data: ResMut<MenuUIData>,
    mut exit: EventWriter<AppExit>,
    mut size: ResMut<MazeSize>,
    mut app_state: ResMut<State<AppState>>,
    mut grid_state: ResMut<State<GridState>>,
) {
    // egui::TopPanel::top("top_panel").show(egui_ctx.ctx(), |ui| {
    //     // The top panel is often a good place for a menu bar:
    //     egui::menu::bar(ui, |ui| {
    //         egui::menu::menu(ui, "File", |ui| {
    //             if ui.button("Quit").clicked() {
    //                 exit.send(AppExit);
    //             }
    //         });
    //     });
    // });

    egui::SidePanel::left("side_panel", 200.0).show(egui_ctx.ctx(), |ui| {
        ui.style_mut().spacing.item_spacing.y = 10.0;
        ui.add_space(30.0);
        ui.heading("Bevy Maze");
        ui.add_space(30.0);
        let mut x = size.x;
        let mut y = size.y;
        CollapsingHeader::new("Settings")
            .default_open(true)
            .show(ui, |ui| {
                ui.add(egui::Slider::new(&mut x, 10..=100).text("Width"));
                ui.add(egui::Slider::new(&mut y, 10..=100).text("Height"));
            });

        if x != size.x || y != size.y {
            size.x = x;
            size.y = y;
            grid_state.set(crate::grid::GridState::Reset).unwrap();
        }
        ui.add_space(30.0);

        ui.with_layout(egui::Layout::top_down_justified(Align::Center), |ui| {
            if ui.button("New Game").clicked() {
                app_state.set(AppState::Playing).unwrap();
            }

            if ui.button("Settings").clicked() {
                data.settings = !data.settings;
            }

            if ui.button("Quit").clicked() {
                if ui.button("Quit").clicked() {
                    exit.send(AppExit);
                }
            }
        });

        // CollapsingHeader::new("Resize")
        // .default_open(false)
        // .show(ui, |ui| {
        //     Resize::default().default_height(100.0).show(ui, |ui| {
        //         ui.label("This ui can be resized!");
        //         ui.label("Just pull the handle on the bottom right");
        //     });
        // });

        // ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {

        //     ui.add(
        //         egui::Hyperlink::new("https://github.com/emilk/egui/").text("powered by egui"),
        //     );
        // });
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
