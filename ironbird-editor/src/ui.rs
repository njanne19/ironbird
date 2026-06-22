use std::collections::HashMap;
use bevy::{
    camera::{CameraOutputMode, RenderTarget, Viewport, visibility::RenderLayers}, log::tracing_subscriber::fmt, prelude::*, window::PrimaryWindow
};
use bevy::render::render_resource::{
    BlendState,
    Extent3d,
    TextureDescriptor,
    TextureDimension,
    TextureFormat,
    TextureUsages,
};
use bevy_egui::{
    EguiContext, EguiContexts, EguiGlobalSettings, EguiPrimaryContextPass, EguiTextureHandle, PrimaryEguiContext, egui
};
use egui::{LayerId, Ui, UiBuilder};
use egui_tiles::{Tiles, Tree, Behavior, Linear, LinearDir};
use serde::{Serialize, Deserialize};


#[derive(Reflect, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ViewportTargetId(usize);
impl std::fmt::Display for ViewportTargetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
const EDITOR_MAIN_VIEWPORT_ID: ViewportTargetId = ViewportTargetId(0);

// The actual different types of panes that are available
#[derive(Reflect, Serialize, Deserialize, Debug, Clone, Default)]
enum EditorUiPane {
    #[default]
    Blank,
    Viewport(ViewportTargetId),
}

#[derive(Default)]
struct EditorUiOutput {
    visible_viewports: Vec<VisibleViewport>,
}

struct VisibleViewport {
    target_id: ViewportTargetId,
    rect: egui::Rect,
}

// A context struct that allows panes to both 
// read from editor state, and make writes 
// to an output field that get propagated to state
struct EditorUiPaneContext<'a> {
    viewport_textures: &'a HashMap<ViewportTargetId, egui::TextureId>,
    output: &'a mut EditorUiOutput,
}

impl Behavior<EditorUiPane> for EditorUiPaneContext<'_> {
    fn tab_title_for_pane(&mut self, pane: &EditorUiPane) -> egui::WidgetText {
        match pane {
            EditorUiPane::Blank => "Empty".into(),
            EditorUiPane::Viewport(id) => {
                format!("Viewport {}", id).into()
            },
        }
    }

    fn pane_ui(
        &mut self,
        ui: &mut Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut EditorUiPane,
    ) -> egui_tiles::UiResponse {
        match pane {
            EditorUiPane::Viewport(target_id) => {
                let rect = ui.available_rect_before_wrap();
                if let Some(texture_id) = self.viewport_textures.get(target_id) {
                    let _response = ui.add(
                        egui::Image::new((*texture_id, rect.size()))
                            .sense(egui::Sense::click_and_drag())
                    );
                } else {
                    ui.label("Missing viewport texture");
                }
            },
            EditorUiPane::Blank => {
                ui.label("Empty");
            }
        }
        egui_tiles::UiResponse::None
    }

    fn simplification_options(&self) -> egui_tiles::SimplificationOptions {
        egui_tiles::SimplificationOptions {
            all_panes_must_have_tabs: true,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
struct ViewportTarget {
    // Stores the texture this target displays
    // and a reference to the camera that renders it
    pub texture_handle: Handle<Image>,
    pub camera_entity: Entity,
}

// Editor state that can be serialized. Saved 
// and used again in another session. 
#[derive(Reflect, Serialize, Deserialize, Debug, Clone)]
#[reflect(Serialize, Deserialize)]
struct PersistentState {}

// Editor state unique to this session. Cannot 
// be serialized nor used again.
#[derive(Debug, Clone)]
struct RuntimeState {
    panel_tree: Tree<EditorUiPane>,
    viewport_targets: HashMap<ViewportTargetId, ViewportTarget>,
}

#[derive(Resource, Debug, Clone)]
struct EditorState {
    persistent: PersistentState,
    runtime: RuntimeState,
}

pub struct EditorUiPlugin;

impl Plugin for EditorUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, editor_init_system);
        app.add_systems(Startup, basic_objects.spawn());
        app.add_systems(EguiPrimaryContextPass, editor_main_loop_system);
    }
}

fn editor_main_loop_system(
    mut egui_contexts: EguiContexts,
    mut state: ResMut<EditorState>,
    mut camera: Single<&mut Camera, Without<EguiContext>>,
    window: Single<&mut Window, With<PrimaryWindow>>,
) -> Result {

    let _span = info_span!("IBD UI System").entered();

    let egui_ctx = egui_contexts.ctx_mut()?;
    let mut root_ui = Ui::new(
        egui_ctx.clone(),
        "root".into(),
        UiBuilder::new()
            .layer_id(LayerId::background())
            .max_rect(egui_ctx.viewport_rect()),
    );

    // Register viewport image textures with egui
    let viewport_textures: HashMap<ViewportTargetId, egui::TextureId> = state.runtime.viewport_targets
            .iter()
            .map(|(target_id, target)|{
                let texture_id = match egui_contexts.image_id(&target.texture_handle) {
                    Some(id) => id,
                    None => {
                        egui_contexts.add_image(EguiTextureHandle::Weak(
                                AssetId::<Image>::from(&target.texture_handle)
                        ))
                    }
                };
                (target_id.clone(), texture_id)
            })
            .collect();

    let mut ui_output = EditorUiOutput::default();
    egui::CentralPanel::default().show_inside(&mut root_ui, |ui| {
        let tree = &mut state.runtime.panel_tree;
        let mut pane_context = EditorUiPaneContext {
            viewport_textures: &viewport_textures,
            output: &mut ui_output,
        };
        tree.ui(&mut pane_context, ui);
    });

    Ok(())
}

fn editor_init_system(
    mut commands: Commands,
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
    mut images: ResMut<Assets<Image>>,
) {
    // TODO: at some point, we should split this 
    // up into a loaded config by the user, 
    // and whatever sensible defaults look like.

    // Disable the automatic creation of a primary context to 
    // set it up manually for the camera we need.
    egui_global_settings.auto_create_primary_context = false;

    // Egui camera
    commands.spawn((
        // The 'PrimaryEguiContext' component requires everything needed to 
        // render a primary context
        PrimaryEguiContext,
        Camera2d,
        // Setting RenderLayers to none makes sure we won't render anything apart from the UI.
        RenderLayers::none(),
        Camera {
            order: 1,
            output_mode: CameraOutputMode::Write {
                blend_state: Some(BlendState::ALPHA_BLENDING),
                clear_color: ClearColorConfig::None,
            },
            clear_color: ClearColorConfig::Custom(Color::NONE),
            ..default()
        },
    ));

    // Default viewport camera. Write 
    // to a texture that we can use later when we display with egui
    let main_viewport_size = Extent3d {
        width: 512,
        height: 512,
        depth_or_array_layers: 1,
    };
    let mut main_viewport_image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("main_viewport_image"),
            size: main_viewport_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    main_viewport_image.resize(main_viewport_size);
    let main_viewport_image_handle = images.add(main_viewport_image);

    let main_viewport_camera = commands.spawn((
        Camera3d::default(),
        Camera::default(),
        RenderTarget::from(main_viewport_image_handle.clone()),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    )).id();


    // Set of default splits, just some basics
    let mut tiles = Tiles::<EditorUiPane>::default();

    let scene_heirarchy_tab = tiles.insert_pane(EditorUiPane::Blank);
    let main_viewport_tab = tiles.insert_pane(EditorUiPane::Viewport(EDITOR_MAIN_VIEWPORT_ID));
    let properties_tab = tiles.insert_pane(EditorUiPane::Blank);

    let left = tiles.insert_tab_tile(vec![scene_heirarchy_tab]);
    let center = tiles.insert_tab_tile(vec![main_viewport_tab]);
    let right = tiles.insert_tab_tile(vec![properties_tab]);

    let mut row = Linear::new(
        LinearDir::Horizontal,
        vec![left, center, right],
    );

    row.shares.set_share(left, 1.0);
    row.shares.set_share(center, 3.0);
    row.shares.set_share(right, 1.0);

    let root = tiles.insert_container(row);

    // Initialized viewport target 0 to be the 
    // editor viewport.
    commands.insert_resource(EditorState {
        persistent: PersistentState{},
        runtime: RuntimeState {
            panel_tree: Tree::new("main_tree", root, tiles),
            viewport_targets: HashMap::from([
                (EDITOR_MAIN_VIEWPORT_ID, ViewportTarget{
                    texture_handle: main_viewport_image_handle,
                    camera_entity: main_viewport_camera,
                })
            ])
        }
    });

}

fn basic_objects(
) -> impl SceneList {
    bsn_list! [
        (
            #CircularBase
            Mesh3d(asset_value(Circle::new(4.0)))
            MeshMaterial3d::<StandardMaterial>(asset_value(Color::WHITE))
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
        ),
        (
            #Cube
            Mesh3d(asset_value(Cuboid::new(1.0, 1.0, 1.0)))
            MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb_u8(124, 144, 255)))
            Transform::from_xyz(0.0, 0.5, 0.0)
        ),
        (
            PointLight {
                shadow_maps_enabled: true,
            }
            Transform::from_xyz(4.0, 8.0, 4.0)
        ),
    ]
}
