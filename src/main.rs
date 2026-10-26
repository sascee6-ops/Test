#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[global_allocator]
static GLOBAL_ALLOCATOR: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    bevy::app::App::new()
        .add_plugins((
            bevy::app::TaskPoolPlugin::default(),
            bevy::diagnostic::FrameCountPlugin,
            bevy::time::TimePlugin,
            bevy::transform::TransformPlugin,
            bevy::input::InputPlugin,
            bevy::a11y::AccessibilityPlugin,
            bevy::window::WindowPlugin {
                primary_window: std::option::Option::Some(bevy::window::Window {
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    mode: bevy::window::WindowMode::BorderlessFullscreen(
                        bevy::window::MonitorSelection::Current,
                    ),
                    position: bevy::window::WindowPosition::Automatic,
                    resolution: bevy::window::WindowResolution::new(1920, 1080),
                    title: std::string::String::from(if cfg!(target_os = "linux") {
                        "Subsystem for Arxumbra"
                    } else if cfg!(target_os = "macos") {
                        "Subsystem for Arxumbra"
                    } else {
                        "Subsystem for Arxumbra"
                    }),
                    name: std::option::Option::Some(std::string::String::from(
                        "Subsystem-for-Arxumbra",
                    )),
                    resize_constraints: bevy::window::WindowResizeConstraints {
                        min_width: 640.0,
                        min_height: 400.0,
                        ..bevy::window::WindowResizeConstraints::default()
                    },
                    resizable: false,
                    decorations: false,
                    transparent: false,
                    focused: true,
                    skip_taskbar: false,
                    desired_maximum_frame_latency: std::option::Option::None,
                    ..bevy::window::Window::default()
                }),
                primary_cursor_options: std::option::Option::Some(bevy::window::CursorOptions {
                    visible: false,
                    grab_mode: bevy::window::CursorGrabMode::None,
                    hit_test: true,
                }),
                ..bevy::window::WindowPlugin::default()
            },
            bevy::asset::AssetPlugin::default(),
            bevy::winit::WinitPlugin {
                run_on_any_thread: false,
            },
        ))
        .add_plugins((
            bevy::render::RenderPlugin {
                render_creation: bevy::render::settings::RenderCreation::Automatic(
                    std::boxed::Box::new(bevy::render::settings::WgpuSettings {
                        backends: std::option::Option::Some(
                            bevy::render::settings::Backends::VULKAN
                                | bevy::render::settings::Backends::METAL
                                | bevy::render::settings::Backends::DX12,
                        ),
                        power_preference: bevy::render::settings::PowerPreference::HighPerformance,
                        priority: bevy::render::settings::WgpuSettingsPriority::Functionality,
                        instance_flags: bevy::render::settings::InstanceFlags::empty(),
                        ..bevy::render::settings::WgpuSettings::default()
                    }),
                ),
                synchronous_pipeline_compilation: false,
                debug_flags: bevy::render::RenderDebugFlags::empty(),
            },
            bevy::image::ImagePlugin {
                default_sampler: bevy::image::ImageSamplerDescriptor::linear(),
            },
            bevy::mesh::MeshPlugin,
            bevy::camera::CameraPlugin,
            bevy::render::pipelined_rendering::PipelinedRenderingPlugin,
            bevy::core_pipeline::CorePipelinePlugin,
            bevy::sprite::SpritePlugin,
            bevy::sprite_render::SpriteRenderPlugin,
            bevy::text::TextPlugin,
        ))
        .insert_resource(bevy::camera::ClearColor(bevy::color::Color::srgb(
            0.043, 0.043, 0.043,
        )))
        .insert_resource(bevy::winit::WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::reactive_low_power(
                std::time::Duration::MAX,
            ),
        })
        .add_systems(
            bevy::app::Startup,
            |mut commands: bevy::ecs::system::Commands<'_, '_>| {
                commands.spawn((bevy::camera::Camera2d, bevy::render::view::Msaa::Off));
            },
        )
        .add_systems(
            bevy::app::Update,
            |(keys, windows): (
                bevy::ecs::system::Res<
                    '_,
                    bevy::input::ButtonInput<bevy::input::keyboard::KeyCode>,
                >,
                bevy::ecs::system::Query<
                    '_,
                    '_,
                    &bevy::window::Window,
                    bevy::ecs::query::With<bevy::window::PrimaryWindow>,
                >,
            ),
             (time, mut exits): (
                bevy::ecs::system::Res<'_, bevy::time::Time<bevy::time::Real>>,
                bevy::ecs::message::MessageWriter<'_, bevy::app::AppExit>,
            ),
             (mut images, mut commands): (
                bevy::ecs::system::ResMut<'_, bevy::asset::Assets<bevy::image::Image>>,
                bevy::ecs::system::Commands<'_, '_>,
            ),
             mut settings: bevy::ecs::system::ResMut<'_, bevy::winit::WinitSettings>,
             sprites: bevy::ecs::system::Query<
                '_,
                '_,
                (
                    &bevy::sprite::Sprite,
                    &bevy::transform::components::Transform,
                    &bevy::camera::visibility::Visibility,
                ),
                bevy::ecs::query::Without<bevy::text::TextColor>,
            >,
             texts: bevy::ecs::system::Query<
                '_,
                '_,
                (
                    &bevy::text::TextColor,
                    &bevy::transform::components::Transform,
                    &bevy::camera::visibility::Visibility,
                ),
                bevy::ecs::query::Without<bevy::sprite::Sprite>,
            >,
             layout_info: bevy::ecs::system::Query<'_, '_, &bevy::text::TextLayoutInfo>,
             (mut focus_state, mut escape_state): (
                bevy::ecs::system::Local<'_, (bool, u16, u8, bool)>,
                bevy::ecs::system::Local<
                    '_,
                    (std::option::Option<std::time::Instant>, bool, bool, bool),
                >,
            ),
             (mut app_state, mut dot_state): (
                bevy::ecs::system::Local<'_, [u8; 4]>,
                bevy::ecs::system::Local<
                    '_,
                    std::option::Option<(
                        bevy::ecs::entity::Entity,
                        bevy::math::Vec2,
                        bevy::math::Vec2,
                        bevy::math::Vec2,
                    )>,
                >,
            ),
             (mut metrics_state, mut anim_state): (
                bevy::ecs::system::Local<'_, (f32, f32, bevy::math::Vec2, f32)>,
                bevy::ecs::system::Local<'_, ([f32; 17], [f32; 17], [f32; 7])>,
            ),
             (mut asset_handles, mut ui_entities): (
                bevy::ecs::system::Local<
                    '_,
                    std::option::Option<[bevy::asset::Handle<bevy::image::Image>; 14]>,
                >,
                bevy::ecs::system::Local<
                    '_,
                    std::option::Option<[bevy::ecs::entity::Entity; 110]>,
                >,
            ),
             (mut wizard_state, mut draft_state): (
                bevy::ecs::system::Local<'_, (u8, u8, u8, u8)>,
                bevy::ecs::system::Local<'_, (std::string::String, u16, bool)>,
            ),
             (mut feedback_state, mut catalog_state): (
                bevy::ecs::system::Local<'_, (u8, u8, u8)>,
                bevy::ecs::system::Local<
                    '_,
                    (
                        std::vec::Vec<(
                            u64,
                            u8,
                            std::string::String,
                            u8,
                            u8,
                        )>,
                        u8,
                    ),
                >,
            ),
             dynamic_texts: bevy::ecs::system::Query<'_, '_, &bevy::sprite::Text2d>,
             (mut list_state, mut tap_state, mut key_events): (
                bevy::ecs::system::Local<'_, (usize, u64)>,
                bevy::ecs::system::Local<'_, (bool, bool, bool)>,
                bevy::ecs::message::MessageReader<
                    '_,
                    '_,
                    bevy::input::keyboard::KeyboardInput,
                >,
            )| {
                *tap_state = (false, false, false);
                key_events.read().for_each(
                    |key_event: &bevy::input::keyboard::KeyboardInput| {
                        if key_event.state == bevy::input::ButtonState::Pressed
                            && !key_event.repeat
                        {
                            match key_event.key_code {
                                bevy::input::keyboard::KeyCode::F1 => tap_state.0 = true,
                                bevy::input::keyboard::KeyCode::KeyE => tap_state.1 = true,
                                bevy::input::keyboard::KeyCode::Escape => tap_state.2 = true,
                                _ => {}
                            }
                        }
                        if key_event.state == bevy::input::ButtonState::Pressed
                            && draft_state.2
                            && !keys.any_pressed([
                                bevy::input::keyboard::KeyCode::ControlLeft,
                                bevy::input::keyboard::KeyCode::ControlRight,
                                bevy::input::keyboard::KeyCode::AltLeft,
                                bevy::input::keyboard::KeyCode::AltRight,
                                bevy::input::keyboard::KeyCode::SuperLeft,
                                bevy::input::keyboard::KeyCode::SuperRight,
                            ])
                        {
                            if key_event.key_code == bevy::input::keyboard::KeyCode::Backspace
                                || key_event.key_code == bevy::input::keyboard::KeyCode::Delete
                            {
                                std::iter::once(
                                    if key_event.key_code
                                        == bevy::input::keyboard::KeyCode::Backspace
                                    {
                                        draft_state.1.saturating_sub(1)
                                    } else {
                                        draft_state.1
                                    },
                                )
                                .for_each(|victim: u16| {
                                    std::iter::once(
                                        draft_state
                                            .0
                                            .char_indices()
                                            .nth(usize::from(victim))
                                            .map(|(byte_at, glyph): (usize, char)| {
                                                (byte_at, glyph.len_utf8())
                                            }),
                                    )
                                    .for_each(
                                        |cut: std::option::Option<(usize, usize)>| {
                                            cut.into_iter().for_each(|(byte_at, width): (usize, usize)| {
                                                draft_state.0.replace_range(
                                                    byte_at..byte_at + width,
                                                    "",
                                                );
                                                draft_state.1 = draft_state.1.min(victim);
                                                feedback_state.0 = 0u8;
                                            });
                                        },
                                    );
                                });
                            } else if key_event.key_code
                                == bevy::input::keyboard::KeyCode::ArrowLeft
                            {
                                draft_state.1 = draft_state.1.saturating_sub(1);
                            } else if key_event.key_code
                                == bevy::input::keyboard::KeyCode::ArrowRight
                            {
                                std::iter::once(draft_state.0.chars().count()).for_each(
                                    |total: usize| {
                                        draft_state.1 = (draft_state.1 + 1)
                                            .min(u16::try_from(total).unwrap_or(u16::MAX));
                                    },
                                );
                            } else {
                                std::iter::once(
                                    key_event
                                        .text
                                        .as_deref()
                                        .map(|typed: &str| {
                                            typed
                                                .chars()
                                                .filter(|glyph: &char| !glyph.is_control())
                                                .collect::<std::string::String>()
                                        }),
                                )
                                .for_each(
                                    |typed: std::option::Option<std::string::String>| {
                                        typed.into_iter().for_each(
                                            |cleaned: std::string::String| {
                                                std::iter::once((
                                                    usize::from(draft_state.1),
                                                    draft_state.0.chars().count(),
                                                ))
                                                .for_each(|(caret, total): (usize, usize)| {
                                                    std::iter::once(
                                                        cleaned
                                                            .chars()
                                                            .take(64usize.saturating_sub(total))
                                                            .collect::<std::string::String>(),
                                                    )
                                                    .for_each(|piece: std::string::String| {
                                                        if !piece.is_empty() {
                                                            std::iter::once(
                                                                draft_state
                                                                    .0
                                                                    .char_indices()
                                                                    .nth(caret)
                                                                    .map(|(byte_at, _): (usize, char)| byte_at)
                                                                    .unwrap_or(draft_state.0.len()),
                                                            )
                                                            .for_each(|byte_at: usize| {
                                                                draft_state.0.insert_str(
                                                                    byte_at, &piece,
                                                                );
                                                                draft_state.1 = u16::try_from(
                                                                    caret + piece.chars().count(),
                                                                )
                                                                .unwrap_or(u16::MAX);
                                                                feedback_state.0 = 0u8;
                                                            });
                                                        }
                                                    });
                                                });
                                            },
                                        );
                                    },
                                );
                            }
                        }
                    },
                );
                windows.single().into_iter().for_each(|window: &bevy::window::Window| {
                    if focus_state.2 == 0u8 {
                        anim_state.2 = [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0];
                    }

                    if asset_handles.is_none() {
                        *asset_handles = std::option::Option::Some([
                            images.add(
                                bevy::image::Image::from_buffer(
                                    &[
                                        137u8, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68,
                                        82, 0, 0, 0, 32, 0, 0, 0, 32, 8, 6, 0, 0, 0, 115, 122, 122,
                                        244, 0, 0, 0, 211, 73, 68, 65, 84, 120, 218, 99, 96, 24, 5,
                                        163, 96, 168, 130, 255, 255, 255, 203, 0, 241, 97, 40, 150,
                                        161, 183, 229, 58, 64, 252, 252, 63, 2, 128, 216, 58, 244,
                                        178, 220, 6, 136, 63, 255, 199, 4, 32, 49, 27, 90, 91, 30,
                                        2, 196, 191, 255, 227, 6, 32, 185, 16, 90, 89, 94, 240, 159,
                                        120, 80, 64, 109, 203, 187, 255, 147, 14, 186, 169, 97, 49,
                                        11, 16, 47, 255, 79, 62, 0, 233, 101, 33, 215, 114, 30, 32,
                                        222, 253, 159, 114, 0, 50, 131, 135, 84, 203, 37, 128, 248,
                                        252, 127, 234, 1, 144, 89, 18, 196, 90, 174, 1, 196, 183,
                                        255, 83, 31, 128, 204, 212, 32, 198, 1, 199, 145, 52, 189,
                                        167, 130, 197, 200, 102, 28, 39, 198, 1, 243, 161, 138, 247,
                                        3, 177, 15, 21, 28, 224, 3, 53, 11, 4, 230, 19, 155, 242,
                                        53, 160, 180, 3, 14, 67, 65, 6, 54, 160, 225, 253, 56, 212,
                                        58, 32, 155, 73, 106, 98, 196, 229, 0, 23, 44, 106, 93, 112,
                                        57, 128, 146, 114, 192, 129, 88, 67, 73, 81, 59, 234, 128,
                                        81, 7, 12, 29, 7, 140, 130, 81, 48, 10, 232, 1, 0, 148, 140,
                                        151, 121, 14, 221, 47, 11, 0, 0, 0, 0, 73, 69, 78, 68, 174,
                                        66, 96, 130,
                                    ],
                                    bevy::image::ImageType::Extension("png"),
                                    bevy::image::CompressedImageFormats::NONE,
                                    true,
                                    bevy::image::ImageSampler::Default,
                                    bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                )
                                .expect("embedded UI texture must decode"),
                            ),
                            images.add(
                                bevy::image::Image::from_buffer(
                                    &[
                                        137u8, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68,
                                        82, 0, 0, 0, 32, 0, 0, 0, 32, 8, 6, 0, 0, 0, 115, 122, 122,
                                        244, 0, 0, 0, 132, 73, 68, 65, 84, 120, 218, 237, 150, 205,
                                        9, 192, 32, 12, 70, 29, 199, 33, 28, 200, 237, 28, 32, 7,
                                        199, 201, 0, 14, 144, 70, 232, 33, 7, 107, 32, 23, 91, 250,
                                        61, 200, 69, 62, 227, 3, 127, 48, 37, 0, 192, 3, 34, 146,
                                        181, 154, 86, 95, 20, 105, 21, 147, 45, 247, 216, 42, 59,
                                        123, 228, 136, 192, 156, 184, 131, 77, 150, 157, 108, 139,
                                        8, 116, 167, 169, 152, 172, 71, 135, 0, 4, 34, 2, 228, 52,
                                        29, 38, 59, 156, 44, 69, 4, 202, 230, 122, 205, 5, 171, 201,
                                        214, 141, 4, 219, 55, 3, 124, 103, 11, 222, 112, 8, 241, 14,
                                        64, 224, 184, 192, 241, 15, 201, 217, 47, 25, 0, 191, 225,
                                        2, 132, 108, 239, 157, 44, 191, 82, 240, 0, 0, 0, 0, 73, 69,
                                        78, 68, 174, 66, 96, 130,
                                    ],
                                    bevy::image::ImageType::Extension("png"),
                                    bevy::image::CompressedImageFormats::NONE,
                                    true,
                                    bevy::image::ImageSampler::Default,
                                    bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                )
                                .expect("embedded UI texture must decode"),
                            ),
                            images.add(
                                bevy::image::Image::from_buffer(
                                    &[
                                        137u8, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68,
                                        82, 0, 0, 0, 32, 0, 0, 0, 32, 8, 6, 0, 0, 0, 115, 122, 122,
                                        244, 0, 0, 1, 69, 73, 68, 65, 84, 120, 218, 237, 86, 161,
                                        14, 131, 48, 16, 69, 76, 32, 43, 249, 0, 62, 128, 79, 64,
                                        32, 248, 8, 228, 62, 3, 65, 50, 49, 177, 15, 153, 168, 220,
                                        7, 76, 34, 16, 19, 147, 147, 19, 124, 192, 4, 146, 93, 151,
                                        107, 114, 233, 74, 41, 165, 44, 100, 225, 37, 79, 208, 222,
                                        113, 175, 189, 75, 239, 130, 96, 195, 6, 71, 244, 125, 159,
                                        1, 27, 100, 246, 203, 192, 49, 240, 210, 127, 67, 172, 197,
                                        75, 6, 102, 192, 35, 176, 35, 65, 229, 13, 72, 116, 104,
                                        195, 124, 6, 222, 1, 247, 192, 150, 4, 106, 113, 109, 55,
                                        182, 63, 55, 120, 10, 188, 217, 156, 112, 224, 134, 132,
                                        111, 234, 18, 56, 2, 158, 149, 28, 115, 155, 28, 99, 141,
                                        112, 197, 87, 252, 43, 154, 34, 160, 36, 206, 47, 96, 238,
                                        112, 136, 28, 125, 37, 202, 41, 206, 149, 114, 130, 73, 87,
                                        169, 73, 157, 64, 229, 34, 224, 1, 172, 135, 210, 128, 121,
                                        103, 134, 235, 175, 241, 31, 206, 2, 174, 248, 93, 0, 159,
                                        164, 16, 79, 74, 32, 142, 107, 178, 0, 133, 109, 129, 190,
                                        215, 217, 2, 112, 45, 4, 30, 148, 42, 87, 209, 161, 77, 72,
                                        252, 252, 8, 32, 123, 137, 65, 64, 162, 177, 247, 46, 128,
                                        25, 4, 176, 197, 5, 224, 62, 215, 4, 231, 3, 182, 139, 8,
                                        136, 52, 69, 24, 253, 76, 0, 177, 251, 96, 196, 102, 150,
                                        128, 198, 212, 80, 198, 4, 96, 163, 106, 92, 4, 208, 167,
                                        248, 62, 52, 112, 152, 4, 224, 192, 114, 119, 125, 138, 173,
                                        154, 145, 78, 128, 151, 102, 100, 219, 142, 169, 0, 175,
                                        237, 120, 194, 64, 34, 177, 204, 64, 98, 49, 112, 232, 158,
                                        98, 191, 35, 217, 106, 134, 210, 213, 140, 229, 27, 254, 14,
                                        111, 113, 49, 85, 189, 230, 39, 132, 216, 0, 0, 0, 0, 73,
                                        69, 78, 68, 174, 66, 96, 130,
                                    ],
                                    bevy::image::ImageType::Extension("png"),
                                    bevy::image::CompressedImageFormats::NONE,
                                    true,
                                    bevy::image::ImageSampler::Default,
                                    bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                )
                                .expect("embedded UI texture must decode"),
                            ),
                            images.add(
                                bevy::image::Image::from_buffer(
                                    &[
                                        137u8, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68,
                                        82, 0, 0, 0, 32, 0, 0, 0, 32, 8, 6, 0, 0, 0, 115, 122, 122,
                                        244, 0, 0, 0, 255, 73, 68, 65, 84, 120, 218, 99, 96, 24, 5,
                                        163, 128, 4, 240, 255, 255, 255, 8, 32, 158, 79, 0, 71, 208,
                                        210, 1, 251, 255, 19, 6, 251, 105, 233, 0, 3, 168, 47, 113,
                                        1, 144, 156, 1, 173, 163, 193, 1, 102, 25, 146, 24, 204, 81,
                                        14, 244, 72, 7, 163, 14, 24, 112, 7, 120, 224, 113, 128, 7,
                                        61, 28, 176, 25, 106, 89, 8, 146, 88, 8, 84, 108, 51, 189,
                                        130, 255, 48, 22, 185, 195, 52, 143, 6, 160, 225, 231, 161,
                                        150, 88, 96, 145, 179, 128, 202, 157, 167, 149, 229, 9, 80,
                                        11, 86, 227, 81, 179, 26, 170, 38, 129, 218, 150, 115, 0,
                                        241, 99, 32, 254, 13, 196, 42, 120, 212, 169, 64, 213, 128,
                                        212, 114, 80, 211, 1, 53, 80, 159, 205, 134, 166, 3, 124,
                                        120, 54, 84, 109, 13, 181, 44, 151, 0, 226, 247, 80, 60,
                                        157, 136, 202, 104, 58, 146, 122, 9, 106, 56, 0, 102, 233,
                                        100, 32, 254, 78, 132, 3, 190, 67, 213, 130, 29, 67, 169,
                                        229, 26, 208, 56, 189, 15, 196, 203, 255, 19, 15, 150, 67,
                                        245, 128, 244, 106, 80, 163, 208, 233, 254, 79, 58, 232,
                                        166, 168, 112, 2, 106, 20, 64, 10, 242, 231, 68, 180, 132,
                                        208, 241, 115, 164, 40, 17, 32, 215, 17, 37, 255, 41, 7, 37,
                                        148, 68, 1, 11, 17, 217, 142, 16, 102, 25, 109, 77, 143,
                                        130, 81, 48, 168, 1, 0, 84, 110, 80, 140, 82, 253, 103, 181,
                                        0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
                                    ],
                                    bevy::image::ImageType::Extension("png"),
                                    bevy::image::CompressedImageFormats::NONE,
                                    true,
                                    bevy::image::ImageSampler::Default,
                                    bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                )
                                .expect("embedded UI texture must decode"),
                            ),
                            images.add(
                                bevy::image::Image::from_buffer(
                                    &[
                                        137u8, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68,
                                        82, 0, 0, 0, 32, 0, 0, 0, 32, 8, 6, 0, 0, 0, 115, 122, 122,
                                        244, 0, 0, 1, 4, 73, 68, 65, 84, 120, 218, 99, 96, 24, 5,
                                        163, 96, 20, 140, 2, 42, 130, 255, 255, 255, 115, 0, 177,
                                        15, 16, 183, 3, 177, 10, 16, 179, 208, 203, 98, 144, 101,
                                        211, 129, 248, 243, 127, 84, 160, 66, 15, 31, 119, 3, 241,
                                        239, 255, 216, 65, 59, 45, 45, 231, 1, 226, 195, 255, 241,
                                        3, 31, 90, 89, 206, 130, 100, 57, 40, 216, 251, 129, 120,
                                        53, 154, 229, 32, 113, 14, 90, 57, 160, 29, 201, 162, 24,
                                        36, 113, 100, 71, 76, 167, 149, 229, 10, 64, 252, 29, 201,
                                        34, 15, 36, 185, 126, 168, 216, 111, 138, 19, 32, 52, 152,
                                        35, 128, 56, 7, 136, 155, 129, 120, 62, 16, 111, 7, 226,
                                        219, 104, 65, 125, 26, 228, 8, 80, 72, 32, 229, 130, 110,
                                        106, 249, 20, 29, 188, 199, 147, 226, 97, 224, 48, 213, 226,
                                        30, 234, 179, 213, 72, 150, 78, 38, 194, 114, 30, 90, 196,
                                        187, 4, 16, 39, 160, 37, 62, 100, 240, 29, 42, 199, 66, 235,
                                        130, 231, 56, 154, 197, 191, 161, 105, 66, 129, 214, 22,
                                        219, 0, 241, 107, 104, 16, 31, 135, 250, 118, 50, 52, 77,
                                        108, 167, 71, 89, 15, 203, 223, 175, 161, 252, 4, 164, 80,
                                        152, 79, 15, 7, 72, 64, 29, 97, 131, 196, 135, 37, 204, 230,
                                        129, 170, 122, 97, 161, 146, 51, 80, 14, 240, 128, 58, 32,
                                        98, 32, 27, 32, 10, 116, 107, 116, 140, 130, 81, 48, 108, 1,
                                        0, 32, 227, 53, 125, 1, 188, 2, 63, 0, 0, 0, 0, 73, 69, 78,
                                        68, 174, 66, 96, 130,
                                    ],
                                    bevy::image::ImageType::Extension("png"),
                                    bevy::image::CompressedImageFormats::NONE,
                                    true,
                                    bevy::image::ImageSampler::Default,
                                    bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                )
                                .expect("embedded UI texture must decode"),
                            ),
                            images.add(
                                bevy::image::Image::from_buffer(
                                    &[
                                        137u8, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68,
                                        82, 0, 0, 0, 32, 0, 0, 0, 32, 8, 6, 0, 0, 0, 115, 122, 122,
                                        244, 0, 0, 0, 172, 73, 68, 65, 84, 120, 218, 237, 150, 193,
                                        9, 192, 32, 12, 69, 61, 122, 232, 40, 14, 225, 16, 25, 192,
                                        81, 220, 196, 49, 28, 192, 33, 122, 236, 40, 54, 66, 122,
                                        172, 160, 166, 81, 138, 31, 62, 130, 96, 124, 240, 35, 70,
                                        169, 173, 173, 85, 149, 115, 214, 104, 135, 246, 131, 46,
                                        53, 116, 207, 229, 41, 243, 41, 53, 65, 16, 117, 209, 137,
                                        6, 180, 237, 52, 80, 141, 34, 215, 2, 224, 233, 16, 48, 68,
                                        9, 84, 203, 247, 0, 88, 6, 0, 59, 2, 16, 209, 97, 208, 113,
                                        4, 128, 83, 60, 17, 208, 126, 144, 138, 96, 109, 0, 92, 143,
                                        202, 19, 52, 18, 0, 182, 146, 249, 37, 1, 96, 202, 69, 47,
                                        142, 255, 239, 129, 21, 154, 112, 110, 4, 43, 52, 225, 220,
                                        103, 40, 209, 3, 192, 0, 208, 245, 29, 215, 6, 146, 231,
                                        151, 252, 116, 32, 153, 59, 146, 77, 31, 74, 183, 182, 36,
                                        117, 3, 179, 101, 165, 43, 18, 99, 136, 153, 0, 0, 0, 0, 73,
                                        69, 78, 68, 174, 66, 96, 130,
                                    ],
                                    bevy::image::ImageType::Extension("png"),
                                    bevy::image::CompressedImageFormats::NONE,
                                    true,
                                    bevy::image::ImageSampler::Default,
                                    bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                )
                                .expect("embedded UI texture must decode"),
                            ),
                            images.add(
                                bevy::image::Image::from_buffer(
                                    &[
                                        137u8, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68,
                                        82, 0, 0, 0, 32, 0, 0, 0, 32, 8, 6, 0, 0, 0, 115, 122, 122,
                                        244, 0, 0, 0, 93, 73, 68, 65, 84, 120, 218, 237, 149, 177,
                                        9, 0, 32, 12, 4, 45, 28, 50, 165, 187, 56, 134, 195, 70,
                                        139, 128, 41, 172, 68, 136, 132, 59, 248, 202, 34, 7, 241,
                                        73, 41, 0, 0, 151, 168, 106, 93, 233, 150, 26, 33, 32, 186,
                                        145, 8, 129, 230, 4, 26, 2, 121, 5, 236, 183, 139, 13, 244,
                                        25, 78, 96, 28, 222, 229, 73, 59, 172, 102, 183, 244, 20, 2,
                                        177, 43, 160, 5, 8, 252, 124, 140, 98, 207, 49, 0, 164, 97,
                                        2, 136, 103, 151, 240, 156, 95, 175, 42, 0, 0, 0, 0, 73, 69,
                                        78, 68, 174, 66, 96, 130,
                                    ],
                                    bevy::image::ImageType::Extension("png"),
                                    bevy::image::CompressedImageFormats::NONE,
                                    true,
                                    bevy::image::ImageSampler::Default,
                                    bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                )
                                .expect("embedded UI texture must decode"),
                            ),
                            images.add(
                                bevy::image::Image::from_buffer(
                                    &[
                                        137u8, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68,
                                        82, 0, 0, 0, 32, 0, 0, 0, 32, 8, 6, 0, 0, 0, 115, 122, 122,
                                        244, 0, 0, 0, 137, 73, 68, 65, 84, 120, 218, 237, 151, 193,
                                        9, 192, 32, 12, 69, 51, 144, 35, 116, 4, 183, 116, 24, 135,
                                        112, 140, 14, 240, 27, 193, 131, 7, 75, 161, 173, 126, 15,
                                        255, 65, 110, 146, 60, 33, 129, 196, 108, 0, 128, 232, 145,
                                        60, 10, 190, 83, 90, 174, 104, 79, 248, 163, 224, 145, 49,
                                        143, 154, 59, 220, 21, 63, 60, 78, 204, 167, 214, 56, 70,
                                        63, 95, 81, 188, 151, 8, 189, 64, 198, 122, 114, 223, 112,
                                        44, 162, 181, 14, 101, 145, 236, 167, 81, 123, 61, 162, 6,
                                        50, 18, 144, 128, 4, 36, 32, 1, 9, 72, 96, 11, 1, 250, 66,
                                        66, 95, 201, 184, 75, 41, 125, 45, 223, 226, 48, 161, 159,
                                        102, 91, 28, 167, 172, 243, 252, 2, 41, 214, 28, 70, 85, 68,
                                        209, 65, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
                                    ],
                                    bevy::image::ImageType::Extension("png"),
                                    bevy::image::CompressedImageFormats::NONE,
                                    true,
                                    bevy::image::ImageSampler::Default,
                                    bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                )
                                .expect("embedded UI texture must decode"),
                            ),
                            images.add(
                                bevy::image::Image::from_buffer(
                                    &[
                                        137u8, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68,
                                        82, 0, 0, 0, 32, 0, 0, 0, 32, 8, 6, 0, 0, 0, 115, 122, 122,
                                        244, 0, 0, 0, 198, 73, 68, 65, 84, 120, 218, 237, 87, 201,
                                        13, 131, 64, 12, 156, 7, 69, 164, 8, 74, 160, 4, 138, 160,
                                        20, 122, 225, 185, 133, 80, 66, 30, 91, 194, 62, 41, 192,
                                        216, 209, 60, 66, 68, 148, 72, 28, 222, 135, 71, 154, 23,
                                        139, 103, 214, 107, 75, 54, 176, 3, 17, 233, 149, 147, 50,
                                        203, 113, 100, 198, 234, 241, 11, 122, 168, 85, 206, 252,
                                        177, 40, 147, 114, 60, 200, 196, 88, 194, 216, 237, 55, 241,
                                        78, 185, 240, 240, 160, 108, 112, 18, 44, 22, 99, 22, 106,
                                        116, 123, 55, 183, 15, 79, 229, 3, 23, 193, 98, 83, 99, 217,
                                        100, 130, 169, 41, 87, 138, 127, 152, 48, 173, 249, 189,
                                        224, 12, 3, 110, 2, 159, 67, 94, 133, 201, 10, 45, 103, 190,
                                        249, 159, 53, 97, 154, 19, 216, 38, 9, 55, 131, 221, 145,
                                        193, 84, 140, 14, 6, 172, 69, 37, 12, 132, 129, 48, 16, 6,
                                        194, 64, 24, 8, 3, 85, 24, 112, 31, 72, 220, 71, 50, 223,
                                        161, 212, 125, 44, 175, 98, 49, 113, 95, 205, 170, 88, 78,
                                        189, 214, 243, 21, 237, 121, 110, 202, 111, 5, 188, 21, 0,
                                        0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
                                    ],
                                    bevy::image::ImageType::Extension("png"),
                                    bevy::image::CompressedImageFormats::NONE,
                                    true,
                                    bevy::image::ImageSampler::Default,
                                    bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                )
                                .expect("embedded UI texture must decode"),
                            ),
                            images.add(
                                bevy::image::Image::from_buffer(
                                    &[
                                        137u8, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68,
                                        82, 0, 0, 0, 40, 0, 0, 0, 40, 8, 6, 0, 0, 0, 140, 254, 184,
                                        109, 0, 0, 0, 205, 73, 68, 65, 84, 120, 218, 237, 153, 187,
                                        9, 196, 48, 16, 68, 55, 112, 57, 46, 67, 5, 184, 4, 247,
                                        227, 194, 28, 170, 0, 149, 160, 80, 161, 131, 185, 21, 108,
                                        96, 115, 28, 167, 59, 99, 123, 4, 251, 96, 82, 237, 3, 161,
                                        223, 74, 164, 1, 0, 65, 179, 104, 86, 77, 198, 255, 100, 27,
                                        163, 142, 21, 228, 12, 58, 192, 160, 153, 53, 9, 215, 145,
                                        172, 198, 240, 171, 220, 168, 137, 184, 143, 90, 107, 108,
                                        149, 155, 52, 5, 247, 83, 107, 78, 45, 114, 27, 158, 99,
                                        251, 40, 105, 211, 90, 240, 60, 229, 109, 186, 109, 65, 68,
                                        240, 16, 15, 11, 199, 86, 18, 27, 243, 94, 48, 17, 10, 166,
                                        253, 38, 204, 74, 16, 219, 213, 89, 89, 196, 142, 30, 86,
                                        86, 57, 121, 182, 94, 77, 22, 144, 227, 130, 46, 232, 130,
                                        46, 232, 130, 46, 232, 130, 46, 248, 85, 144, 254, 186, 69,
                                        127, 97, 165, 191, 242, 115, 63, 154, 232, 159, 157, 189,
                                        60, 220, 185, 91, 31, 244, 205, 163, 46, 218, 111, 93, 52,
                                        48, 187, 104, 1, 119, 209, 68, 103, 250, 134, 120, 1, 111,
                                        93, 194, 131, 217, 50, 190, 59, 0, 0, 0, 0, 73, 69, 78, 68,
                                        174, 66, 96, 130,
                                    ],
                                    bevy::image::ImageType::Extension("png"),
                                    bevy::image::CompressedImageFormats::NONE,
                                    true,
                                    bevy::image::ImageSampler::Default,
                                    bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                )
                                .expect("embedded UI texture must decode"),
                            ),
                            images.add(
                                bevy::image::Image::from_buffer(
                                    &[
                                        137u8, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68,
                                        82, 0, 0, 0, 40, 0, 0, 0, 40, 8, 6, 0, 0, 0, 140, 254, 184,
                                        109, 0, 0, 0, 153, 73, 68, 65, 84, 120, 218, 237, 217, 177,
                                        9, 128, 48, 20, 132, 225, 20, 142, 227, 24, 14, 224, 8, 217,
                                        199, 193, 44, 29, 32, 35, 164, 76, 105, 241, 124, 17, 20,
                                        196, 70, 27, 253, 149, 59, 184, 254, 3, 81, 146, 51, 216,
                                        179, 201, 222, 209, 59, 120, 187, 112, 37, 246, 110, 146,
                                        55, 122, 27, 42, 112, 203, 228, 109, 201, 192, 154, 226,
                                        237, 201, 192, 154, 249, 132, 52, 94, 202, 225, 113, 27, 51,
                                        211, 254, 226, 24, 55, 145, 14, 76, 116, 160, 173, 31, 115,
                                        56, 112, 160, 3, 71, 58, 48, 211, 129, 38, 160, 128, 2, 10,
                                        40, 160, 128, 2, 10, 248, 105, 96, 214, 129, 245, 239, 71,
                                        254, 78, 215, 206, 191, 94, 220, 209, 211, 7, 122, 60, 66,
                                        207, 111, 232, 1, 19, 59, 1, 227, 70, 244, 219, 191, 33, 22,
                                        102, 83, 213, 118, 44, 177, 29, 179, 0, 0, 0, 0, 73, 69, 78,
                                        68, 174, 66, 96, 130,
                                    ],
                                    bevy::image::ImageType::Extension("png"),
                                    bevy::image::CompressedImageFormats::NONE,
                                    true,
                                    bevy::image::ImageSampler::Default,
                                    bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                )
                                .expect("embedded UI texture must decode"),
                            ),
                            images.add(
                                bevy::image::Image::from_buffer(
                                    &[
                                        137u8, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68,
                                        82, 0, 0, 0, 32, 0, 0, 0, 32, 8, 6, 0, 0, 0, 115, 122, 122,
                                        244, 0, 0, 1, 108, 73, 68, 65, 84, 120, 218, 205, 87, 177,
                                        13, 131, 48, 16, 116, 155, 134, 130, 154, 6, 209, 83, 164,
                                        101, 147, 12, 192, 52, 89, 0, 144, 216, 128, 13, 24, 33,
                                        162, 66, 140, 64, 151, 54, 5, 17, 250, 252, 89, 182, 244, 5,
                                        74, 72, 32, 216, 39, 157, 20, 17, 251, 239, 109, 191, 207,
                                        182, 82, 43, 17, 134, 97, 192, 188, 48, 75, 230, 141, 121,
                                        103, 146, 225, 221, 124, 43, 77, 155, 64, 237, 5, 14, 150,
                                        48, 107, 230, 67, 8, 126, 226, 195, 244, 73, 182, 8, 159,
                                        152, 87, 230, 211, 6, 142, 227, 152, 242, 60, 167, 162, 40,
                                        168, 109, 91, 234, 186, 78, 19, 191, 241, 13, 255, 161, 141,
                                        72, 228, 105, 98, 156, 126, 25, 117, 111, 3, 101, 89, 70,
                                        77, 211, 208, 52, 77, 244, 9, 104, 131, 182, 232, 35, 18,
                                        233, 87, 207, 6, 55, 60, 219, 245, 141, 162, 136, 170, 170,
                                        162, 121, 158, 233, 91, 160, 15, 250, 34, 134, 168, 147,
                                        243, 154, 145, 107, 241, 52, 77, 105, 24, 6, 218, 10, 196,
                                        64, 44, 145, 68, 242, 110, 205, 123, 43, 62, 142, 35, 237,
                                        5, 196, 18, 73, 244, 139, 53, 97, 138, 69, 79, 217, 30, 35,
                                        95, 154, 9, 177, 28, 215, 165, 169, 215, 213, 142, 117, 251,
                                        23, 16, 91, 236, 142, 68, 38, 80, 219, 106, 255, 165, 224,
                                        190, 41, 76, 177, 59, 106, 233, 112, 218, 100, 176, 125,
                                        254, 13, 104, 8, 179, 10, 148, 177, 78, 109, 32, 107, 246,
                                        249, 86, 64, 67, 152, 213, 69, 25, 255, 214, 46, 118, 20,
                                        160, 101, 18, 40, 149, 57, 68, 180, 149, 30, 5, 104, 153, 4,
                                        110, 202, 26, 15, 252, 252, 40, 64, 203, 26, 147, 178, 126,
                                        141, 67, 229, 40, 64, 203, 234, 122, 145, 128, 243, 37, 112,
                                        94, 132, 206, 183, 161, 115, 35, 114, 107, 197, 206, 15, 35,
                                        47, 142, 99, 231, 23, 18, 47, 174, 100, 206, 47, 165, 94,
                                        92, 203, 189, 120, 152, 120, 241, 52, 243, 226, 113, 122,
                                        212, 243, 252, 5, 85, 133, 84, 95, 73, 243, 143, 15, 0, 0,
                                        0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
                                    ],
                                    bevy::image::ImageType::Extension("png"),
                                    bevy::image::CompressedImageFormats::NONE,
                                    true,
                                    bevy::image::ImageSampler::Default,
                                    bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                )
                                .expect("embedded UI texture must decode"),
                            ),
                            images.add(
                                bevy::image::Image::from_buffer(
                                    &[
                                        137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68,
                                        82, 0, 0, 0, 32, 0, 0, 0, 32, 8, 6, 0, 0, 0, 115,
                                        122, 122, 244, 0, 0, 1, 30, 73, 68, 65, 84, 120, 218, 237, 151,
                                        59, 14, 130, 64, 16, 134, 209, 198, 51, 16, 19, 34, 177, 32, 241,
                                        2, 20, 222, 3, 26, 77, 168, 108, 41, 61, 14, 189, 87, 224, 2,
                                        22, 16, 66, 171, 49, 158, 0, 56, 129, 15, 190, 98, 27, 162, 130,
                                        128, 44, 65, 38, 153, 132, 204, 50, 223, 252, 251, 98, 23, 69, 25,
                                        173, 175, 54, 203, 77, 85, 213, 185, 166, 105, 139, 38, 14, 3, 86,
                                        229, 194, 147, 220, 76, 211, 92, 123, 158, 119, 8, 130, 224, 20, 199,
                                        241, 181, 137, 195, 128, 5, 19, 118, 169, 0, 94, 12, 195, 240, 156,
                                        101, 217, 163, 77, 135, 9, 187, 116, 216, 81, 219, 118, 113, 225, 176,
                                        63, 78, 7, 243, 197, 144, 137, 132, 52, 77, 239, 190, 239, 31, 93,
                                        215, 221, 59, 142, 179, 251, 198, 201, 33, 23, 134, 224, 193, 166, 198,
                                        91, 1, 44, 26, 230, 77, 36, 0, 208, 117, 125, 89, 119, 33, 147,
                                        11, 67, 240, 96, 83, 163, 178, 0, 122, 209, 116, 55, 193, 168, 45,
                                        128, 161, 108, 42, 0, 198, 240, 4, 76, 115, 51, 12, 99, 101, 89,
                                        214, 6, 231, 153, 88, 39, 2, 40, 100, 219, 246, 54, 138, 162, 75,
                                        146, 36, 55, 156, 103, 98, 175, 68, 180, 46, 128, 222, 82, 176, 184,
                                        199, 137, 209, 246, 115, 1, 12, 57, 189, 46, 10, 32, 70, 219, 240,
                                        5, 72, 159, 2, 233, 139, 80, 250, 54, 252, 175, 47, 161, 244, 195,
                                        168, 243, 227, 88, 250, 133, 68, 250, 149, 76, 250, 165, 180, 23, 215,
                                        114, 233, 63, 38, 163, 117, 109, 79, 105, 71, 148, 253, 246, 81, 21,
                                        233, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
                                    ],
                                    bevy::image::ImageType::Extension("png"),
                                    bevy::image::CompressedImageFormats::NONE,
                                    true,
                                    bevy::image::ImageSampler::Default,
                                    bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                )
                                .expect("embedded UI texture must decode"),
                            ),
                            images.add(
                                bevy::image::Image::from_buffer(
                                    &[
                                        137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68,
                                        82, 0, 0, 0, 32, 0, 0, 0, 32, 8, 6, 0, 0, 0, 115,
                                        122, 122, 244, 0, 0, 0, 205, 73, 68, 65, 84, 120, 218, 99, 96,
                                        24, 5, 163, 96, 20, 96, 1, 122, 122, 122, 134, 69, 69, 69, 85,
                                        21, 21, 21, 13, 212, 192, 32, 179, 64, 102, 18, 101, 185, 144, 144,
                                        144, 240, 142, 29, 59, 142, 188, 127, 255, 254, 63, 53, 49, 200, 76,
                                        144, 217, 4, 29, 32, 39, 39, 167, 112, 241, 226, 197, 251, 212, 118,
                                        0, 200, 76, 144, 217, 131, 223, 1, 3, 30, 5, 3, 158, 8, 7,
                                        21, 224, 0, 130, 160, 160, 160, 240, 242, 242, 242, 122, 82, 125, 13,
                                        210, 3, 210, 11, 50, 131, 108, 7, 184, 187, 187, 123, 63, 126, 252,
                                        248, 19, 185, 241, 14, 210, 11, 50, 131, 108, 7, 36, 36, 36, 164,
                                        81, 154, 248, 64, 102, 12, 93, 7, 12, 120, 20, 12, 120, 34, 28,
                                        84, 64, 80, 80, 80, 40, 49, 49, 49, 157, 144, 207, 65, 106, 64,
                                        106, 169, 238, 128, 188, 188, 188, 178, 87, 175, 94, 253, 34, 20, 239,
                                        32, 53, 32, 181, 84, 119, 192, 132, 9, 19, 102, 18, 155, 248, 64,
                                        106, 135, 159, 3, 6, 60, 10, 6, 60, 17, 142, 130, 81, 48, 108,
                                        1, 0, 35, 126, 160, 6, 245, 13, 191, 80, 0, 0, 0, 0, 73,
                                        69, 78, 68, 174, 66, 96, 130,
                                    ],
                                    bevy::image::ImageType::Extension("png"),
                                    bevy::image::CompressedImageFormats::NONE,
                                    true,
                                    bevy::image::ImageSampler::Default,
                                    bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                )
                                .expect("embedded UI texture must decode"),
                            ),
                        ]);
                    }

                    if ui_entities.is_none() {
                        asset_handles.as_ref().cloned().into_iter().for_each(
                            |[
                                home_icon,
                                registry_icon,
                                app_icon,
                                flask_icon,
                                rocket_icon,
                                workspace_icon,
                                plus_icon,
                                round_solid,
                                round_outline,
                                panel_round_solid,
                                right_round_solid,
                                dot_image,
                                preset_icon_a,
                                preset_icon_b,
                            ]: [bevy::asset::Handle<bevy::image::Image>; 14]| {
                                *ui_entities = std::option::Option::Some([
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.180, 0.180, 0.180, 1.0,
                                                ),
                                                image: right_round_solid.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::new(384.0, 768.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(16.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 1.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Inherited,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.086, 0.086, 0.086, 1.0,
                                                ),
                                                image: right_round_solid.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::new(383.0, 766.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(16.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 1.1,
                                            ),
                                            bevy::camera::visibility::Visibility::Inherited,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.149, 0.149, 0.149, 1.0,
                                                ),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::splat(44.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 2.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Inherited,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.937, 0.937, 0.937, 1.0,
                                                ),
                                                image: app_icon.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::splat(26.0),
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 3.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Inherited,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.937, 0.937, 0.937, 0.0,
                                                ),
                                                image: flask_icon.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::splat(26.0),
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 3.1,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from(
                                                if cfg!(target_os = "linux") {
                                                    "Subsystem for Arxumbra"
                                                } else if cfg!(target_os = "macos") {
                                                    "Subsystem for Arxumbra"
                                                } else {
                                                    "Subsystem for Arxumbra"
                                                },
                                            )),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(17.0),
                                                weight: bevy::text::FontWeight::SEMIBOLD,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(
                                                0.937, 0.937, 0.937, 1.0,
                                            )),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 3.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Inherited,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from(
                                                "Laboratory",
                                            )),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(17.0),
                                                weight: bevy::text::FontWeight::SEMIBOLD,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(
                                                0.937, 0.937, 0.937, 0.0,
                                            )),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 3.1,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.176, 0.176, 0.176, 1.0,
                                                ),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::new(348.0, 1.0),
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 2.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Inherited,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.865, 0.865, 0.865, 1.0,
                                                ),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::new(348.0, 48.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 2.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Inherited,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.520, 0.520, 0.520, 0.0,
                                                ),
                                                image: round_outline.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::new(348.0, 48.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 2.5,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.161, 0.161, 0.161, 1.0,
                                                ),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::splat(36.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 3.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Inherited,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.937, 0.937, 0.937, 1.0,
                                                ),
                                                image: home_icon.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::splat(22.0),
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 4.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Inherited,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from(
                                                "Home",
                                            )),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(16.0),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(
                                                0.086, 0.086, 0.086, 1.0,
                                            )),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 4.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Inherited,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.125, 0.125, 0.125, 1.0,
                                                ),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::new(348.0, 48.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 2.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Inherited,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.520, 0.520, 0.520, 0.0,
                                                ),
                                                image: round_outline.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::new(348.0, 48.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 2.5,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.176, 0.176, 0.176, 1.0,
                                                ),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::splat(36.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 3.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Inherited,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.780, 0.780, 0.780, 1.0,
                                                ),
                                                image: registry_icon.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::splat(22.0),
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 4.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Inherited,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from(
                                                "Extension Registry",
                                            )),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(16.0),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(
                                                0.780, 0.780, 0.780, 1.0,
                                            )),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 4.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Inherited,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.145, 0.145, 0.145, 0.0,
                                                ),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::new(348.0, 48.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 2.1,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.275, 0.275, 0.275, 0.0,
                                                ),
                                                image: round_outline.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::new(348.0, 48.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 2.6,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.200, 0.200, 0.200, 0.0,
                                                ),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::splat(36.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 3.1,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.880, 0.880, 0.880, 0.0,
                                                ),
                                                image: plus_icon.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::splat(20.0),
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 4.1,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from(
                                                "Create Project",
                                            )),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(16.0),
                                                weight: bevy::text::FontWeight::SEMIBOLD,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(
                                                0.880, 0.880, 0.880, 0.0,
                                            )),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 4.1,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.180, 0.180, 0.180, 0.0,
                                                ),
                                                image: panel_round_solid.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::new(800.0, 736.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(16.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 1.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.063, 0.063, 0.063, 0.0,
                                                ),
                                                image: panel_round_solid.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::new(798.0, 734.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(16.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 1.1,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.133, 0.133, 0.133, 0.0,
                                                ),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::splat(44.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 2.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.937, 0.937, 0.937, 0.0,
                                                ),
                                                image: registry_icon.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::splat(25.0),
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 3.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.937, 0.937, 0.937, 0.0,
                                                ),
                                                image: workspace_icon.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::splat(26.0),
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 3.1,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from(
                                                "Extension Registry",
                                            )),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(20.0),
                                                weight: bevy::text::FontWeight::SEMIBOLD,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(
                                                0.937, 0.937, 0.937, 0.0,
                                            )),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 3.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from(
                                                "Workspace",
                                            )),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(20.0),
                                                weight: bevy::text::FontWeight::SEMIBOLD,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(
                                                0.937, 0.937, 0.937, 0.0,
                                            )),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 3.1,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.176, 0.176, 0.176, 0.0,
                                                ),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::new(700.0, 1.0),
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 2.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.188, 0.188, 0.188, 0.0,
                                                ),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::new(480.0, 151.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 2.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.090, 0.090, 0.090, 0.0,
                                                ),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::new(478.0, 149.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 2.1,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.157, 0.157, 0.157, 0.0,
                                                ),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::splat(42.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 3.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.937, 0.937, 0.937, 0.0,
                                                ),
                                                image: flask_icon.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::splat(25.0),
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 4.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from(
                                                "Laboratory",
                                            )),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(18.0),
                                                weight: bevy::text::FontWeight::SEMIBOLD,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(
                                                0.937, 0.937, 0.937, 0.0,
                                            )),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 4.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.176, 0.176, 0.176, 0.0,
                                                ),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::new(444.0, 1.0),
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 3.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.215, 0.215, 0.215, 0.0,
                                                ),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::new(148.0, 44.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 3.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.360, 0.360, 0.360, 0.0,
                                                ),
                                                image: round_outline.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::new(148.0, 44.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 3.5,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.265, 0.265, 0.265, 0.0,
                                                ),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::splat(32.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 4.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.910, 0.910, 0.910, 0.0,
                                                ),
                                                image: rocket_icon.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::splat(20.0),
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 5.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from(
                                                "Launch",
                                            )),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(15.5),
                                                weight: bevy::text::FontWeight::SEMIBOLD,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(
                                                0.910, 0.910, 0.910, 0.0,
                                            )),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 5.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.0, 0.0, 0.0, 0.0,
                                                ),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::new(1920.0, 1080.0),
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 12.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.245, 0.245, 0.245, 0.0,
                                                ),
                                                image: panel_round_solid.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::new(460.0, 300.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(16.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 13.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(
                                                    0.094, 0.094, 0.094, 0.0,
                                                ),
                                                image: panel_round_solid.clone(),
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::new(458.0, 298.0),
                                                ),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(16.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 13.1,
                                            ),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.133, 0.133, 0.133, 1.0),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::splat(44.0)),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 14.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0),
                                                image: plus_icon.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::splat(24.0)),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 15.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("Create Project")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(19.0),
                                                weight: bevy::text::FontWeight::SEMIBOLD,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 15.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.176, 0.176, 0.176, 1.0),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::new(504.0, 1.0)),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 14.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("App Icon")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(13.0),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.62, 0.62, 0.62, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 15.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.161, 0.161, 0.161, 1.0),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::splat(68.0)),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(16.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 14.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0),
                                                image: preset_icon_a.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::splat(40.0)),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 15.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.125, 0.125, 0.125, 1.0),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::splat(68.0)),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(16.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 14.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0),
                                                image: preset_icon_b.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::splat(40.0)),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 15.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("Application Name")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(13.0),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.62, 0.62, 0.62, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 15.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.125, 0.125, 0.125, 1.0),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::new(564.0, 46.0)),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 14.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.4, 0.4, 0.4, 0.0),
                                                image: round_outline.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::new(564.0, 46.0)),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 14.5),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(16.0),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 15.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::new(2.0, 20.0)),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 15.5),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(12.5),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.7, 0.7, 0.7, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 15.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.865, 0.865, 0.865, 1.0),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::new(120.0, 44.0)),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 14.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("Next")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(15.0),
                                                weight: bevy::text::FontWeight::SEMIBOLD,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.086, 0.086, 0.086, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 15.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.145, 0.145, 0.145, 1.0),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::new(564.0, 46.0)),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 14.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("Operating System")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(15.0),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 15.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.145, 0.145, 0.145, 1.0),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::new(564.0, 46.0)),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 14.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("Command Line Interface")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(15.0),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 15.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.145, 0.145, 0.145, 1.0),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::new(564.0, 46.0)),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 14.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("Graphical User Interface")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(15.0),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 15.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.125, 0.125, 0.125, 1.0),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::new(564.0, 40.0)),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 14.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("2D")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(14.5),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 15.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.125, 0.125, 0.125, 1.0),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::new(564.0, 40.0)),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 14.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("3D")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(14.5),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 15.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.215, 0.215, 0.215, 1.0),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::new(104.0, 44.0)),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 14.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("Back")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(15.0),
                                                weight: bevy::text::FontWeight::SEMIBOLD,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 15.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("No saved projects yet")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(14.5),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.62, 0.62, 0.62, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(13.5),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.78, 0.78, 0.78, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.145, 0.145, 0.145, 1.0),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::new(700.0, 48.0)),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 2.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0),
                                                image: preset_icon_a.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::splat(26.0)),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0),
                                                image: preset_icon_b.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::splat(26.0)),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(15.0),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(12.5),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.62, 0.62, 0.62, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.145, 0.145, 0.145, 1.0),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::new(700.0, 48.0)),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 2.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0),
                                                image: preset_icon_a.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::splat(26.0)),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0),
                                                image: preset_icon_b.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::splat(26.0)),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(15.0),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(12.5),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.62, 0.62, 0.62, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.145, 0.145, 0.145, 1.0),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::new(700.0, 48.0)),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 2.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0),
                                                image: preset_icon_a.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::splat(26.0)),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0),
                                                image: preset_icon_b.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::splat(26.0)),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(15.0),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(12.5),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.62, 0.62, 0.62, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.145, 0.145, 0.145, 1.0),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::new(700.0, 48.0)),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 2.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0),
                                                image: preset_icon_a.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::splat(26.0)),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0),
                                                image: preset_icon_b.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::splat(26.0)),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(15.0),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(12.5),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.62, 0.62, 0.62, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.145, 0.145, 0.145, 1.0),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::new(700.0, 48.0)),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 2.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0),
                                                image: preset_icon_a.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::splat(26.0)),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0),
                                                image: preset_icon_b.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::splat(26.0)),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(15.0),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(12.5),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.62, 0.62, 0.62, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.145, 0.145, 0.145, 1.0),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::new(700.0, 48.0)),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(12.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 2.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0),
                                                image: preset_icon_a.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::splat(26.0)),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0),
                                                image: preset_icon_b.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::splat(26.0)),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(15.0),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(12.5),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.62, 0.62, 0.62, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER_LEFT,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.176, 0.176, 0.176, 1.0),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::splat(32.0)),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(10.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 2.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("^")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(14.0),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                color: bevy::color::Color::srgba(0.176, 0.176, 0.176, 1.0),
                                                image: round_solid.clone(),
                                                custom_size: std::option::Option::Some(bevy::math::Vec2::splat(32.0)),
                                                image_mode: bevy::sprite::SpriteImageMode::Sliced(
                                                    bevy::sprite::TextureSlicer {
                                                        border: bevy::sprite::BorderRect::all(10.0),
                                                        center_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        sides_scale_mode:
                                                            bevy::sprite::SliceScaleMode::Stretch,
                                                        max_corner_scale: 1.0,
                                                    },
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 2.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                    commands
                                        .spawn((
                                            bevy::sprite::Text2d(std::string::String::from("v")),
                                            bevy::text::TextFont {
                                                font_size: bevy::text::FontSize::Px(14.0),
                                                weight: bevy::text::FontWeight::MEDIUM,
                                                ..bevy::text::TextFont::default()
                                            },
                                            bevy::text::TextColor(bevy::color::Color::srgba(0.937, 0.937, 0.937, 1.0)),
                                            bevy::text::TextBounds::UNBOUNDED,
                                            bevy::text::TextLayout::no_wrap(),
                                            bevy::sprite::Anchor::CENTER,
                                            bevy::transform::components::Transform::from_xyz(0.0, 0.0, 3.0),
                                            bevy::camera::visibility::Visibility::Hidden,
                                        ))
                                        .id(),
                                ]);

                                *dot_state = std::option::Option::Some((
                                    commands
                                        .spawn((
                                            bevy::sprite::Sprite {
                                                image: dot_image,
                                                custom_size: std::option::Option::Some(
                                                    bevy::math::Vec2::splat(11.0),
                                                ),
                                                ..bevy::sprite::Sprite::default()
                                            },
                                            bevy::transform::components::Transform::from_xyz(
                                                0.0, 0.0, 20.0,
                                            ),
                                            bevy::camera::visibility::Visibility::Inherited,
                                        ))
                                        .id(),
                                    bevy::math::Vec2::new(-1.0, -1.0),
                                    bevy::math::Vec2::ZERO,
                                    bevy::math::Vec2::new(window.width(), window.height()),
                                ));
                            },
                        );
                    }

                    focus_state.2 = focus_state.2.saturating_add(1).min(4);
                    metrics_state.2 =
                        bevy::math::Vec2::new(window.width().max(1.0), window.height().max(1.0));
                    metrics_state.3 = time.delta_secs().clamp(0.0, 0.05);

                    if focus_state.2 >= 2u8 {
                        (*dot_state).into_iter().for_each(
                            |(dot_entity, prev_pos, _prev_vel, prev_size): (
                                bevy::ecs::entity::Entity,
                                bevy::math::Vec2,
                                bevy::math::Vec2,
                                bevy::math::Vec2,
                            )| {
                                if (prev_size != metrics_state.2
                                    && prev_pos == prev_size * 0.5)
                                    || (prev_pos.x < 0.0 && prev_size == metrics_state.2)
                                {
                                    *dot_state = std::option::Option::Some((
                                        dot_entity,
                                        metrics_state.2 * 0.5,
                                        bevy::math::Vec2::ZERO,
                                        metrics_state.2,
                                    ));
                                } else if prev_size != metrics_state.2 {
                                    *dot_state = std::option::Option::Some((
                                        dot_entity,
                                        prev_pos,
                                        bevy::math::Vec2::ZERO,
                                        metrics_state.2,
                                    ));
                                }
                            },
                        );
                    }

                    match (
                        window.focused,
                        u16::from(keys.pressed(bevy::input::keyboard::KeyCode::ControlLeft))
                            | (u16::from(
                                keys.pressed(bevy::input::keyboard::KeyCode::ControlRight),
                            ) << 1)
                            | (u16::from(keys.pressed(bevy::input::keyboard::KeyCode::KeyW)) << 2)
                            | (u16::from(keys.pressed(bevy::input::keyboard::KeyCode::KeyA)) << 3)
                            | (u16::from(keys.pressed(bevy::input::keyboard::KeyCode::KeyS)) << 4)
                            | (u16::from(keys.pressed(bevy::input::keyboard::KeyCode::KeyD)) << 5)
                            | (u16::from(keys.pressed(bevy::input::keyboard::KeyCode::KeyE)) << 6)
                            | (u16::from(keys.pressed(bevy::input::keyboard::KeyCode::F1)) << 7)
                            | (u16::from(keys.pressed(bevy::input::keyboard::KeyCode::Escape))
                                << 8),
                    ) {
                        (false, _) => {
                            focus_state.0 = false;
                            focus_state.1 = 0u16;
                            focus_state.3 = false;
                            *escape_state = (std::option::Option::None, false, false, false);
                            (*dot_state).into_iter().for_each(
                                |(dot_entity, dot_pos, _dot_vel, dot_size): (
                                    bevy::ecs::entity::Entity,
                                    bevy::math::Vec2,
                                    bevy::math::Vec2,
                                    bevy::math::Vec2,
                                )| {
                                    *dot_state = std::option::Option::Some((
                                        dot_entity,
                                        dot_pos,
                                        bevy::math::Vec2::ZERO,
                                        dot_size,
                                    ));
                                },
                            );
                        }
                        (true, pressed_mask) => {
                            if !focus_state.0 {
                                focus_state.0 = true;
                                focus_state.1 = pressed_mask;
                            } else {
                                focus_state.1 &= pressed_mask;
                            }
                            focus_state.3 = ((keys
                                .pressed(bevy::input::keyboard::KeyCode::ControlLeft)
                                && (focus_state.1 & 1u16) == 0)
                                || (keys.pressed(bevy::input::keyboard::KeyCode::ControlRight)
                                    && (focus_state.1 & 2u16) == 0))
                                && !keys.any_pressed([
                                    bevy::input::keyboard::KeyCode::ShiftLeft,
                                    bevy::input::keyboard::KeyCode::ShiftRight,
                                    bevy::input::keyboard::KeyCode::AltLeft,
                                    bevy::input::keyboard::KeyCode::AltRight,
                                    bevy::input::keyboard::KeyCode::SuperLeft,
                                    bevy::input::keyboard::KeyCode::SuperRight,
                                ]);
                        }
                    }

                    escape_state.3 = false;
                    if !window.focused || !keys.pressed(bevy::input::keyboard::KeyCode::Escape) {
                        *escape_state = (std::option::Option::None, false, false, false);
                    } else if (focus_state.1 & 256u16) == 0 && !escape_state.2 {
                        if escape_state.0.is_none() && tap_state.2 {
                            *escape_state = (
                                std::option::Option::Some(std::time::Instant::now()),
                                app_state[0] == 2u8,
                                false,
                                false,
                            );
                        }
                        escape_state
                            .0
                            .into_iter()
                            .for_each(|started: std::time::Instant| {
                                if escape_state.1 && started.elapsed().as_secs_f64() >= 5.0f64 {
                                    *escape_state = (std::option::Option::None, false, true, true);
                                    anim_state.2[3] = 0.0;
                                    app_state[0] = 1u8;
                                    app_state[2] = 0u8;
                                    *draft_state = (std::string::String::new(), 0u16, false);
                                    feedback_state.0 = 0u8;
                                    feedback_state.2 = 0u8;
                                } else if !escape_state.1
                                    && started.elapsed().as_secs_f64() >= 7.0f64
                                {
                                    exits.write(bevy::app::AppExit::Success);
                                    *escape_state = (std::option::Option::None, false, true, true);
                                }
                            });
                    }

                    if !escape_state.3
                        && window.focused
                        && tap_state.0
                        && (focus_state.1 & 128u16) == 0
                        && !keys.any_pressed([
                            bevy::input::keyboard::KeyCode::ControlLeft,
                            bevy::input::keyboard::KeyCode::ControlRight,
                            bevy::input::keyboard::KeyCode::ShiftLeft,
                            bevy::input::keyboard::KeyCode::ShiftRight,
                            bevy::input::keyboard::KeyCode::AltLeft,
                            bevy::input::keyboard::KeyCode::AltRight,
                            bevy::input::keyboard::KeyCode::SuperLeft,
                            bevy::input::keyboard::KeyCode::SuperRight,
                        ])
                    {
                        app_state[1] = u8::from(app_state[1] == 0u8);
                    }

                    ui_entities
                        .as_ref()
                        .into_iter()
                        .for_each(|entities: &[bevy::ecs::entity::Entity; 110]| {
                            layout_info
                                .get(entities[5])
                                .into_iter()
                                .for_each(|info: &bevy::text::TextLayoutInfo| {
                                    if info.scale_factor > 0.0
                                        && info.size.x.is_finite()
                                        && info.size.x > 0.0
                                    {
                                        metrics_state.0 = info.size.x / info.scale_factor;
                                    }
                                });
                        });

                    metrics_state.1 = if metrics_state.0 > 0.0 {
                        (((76.0 + metrics_state.0 + 26.0) / 8.0).ceil() * 8.0)
                            .clamp(360.0, (metrics_state.2.x * 0.48).max(360.0))
                    } else {
                        384.0f32.min((metrics_state.2.x * 0.48).max(320.0))
                    };

                    (*dot_state).into_iter().for_each(
                        |(dot_entity, prev_pos, prev_vel, _prev_size): (
                            bevy::ecs::entity::Entity,
                            bevy::math::Vec2,
                            bevy::math::Vec2,
                            bevy::math::Vec2,
                        )| {
                            std::iter::once(
                                bevy::math::Vec2::new(
                                    if focus_state.3 {
                                        f32::from(u8::from(
                                            keys.pressed(bevy::input::keyboard::KeyCode::KeyD)
                                                && (focus_state.1 & 32u16) == 0,
                                        )) - f32::from(u8::from(
                                            keys.pressed(bevy::input::keyboard::KeyCode::KeyA)
                                                && (focus_state.1 & 8u16) == 0,
                                        ))
                                    } else {
                                        0.0
                                    },
                                    if focus_state.3 {
                                        f32::from(u8::from(
                                            keys.pressed(bevy::input::keyboard::KeyCode::KeyS)
                                                && (focus_state.1 & 16u16) == 0,
                                        )) - f32::from(u8::from(
                                            keys.pressed(bevy::input::keyboard::KeyCode::KeyW)
                                                && (focus_state.1 & 4u16) == 0,
                                        ))
                                    } else {
                                        0.0
                                    },
                                )
                                .normalize_or_zero(),
                            )
                            .for_each(|move_dir: bevy::math::Vec2| {
                                std::iter::once(if move_dir == bevy::math::Vec2::ZERO {
                                    bevy::math::Vec2::ZERO
                                } else {
                                    prev_vel
                                        + (move_dir * 540.0 - prev_vel)
                                            * (1.0
                                                - (-(if prev_vel.dot(move_dir) < 0.0 {
                                                    32.0f32
                                                } else {
                                                    22.0f32
                                                }) * metrics_state.3)
                                                    .exp())
                                })
                                .for_each(|next_vel: bevy::math::Vec2| {
                                    *dot_state = std::option::Option::Some((
                                        dot_entity,
                                        (prev_pos + next_vel * metrics_state.3).clamp(
                                            bevy::math::Vec2::new(
                                                if prev_pos.x < 0.0 {
                                                    -1.0
                                                } else {
                                                    5.5f32.min(metrics_state.2.x * 0.5)
                                                },
                                                if prev_pos.x < 0.0 {
                                                    -1.0
                                                } else {
                                                    5.5f32.min(metrics_state.2.y * 0.5)
                                                },
                                            ),
                                            bevy::math::Vec2::new(
                                                (metrics_state.2.x - 5.5f32)
                                                    .max(metrics_state.2.x * 0.5),
                                                (metrics_state.2.y - 5.5f32)
                                                    .max(metrics_state.2.y * 0.5),
                                            ),
                                        ),
                                        next_vel,
                                        metrics_state.2,
                                    ));
                                });
                            });
                        },
                    );

                    if cfg!(debug_assertions) {
                        std::env::var("ARXUMBRA_PROBE")
                            .ok()
                            .into_iter()
                            .for_each(|probe: std::string::String| {
                                if probe == "workspace" || probe == "wizard" {
                                    app_state[0] = 2u8;
                                    app_state[1] = 0u8;
                                }
                                if probe == "wizard" {
                                    app_state[2] = 1u8;
                                    draft_state.0 = std::string::String::from("Arena Test");
                                    draft_state.1 = 10u16;
                                    draft_state.2 = true;
                                }
                            });
                    }
                    std::iter::once((
                        metrics_state.2.x,
                        metrics_state.2.y,
                        metrics_state.1,
                        bevy::math::Rect::new(18.0, 92.0, metrics_state.1 - 18.0, 140.0),
                        bevy::math::Rect::new(18.0, 148.0, metrics_state.1 - 18.0, 196.0),
                        bevy::math::Rect::new(
                            metrics_state.1 + 16.0,
                            16.0,
                            (metrics_state.2.x - 16.0).max(metrics_state.1 + 96.0),
                            (metrics_state.2.y - 16.0).max(96.0),
                        ),
                        bevy::math::Rect::from_center_size(
                            metrics_state.2 * 0.5,
                            bevy::math::Vec2::new(
                                (metrics_state.2.x * 0.42)
                                    .clamp(420.0, 620.0)
                                    .min((metrics_state.2.x - 48.0).max(200.0)),
                                (metrics_state.2.y * 0.62)
                                    .clamp(380.0, 560.0)
                                    .min((metrics_state.2.y - 48.0).max(200.0)),
                            ),
                        ),
                        (*dot_state)
                            .map(
                                |(_, pos, _, _): (
                                    bevy::ecs::entity::Entity,
                                    bevy::math::Vec2,
                                    bevy::math::Vec2,
                                    bevy::math::Vec2,
                                )| pos.max(bevy::math::Vec2::ZERO),
                            )
                            .unwrap_or(metrics_state.2 * 0.5),
                    ))
                    .for_each(
                        |(vw, vh, sw, row_one_rect, row_two_rect, panel_rect, modal_rect, dot_pos): (
                            f32,
                            f32,
                            f32,
                            bevy::math::Rect,
                            bevy::math::Rect,
                            bevy::math::Rect,
                            bevy::math::Rect,
                            bevy::math::Vec2,
                        )| {
                            std::iter::once(bevy::math::Rect::new(
                                panel_rect.min.x + 28.0,
                                122.0,
                                (panel_rect.min.x + 28.0 + 480.0)
                                    .min((panel_rect.max.x - 28.0).max(panel_rect.min.x + 200.0)),
                                273.0,
                            ))
                            .for_each(|card_rect: bevy::math::Rect| {
                                std::iter::once(bevy::math::Rect::new(
                                    card_rect.max.x - 18.0 - 148.0,
                                    213.0,
                                    card_rect.max.x - 18.0,
                                    257.0,
                                ))
                                .for_each(|launch_rect: bevy::math::Rect| {
                                    std::iter::once((
                                        app_state[0],
                                        app_state[1] == 0u8,
                                        app_state[2] != 0u8,
                                    ))
                                    .for_each(
                                        |(cur_mode, cur_menu, cur_modal): (u8, bool, bool)| {
                                    std::iter::once((
                                        modal_rect.min.x,
                                        modal_rect.min.y,
                                        modal_rect.max.x,
                                        modal_rect.max.y,
                                    ))
                                    .for_each(|(ml, mt, mr, mb): (f32, f32, f32, f32)| {
                                        std::iter::once((
                                            bevy::math::Rect::new(
                                                mr - 148.0,
                                                mb - 72.0,
                                                mr - 28.0,
                                                mb - 28.0,
                                            ),
                                            bevy::math::Rect::new(
                                                ml + 28.0,
                                                mb - 72.0,
                                                ml + 132.0,
                                                mb - 28.0,
                                            ),
                                            bevy::math::Rect::new(
                                                ml + 28.0,
                                                mt + 214.0,
                                                mr - 28.0,
                                                mt + 260.0,
                                            ),
                                            [
                                                bevy::math::Rect::from_center_size(
                                                    bevy::math::Vec2::new(ml + 62.0, mt + 150.0),
                                                    bevy::math::Vec2::splat(68.0),
                                                ),
                                                bevy::math::Rect::from_center_size(
                                                    bevy::math::Vec2::new(ml + 146.0, mt + 150.0),
                                                    bevy::math::Vec2::splat(68.0),
                                                ),
                                            ],
                                            [
                                                bevy::math::Rect::new(
                                                    ml + 28.0,
                                                    mt + 85.0,
                                                    mr - 28.0,
                                                    mt + 131.0,
                                                ),
                                                bevy::math::Rect::new(
                                                    ml + 28.0,
                                                    mt + 139.0,
                                                    mr - 28.0,
                                                    mt + 185.0,
                                                ),
                                                bevy::math::Rect::new(
                                                    ml + 28.0,
                                                    mt + 193.0,
                                                    mr - 28.0,
                                                    mt + 239.0,
                                                ),
                                            ],
                                            [
                                                bevy::math::Rect::new(
                                                    ml + 46.0,
                                                    mt + 247.0,
                                                    mr - 46.0,
                                                    mt + 287.0,
                                                ),
                                                bevy::math::Rect::new(
                                                    ml + 46.0,
                                                    mt + 295.0,
                                                    mr - 46.0,
                                                    mt + 335.0,
                                                ),
                                            ],
                                            [
                                                bevy::math::Rect::from_center_size(
                                                    bevy::math::Vec2::new(
                                                        panel_rect.max.x - 104.0,
                                                        486.0,
                                                    ),
                                                    bevy::math::Vec2::splat(32.0),
                                                ),
                                                bevy::math::Rect::from_center_size(
                                                    bevy::math::Vec2::new(
                                                        panel_rect.max.x - 64.0,
                                                        486.0,
                                                    ),
                                                    bevy::math::Vec2::splat(32.0),
                                                ),
                                            ],
                                        ))
                                        .for_each(
                                            |(
                                                next_rect,
                                                back_rect,
                                                field_rect,
                                                icon_rects,
                                                type_rects,
                                                dim_rects,
                                                scroll_rects,
                                            ): (
                                                bevy::math::Rect,
                                                bevy::math::Rect,
                                                bevy::math::Rect,
                                                [bevy::math::Rect; 2],
                                                [bevy::math::Rect; 3],
                                                [bevy::math::Rect; 2],
                                                [bevy::math::Rect; 2],
                                            )| {
                                                std::iter::once(
                                                    if !window.focused {
                                                        std::option::Option::None
                                                    } else if cur_modal {
                                                        [
                                                            (
                                                                5u8,
                                                                icon_rects[0],
                                                                16.0f32,
                                                                wizard_state.0 == 0u8,
                                                            ),
                                                            (
                                                                6u8,
                                                                icon_rects[1],
                                                                16.0f32,
                                                                wizard_state.0 == 0u8,
                                                            ),
                                                            (
                                                                7u8,
                                                                field_rect,
                                                                12.0f32,
                                                                wizard_state.0 == 0u8,
                                                            ),
                                                            (8u8, next_rect, 12.0f32, true),
                                                            (
                                                                14u8,
                                                                back_rect,
                                                                12.0f32,
                                                                wizard_state.0 == 1u8,
                                                            ),
                                                            (
                                                                9u8,
                                                                type_rects[0],
                                                                12.0f32,
                                                                wizard_state.0 == 1u8,
                                                            ),
                                                            (
                                                                10u8,
                                                                type_rects[1],
                                                                12.0f32,
                                                                wizard_state.0 == 1u8,
                                                            ),
                                                            (
                                                                11u8,
                                                                type_rects[2],
                                                                12.0f32,
                                                                wizard_state.0 == 1u8,
                                                            ),
                                                            (
                                                                12u8,
                                                                dim_rects[0],
                                                                12.0f32,
                                                                wizard_state.0 == 1u8
                                                                    && wizard_state.2 == 3u8,
                                                            ),
                                                            (
                                                                13u8,
                                                                dim_rects[1],
                                                                12.0f32,
                                                                wizard_state.0 == 1u8
                                                                    && wizard_state.2 == 3u8,
                                                            ),
                                                            (
                                                                4u8,
                                                                modal_rect,
                                                                14.0f32,
                                                                !modal_rect.contains(dot_pos)
                                                                    || ((dot_pos.x
                                                                        - modal_rect.center().x)
                                                                        .abs()
                                                                        - (modal_rect.width()
                                                                            * 0.5
                                                                            - 14.0))
                                                                        .max(0.0)
                                                                        .hypot(
                                                                            ((dot_pos.y
                                                                                - modal_rect
                                                                                    .center()
                                                                                    .y)
                                                                                .abs()
                                                                                - (modal_rect
                                                                                    .height()
                                                                                    * 0.5
                                                                                    - 14.0))
                                                                                .max(0.0),
                                                                        )
                                                                        > 14.0,
                                                            ),
                                                        ]
                                                        .into_iter()
                                                        .filter(
                                                            |&(
                                                                _code,
                                                                rect,
                                                                radius,
                                                                enabled,
                                                            ): &(
                                                                u8,
                                                                bevy::math::Rect,
                                                                f32,
                                                                bool,
                                                            )| {
                                                                enabled
                                                                    && rect.contains(dot_pos)
                                                                    && ((dot_pos.x
                                                                        - rect.center().x)
                                                                        .abs()
                                                                        - (rect.width() * 0.5
                                                                            - radius))
                                                                        .max(0.0)
                                                                        .hypot(
                                                                            ((dot_pos.y
                                                                                - rect.center().y)
                                                                                .abs()
                                                                                - (rect.height()
                                                                                    * 0.5
                                                                                    - radius))
                                                                                .max(0.0),
                                                                        )
                                                                        <= radius
                                                            },
                                                        )
                                                        .map(
                                                            |(code, _r, _rad, _e): (
                                                                u8,
                                                                bevy::math::Rect,
                                                                f32,
                                                                bool,
                                                            )| code,
                                                        )
                                                        .next()
                                                    } else if cur_mode == 2u8 {
                                                        [
                                                            (
                                                                3u8,
                                                                row_two_rect,
                                                                9.0f32,
                                                                cur_menu,
                                                            ),
                                                            (
                                                                15u8,
                                                                scroll_rects[0],
                                                                10.0f32,
                                                                catalog_state.0.len() > 6usize,
                                                            ),
                                                            (
                                                                16u8,
                                                                scroll_rects[1],
                                                                10.0f32,
                                                                catalog_state.0.len() > 6usize,
                                                            ),
                                                        ]
                                                        .into_iter()
                                                        .filter(
                                                            |&(
                                                                _code,
                                                                rect,
                                                                radius,
                                                                enabled,
                                                            ): &(
                                                                u8,
                                                                bevy::math::Rect,
                                                                f32,
                                                                bool,
                                                            )| {
                                                                enabled
                                                                    && rect.contains(dot_pos)
                                                                    && ((dot_pos.x
                                                                        - rect.center().x)
                                                                        .abs()
                                                                        - (rect.width() * 0.5
                                                                            - radius))
                                                                        .max(0.0)
                                                                        .hypot(
                                                                            ((dot_pos.y
                                                                                - rect.center().y)
                                                                                .abs()
                                                                                - (rect.height()
                                                                                    * 0.5
                                                                                    - radius))
                                                                                .max(0.0),
                                                                        )
                                                                        <= radius
                                                            },
                                                        )
                                                        .map(
                                                            |(code, _r, _rad, _e): (
                                                                u8,
                                                                bevy::math::Rect,
                                                                f32,
                                                                bool,
                                                            )| code,
                                                        )
                                                        .next()
                                                    } else {
                                                        [
                                                            (0u8, row_one_rect, 9.0f32, cur_menu),
                                                            (1u8, row_two_rect, 9.0f32, cur_menu),
                                                            (
                                                                2u8,
                                                                launch_rect,
                                                                9.0f32,
                                                                cur_mode == 1u8
                                                                    && metrics_state.0 > 0.0,
                                                            ),
                                                        ]
                                                        .into_iter()
                                                        .filter(
                                                            |&(
                                                                _code,
                                                                rect,
                                                                radius,
                                                                enabled,
                                                            ): &(
                                                                u8,
                                                                bevy::math::Rect,
                                                                f32,
                                                                bool,
                                                            )| {
                                                                enabled
                                                                    && rect.contains(dot_pos)
                                                                    && ((dot_pos.x
                                                                        - rect.center().x)
                                                                        .abs()
                                                                        - (rect.width() * 0.5
                                                                            - radius))
                                                                        .max(0.0)
                                                                        .hypot(
                                                                            ((dot_pos.y
                                                                                - rect.center().y)
                                                                                .abs()
                                                                                - (rect.height()
                                                                                    * 0.5
                                                                                    - radius))
                                                                                .max(0.0),
                                                                        )
                                                                        <= radius
                                                            },
                                                        )
                                                        .map(
                                                            |(code, _r, _rad, _e): (
                                                                u8,
                                                                bevy::math::Rect,
                                                                f32,
                                                                bool,
                                                            )| code,
                                                        )
                                                        .next()
                                                    },
                                                )
                                                .for_each(
                                                    |hit_target: std::option::Option<u8>| {
                                                        std::iter::once(match hit_target {
                                                            std::option::Option::Some(
                                                                0u8 | 1u8 | 3u8,
                                                            ) if !(bevy::math::Rect::new(
                                                                0.0, 0.0, sw, vh,
                                                            )
                                                            .contains(dot_pos)
                                                                && (dot_pos.x - (sw - 14.0))
                                                                    .max(0.0)
                                                                    .hypot(
                                                                        ((dot_pos.y
                                                                            - vh * 0.5)
                                                                            .abs()
                                                                            - (vh * 0.5 - 14.0))
                                                                            .max(0.0),
                                                                    )
                                                                    <= 14.0) =>
                                                            {
                                                                std::option::Option::None
                                                            }
                                                            other => other,
                                                        })
                                                        .for_each(
                                                            |hit_target: std::option::Option<u8>| {
                                                                std::iter::once(
                                                                    if !escape_state.3
                                                                        && focus_state.3
                                                                        && tap_state.1
                                                                        && (focus_state.1 & 64u16)
                                                                            == 0
                                                                    {
                                                                        match hit_target {
                                                                            std::option::Option::Some(0u8) => (0u8, cur_menu, false),
                                                                            std::option::Option::Some(1u8) => (1u8, cur_menu, false),
                                                                            std::option::Option::Some(2u8) if cur_mode == 1u8 && !cur_modal => {
                                                                                *escape_state = (
                                                                                    std::option::Option::None,
                                                                                    false,
                                                                                    keys.pressed(
                                                                                        bevy::input::keyboard::KeyCode::Escape,
                                                                                    ),
                                                                                    escape_state.3,
                                                                                );
                                                                                (2u8, cur_menu, false)
                                                                            }
                                                                            std::option::Option::Some(3u8) if cur_mode == 2u8 && !cur_modal => {
                                                                                *wizard_state = (0u8, 0u8, 0u8, 0u8);
                                                                                *draft_state = (
                                                                                    std::string::String::new(),
                                                                                    0u16,
                                                                                    false,
                                                                                );
                                                                                *feedback_state = (0u8, feedback_state.1, feedback_state.2);
                                                                                (2u8, cur_menu, true)
                                                                            }
                                                                            std::option::Option::Some(4u8) if cur_modal => {
                                                                                *draft_state = (
                                                                                    std::string::String::new(),
                                                                                    0u16,
                                                                                    false,
                                                                                );
                                                                                *feedback_state = (0u8, feedback_state.1, feedback_state.2);
                                                                                (2u8, cur_menu, false)
                                                                            }
                                                                            std::option::Option::Some(5u8) if cur_modal => {
                                                                                wizard_state.1 = 0u8;
                                                                                feedback_state.0 = 0u8;
                                                                                (cur_mode, cur_menu, cur_modal)
                                                                            }
                                                                            std::option::Option::Some(6u8) if cur_modal => {
                                                                                wizard_state.1 = 1u8;
                                                                                feedback_state.0 = 0u8;
                                                                                (cur_mode, cur_menu, cur_modal)
                                                                            }
                                                                            std::option::Option::Some(7u8) if cur_modal && wizard_state.0 == 0u8 => {
                                                                                draft_state.2 = true;
                                                                                (cur_mode, cur_menu, cur_modal)
                                                                            }
                                                                            std::option::Option::Some(9u8) if cur_modal && wizard_state.0 == 1u8 => {
                                                                                wizard_state.2 = 1u8;
                                                                                wizard_state.3 = 0u8;
                                                                                feedback_state.0 = 0u8;
                                                                                (cur_mode, cur_menu, cur_modal)
                                                                            }
                                                                            std::option::Option::Some(10u8) if cur_modal && wizard_state.0 == 1u8 => {
                                                                                wizard_state.2 = 2u8;
                                                                                wizard_state.3 = 0u8;
                                                                                feedback_state.0 = 0u8;
                                                                                (cur_mode, cur_menu, cur_modal)
                                                                            }
                                                                            std::option::Option::Some(11u8) if cur_modal && wizard_state.0 == 1u8 => {
                                                                                wizard_state.2 = 3u8;
                                                                                feedback_state.0 = 0u8;
                                                                                (cur_mode, cur_menu, cur_modal)
                                                                            }
                                                                            std::option::Option::Some(12u8) if cur_modal && wizard_state.0 == 1u8 => {
                                                                                wizard_state.3 = 1u8;
                                                                                feedback_state.0 = 0u8;
                                                                                (cur_mode, cur_menu, cur_modal)
                                                                            }
                                                                            std::option::Option::Some(13u8) if cur_modal && wizard_state.0 == 1u8 => {
                                                                                wizard_state.3 = 2u8;
                                                                                feedback_state.0 = 0u8;
                                                                                (cur_mode, cur_menu, cur_modal)
                                                                            }
                                                                            std::option::Option::Some(14u8) if cur_modal && wizard_state.0 == 1u8 => {
                                                                                wizard_state.0 = 0u8;
                                                                                feedback_state.0 = 0u8;
                                                                                (cur_mode, cur_menu, cur_modal)
                                                                            }
                                                                            std::option::Option::Some(15u8) if cur_mode == 2u8 && !cur_modal => {
                                                                                list_state.0 = list_state.0.saturating_sub(1);
                                                                                (cur_mode, cur_menu, cur_modal)
                                                                            }
                                                                            std::option::Option::Some(16u8) if cur_mode == 2u8 && !cur_modal => {
                                                                                list_state.0 = (list_state.0 + 1).min(catalog_state.0.len().saturating_sub(6));
                                                                                (cur_mode, cur_menu, cur_modal)
                                                                            }
                                                                            _ => (cur_mode, cur_menu, cur_modal),
                                                                        }
                                                                    } else {
                                                                        (cur_mode, cur_menu, cur_modal)
                                                                    },
                                                                )
                                                                .for_each(
                                                                    |(next_mode, next_menu, next_modal): (u8, bool, bool)| {
                                                                        if !escape_state.3
                                                                            && focus_state.3
                                                                            && tap_state.1
                                                                            && (focus_state.1 & 64u16) == 0
                                                                            && hit_target != std::option::Option::Some(7u8)
                                                                        {
                                                                            draft_state.2 = false;
                                                                        }
                                                                        *app_state = [
                                                                            next_mode,
                                                                            u8::from(!next_menu),
                                                                            u8::from(next_modal),
                                                                            if !window.focused {
                                                                                0u8
                                                                            } else {
                                                                                hit_target.map_or(0u8, |code: u8| code + 1)
                                                                            },
                                                                        ];
                                                                        std::iter::once(
                                                                            if !escape_state.3
                                                                                && focus_state.3
                                                                                && tap_state.1
                                                                                && (focus_state.1 & 64u16) == 0
                                                                            {
                                                                                match hit_target {
                                                                                    std::option::Option::Some(8u8) if cur_modal && wizard_state.0 == 0u8 => 1u8,
                                                                                    std::option::Option::Some(8u8) if cur_modal && wizard_state.0 == 1u8 => 2u8,
                                                                                    _ => 0u8,
                                                                                }
                                                                            } else {
                                                                                0u8
                                                                            },
                                                                        )
                                                                        .for_each(|next_intent: u8| {
                                                                            if next_intent == 1u8 {
                                                                                std::iter::once(draft_state.0.trim())
                                                                                    .for_each(|trimmed: &str| {
                                                                                        feedback_state.0 = if trimmed.is_empty() {
                                                                                            1u8
                                                                                        } else if trimmed.chars().count() > 64usize {
                                                                                            2u8
                                                                                        } else if trimmed.chars().any(|glyph: char| glyph.is_control()) {
                                                                                            3u8
                                                                                        } else if catalog_state.0.iter().any(|(_id, _icon, name, _t, _d): &(u64, u8, std::string::String, u8, u8)| name == trimmed) {
                                                                                            4u8
                                                                                        } else {
                                                                                            0u8
                                                                                        };
                                                                                        if feedback_state.0 == 0u8 {
                                                                                            wizard_state.0 = 1u8;
                                                                                        }
                                                                                    });
                                                                            } else if next_intent == 2u8 {
                                                                                if wizard_state.2 == 0u8 {
                                                                                    feedback_state.0 = 5u8;
                                                                                } else if wizard_state.2 == 3u8 && wizard_state.3 == 0u8 {
                                                                                    feedback_state.0 = 6u8;
                                                                                } else if feedback_state.1 != 0u8 {
                                                                                    feedback_state.0 = 7u8;
                                                                                } else {
                                                                                    feedback_state.2 = 1u8;
                                                                                }
                                                                            }
                                                                        });
                                                                    },
                                                                );
                                                            },
                                                        );
                                                    },
                                                );
                                            },
                                        );
                                    });
                                    });
                                    std::iter::once(
                                        if cfg!(target_os = "linux") {
                                            std::env::var("XDG_DATA_HOME").ok().filter(|root: &std::string::String| std::path::Path::new(root).is_absolute()).map(|root: std::string::String| std::path::PathBuf::from(root)).or_else(|| std::env::var("HOME").ok().map(|home: std::string::String| std::path::PathBuf::from(home).join(".local").join("share")))
                                        } else if cfg!(target_os = "macos") {
                                            std::env::var("HOME").ok().map(|home: std::string::String| std::path::PathBuf::from(home).join("Library").join("Application Support"))
                                        } else {
                                            std::env::var("APPDATA").ok().map(|root: std::string::String| std::path::PathBuf::from(root)).or_else(|| std::env::var("USERPROFILE").ok().map(|home: std::string::String| std::path::PathBuf::from(home).join("AppData").join("Roaming")))
                                        },
                                    )
                                    .for_each(|data_root: std::option::Option<std::path::PathBuf>| {
                                        if app_state[0] != 2u8 {
                                            catalog_state.1 = app_state[0];
                                        }
                                        if data_root.is_none() && app_state[0] == 2u8 {
                                            feedback_state.1 = 1u8;
                                        }
                                        data_root.into_iter().map(|root: std::path::PathBuf| root.join("Subsystem-for-Arxumbra")).for_each(|dir: std::path::PathBuf| {
                                            if app_state[0] == 2u8 && catalog_state.1 != 2u8 {
                                                catalog_state.1 = 2u8;
                                                list_state.0 = 0usize;
                                                std::fs::remove_file(dir.join("workspace-catalog.tmp")).ok();
                                                match std::fs::read_to_string(dir.join("workspace-catalog")) {
                                                    std::result::Result::Ok(stored) => {
                                                        std::iter::once(stored.lines().collect::<std::vec::Vec<&str>>()).for_each(|lines: std::vec::Vec<&str>| {
                                                            feedback_state.1 = if lines.first() != std::option::Option::Some(&"arxumbra-workspace-catalog v1") {
                                                                if lines.first().map_or(false, |head: &&str| head.starts_with("arxumbra-workspace-catalog v")) { 3u8 } else { 2u8 }
                                                            } else if lines.len() < 2usize || lines[1usize].parse::<usize>().ok() != std::option::Option::Some(lines.len() - 2usize) {
                                                                2u8
                                                            } else {
                                                                lines.iter().skip(2usize).map(|line: &&str| {
                                                                    std::iter::once(line.split('|').collect::<std::vec::Vec<&str>>()).map(|fields: std::vec::Vec<&str>| {
                                                                        if fields.len() != 5usize || fields[2usize].is_empty() || fields[2usize].len() % 2usize != 0 {
                                                                            std::option::Option::None
                                                                        } else {
                                                                            fields[0usize].parse::<u64>().ok().and_then(|id: u64| fields[1usize].parse::<u8>().ok().filter(|icon: &u8| *icon <= 1u8).map(|icon: u8| (id, icon))).and_then(|(id, icon): (u64, u8)| fields[2usize].as_bytes().chunks(2usize).map(|pair: &[u8]| std::str::from_utf8(pair).ok().and_then(|hex: &str| u8::from_str_radix(hex, 16u32).ok())).collect::<std::option::Option<std::vec::Vec<u8>>>().and_then(|bytes: std::vec::Vec<u8>| std::string::String::from_utf8(bytes).ok()).filter(|name: &std::string::String| !name.is_empty() && name.chars().count() <= 64usize && !name.chars().any(|glyph: char| glyph.is_control())).map(|name: std::string::String| (id, icon, name))).and_then(|(id, icon, name): (u64, u8, std::string::String)| fields[3usize].parse::<u8>().ok().filter(|kind: &u8| (1u8..=3u8).contains(kind)).map(|kind: u8| (id, icon, name, kind))).and_then(|(id, icon, name, kind): (u64, u8, std::string::String, u8)| fields[4usize].parse::<u8>().ok().filter(|dim: &u8| if kind == 3u8 { (1u8..=2u8).contains(dim) } else { *dim == 0u8 }).map(|dim: u8| (id, icon, name, kind, dim)))
                                                                        }
                                                                    }).next().flatten()
                                                                }).collect::<std::option::Option<std::vec::Vec<(u64, u8, std::string::String, u8, u8)>>>().map_or(2u8, |records: std::vec::Vec<(u64, u8, std::string::String, u8, u8)>| if records.len() > 4096usize { 2u8 } else { catalog_state.0 = records; 0u8 })
                                                            };
                                                        });
                                                    }
                                                    std::result::Result::Err(unreadable) => {
                                                        feedback_state.1 = if unreadable.kind() == std::io::ErrorKind::NotFound { catalog_state.0.clear(); 0u8 } else { 5u8 };
                                                    }
                                                }
                                            }
                                            if feedback_state.2 == 1u8 {
                                                feedback_state.2 = 0u8;
                                                std::iter::once((draft_state.0.trim().to_string(), catalog_state.0.iter().map(|(id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *id).max().unwrap_or(0u64).saturating_add(1u64))).for_each(|(trimmed, new_id): (std::string::String, u64)| {
                                                    std::iter::once(catalog_state.0.iter().cloned().chain(std::iter::once((new_id, wizard_state.1, trimmed, wizard_state.2, wizard_state.3))).collect::<std::vec::Vec<(u64, u8, std::string::String, u8, u8)>>()).map(|candidate: std::vec::Vec<(u64, u8, std::string::String, u8, u8)>| (std::format!("arxumbra-workspace-catalog v1\n{}\n", candidate.len()) + &candidate.iter().map(|(id, icon, name, kind, dim): &(u64, u8, std::string::String, u8, u8)| std::format!("{}|{}|{}|{}|{}", id, icon, name.as_bytes().iter().map(|byte: &u8| std::format!("{:02x}", byte)).collect::<std::string::String>(), kind, dim)).collect::<std::vec::Vec<std::string::String>>().join("\n") + "\n", candidate)).for_each(|(payload, candidate): (std::string::String, std::vec::Vec<(u64, u8, std::string::String, u8, u8)>)| {
                                                        match std::iter::once(std::fs::create_dir_all(&dir).and_then(|()| std::fs::OpenOptions::new().write(true).create_new(true).open(dir.join("workspace-catalog.lock"))).and_then(|held: std::fs::File| std::fs::write(dir.join("workspace-catalog.tmp"), payload.as_bytes()).and_then(|()| std::fs::File::open(dir.join("workspace-catalog.tmp")).and_then(|flushed: std::fs::File| flushed.sync_all())).and_then(|()| std::fs::rename(dir.join("workspace-catalog.tmp"), dir.join("workspace-catalog"))).map(|()| held))).map(|outcome: std::result::Result<std::fs::File, std::io::Error>| match outcome {
                                                            std::result::Result::Ok(held) => {
                                                                std::fs::remove_file(dir.join("workspace-catalog.lock")).ok();
                                                                std::mem::drop(held);
                                                                true
                                                            }
                                                            std::result::Result::Err(_failure) => {
                                                                std::fs::remove_file(dir.join("workspace-catalog.tmp")).ok();
                                                                std::fs::remove_file(dir.join("workspace-catalog.lock")).ok();
                                                                false
                                                            }
                                                        })
                                                        .next()
                                                    {
                                                            std::option::Option::Some(true) => {
                                                                catalog_state.0 = candidate;
                                                                list_state.1 = new_id;
                                                                app_state[2usize] = 0u8;
                                                                *draft_state = (std::string::String::new(), 0u16, false);
                                                                feedback_state.0 = 0u8;
                                                            }
                                                            _ => {
                                                                feedback_state.0 = 8u8;
                                                            }
                                                        }
                                                    });
                                                });
                                            }
                                        });
                                    });
                                    std::iter::once((
                                        app_state[0],
                                        app_state[1] == 0u8,
                                        app_state[2] != 0u8,
                                        app_state[3],
                                    ))
                                    .for_each(
                                        |(final_mode, final_menu, final_modal, hovered): (
                                            u8,
                                            bool,
                                            bool,
                                            u8,
                                        )| {
                                            [0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8, 16u8].into_iter().for_each(
                                                |idx: u8| {
                                                    std::iter::once((
                                                        usize::from(idx),
                                                        f32::from(u8::from(
                                                            hovered == idx.saturating_add(1),
                                                        )),
                                                        f32::from(u8::from(
                                                            (hovered == idx.saturating_add(1))
                                                                && focus_state.3
                                                                && keys.pressed(
                                                                    bevy::input::keyboard::KeyCode::KeyE,
                                                                )
                                                                && (focus_state.1 & 64u16) == 0,
                                                        )),
                                                    ))
                                                    .for_each(
                                                        |(slot, h_target, p_target): (
                                                            usize,
                                                            f32,
                                                            f32,
                                                        )| {
                                                            anim_state.0[slot] = if (h_target
                                                                - anim_state.0[slot])
                                                                .abs()
                                                                <= 0.002
                                                            {
                                                                h_target
                                                            } else {
                                                                (anim_state.0[slot]
                                                                    + (h_target
                                                                        - anim_state.0[slot])
                                                                        * (1.0
                                                                            - (-20.0f32
                                                                                * metrics_state.3)
                                                                                .exp())
                                                                    + (h_target
                                                                        - anim_state.0[slot])
                                                                        .signum()
                                                                        * 1.6f32
                                                                        * metrics_state.3)
                                                                    .clamp(
                                                                        anim_state.0[slot]
                                                                            .min(h_target),
                                                                        anim_state.0[slot]
                                                                            .max(h_target),
                                                                    )
                                                            };
                                                            anim_state.1[slot] = if (p_target
                                                                - anim_state.1[slot])
                                                                .abs()
                                                                <= 0.002
                                                            {
                                                                p_target
                                                            } else {
                                                                (anim_state.1[slot]
                                                                    + (p_target
                                                                        - anim_state.1[slot])
                                                                        * (1.0
                                                                            - (-28.0f32
                                                                                * metrics_state.3)
                                                                                .exp())
                                                                    + (p_target
                                                                        - anim_state.1[slot])
                                                                        .signum()
                                                                        * 2.4f32
                                                                        * metrics_state.3)
                                                                    .clamp(
                                                                        anim_state.1[slot]
                                                                            .min(p_target),
                                                                        anim_state.1[slot]
                                                                            .max(p_target),
                                                                    )
                                                            };
                                                        },
                                                    );
                                                },
                                            );

                                            [
                                                (
                                                    0usize,
                                                    1.0f32,
                                                    18.0f32,
                                                ),
                                                (
                                                    1usize,
                                                    1.0f32,
                                                    16.0f32,
                                                ),
                                                (
                                                    2usize,
                                                    if final_mode == 2u8 { 1.0f32 } else { 0.0f32 },
                                                    18.0f32,
                                                ),
                                                (
                                                    3usize,
                                                    if final_mode == 2u8 && final_modal {
                                                        1.0f32
                                                    } else {
                                                        0.0f32
                                                    },
                                                    20.0f32,
                                                ),
                                                (
                                                    4usize,
                                                    if final_menu { 1.0f32 } else { 0.0f32 },
                                                    20.0f32,
                                                ),
                                                (
                                                    5usize,
                                                    if final_mode == 2u8
                                                        && final_modal
                                                        && wizard_state.0 == 1u8
                                                    {
                                                        1.0f32
                                                    } else {
                                                        0.0f32
                                                    },
                                                    24.0f32,
                                                ),
                                                (
                                                    6usize,
                                                    if final_mode == 2u8
                                                        && final_modal
                                                        && wizard_state.0 == 1u8
                                                        && wizard_state.2 == 3u8
                                                    {
                                                        1.0f32
                                                    } else {
                                                        0.0f32
                                                    },
                                                    24.0f32,
                                                ),
                                            ]
                                            .into_iter()
                                            .for_each(|(idx, target, rate): (usize, f32, f32)| {
                                                anim_state.2[idx] =
                                                    if (target - anim_state.2[idx]).abs() <= 0.002
                                                    {
                                                        target
                                                    } else {
                                                        (anim_state.2[idx]
                                                            + (target - anim_state.2[idx])
                                                                * (1.0
                                                                    - (-rate * metrics_state.3)
                                                                        .exp())
                                                            + (target - anim_state.2[idx]).signum()
                                                                * 1.6f32
                                                                * metrics_state.3)
                                                            .clamp(
                                                                anim_state.2[idx].min(target),
                                                                anim_state.2[idx].max(target),
                                                            )
                                                    };
                                            });

                                            ui_entities.as_ref().copied().into_iter().for_each(
                                                |e: [bevy::ecs::entity::Entity; 110]| {
                                                    std::iter::once((
                                                        anim_state.2[0],
                                                        anim_state.2[1],
                                                        anim_state.2[2],
                                                        anim_state.2[3],
                                                        anim_state.2[4],
                                                        0.97f32 + 0.03f32 * anim_state.2[3],
                                                    ))
                                                    .for_each(
                                                        |(
                                                            nav_t,
                                                            panel_t,
                                                            ws_t,
                                                            modal_t,
                                                            menu_t,
                                                            m_scale,
                                                        ): (
                                                            f32,
                                                            f32,
                                                            f32,
                                                            f32,
                                                            f32,
                                                            f32,
                                                        )| {
                                                            [
                                                                (
                                                                    e[0],
                                                                    bevy::math::Vec2::new(sw, vh),
                                                                    bevy::math::Vec3::new(
                                                                        sw * 0.5 - vw * 0.5,
                                                                        0.0,
                                                                        1.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.180, 0.180, 0.180, menu_t,
                                                                    ),
                                                                    menu_t > 0.0,
                                                                ),
                                                                (
                                                                    e[1],
                                                                    bevy::math::Vec2::new(
                                                                        (sw - 1.0).max(1.0),
                                                                        (vh - 2.0).max(1.0),
                                                                    ),
                                                                    bevy::math::Vec3::new(
                                                                        (sw - 1.0).max(1.0) * 0.5
                                                                            - vw * 0.5,
                                                                        0.0,
                                                                        1.1,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.086, 0.086, 0.086, menu_t,
                                                                    ),
                                                                    menu_t > 0.0,
                                                                ),
                                                                (
                                                                    e[2],
                                                                    bevy::math::Vec2::splat(44.0),
                                                                    bevy::math::Vec3::new(
                                                                        40.0 - vw * 0.5,
                                                                        vh * 0.5 - 44.0,
                                                                        2.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.149, 0.149, 0.149, menu_t,
                                                                    ),
                                                                    menu_t > 0.0,
                                                                ),
                                                                (
                                                                    e[3],
                                                                    bevy::math::Vec2::splat(26.0),
                                                                    bevy::math::Vec3::new(
                                                                        40.0 - vw * 0.5,
                                                                        vh * 0.5 - 44.0,
                                                                        3.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.937,
                                                                        0.937,
                                                                        0.937,
                                                                        menu_t * (1.0 - ws_t),
                                                                    ),
                                                                    menu_t > 0.0 && ws_t < 1.0,
                                                                ),
                                                                (
                                                                    e[4],
                                                                    bevy::math::Vec2::splat(26.0),
                                                                    bevy::math::Vec3::new(
                                                                        40.0 - vw * 0.5,
                                                                        vh * 0.5 - 44.0,
                                                                        3.1,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.937,
                                                                        0.937,
                                                                        0.937,
                                                                        menu_t * ws_t,
                                                                    ),
                                                                    menu_t > 0.0 && ws_t > 0.0,
                                                                ),
                                                                (
                                                                    e[7],
                                                                    bevy::math::Vec2::new(
                                                                        (sw - 36.0).max(1.0),
                                                                        1.0,
                                                                    ),
                                                                    bevy::math::Vec3::new(
                                                                        sw * 0.5 - vw * 0.5,
                                                                        vh * 0.5 - 78.5,
                                                                        2.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.176, 0.176, 0.176, menu_t,
                                                                    ),
                                                                    menu_t > 0.0,
                                                                ),
                                                                (
                                                                    e[8],
                                                                    row_one_rect.size(),
                                                                    bevy::math::Vec3::new(
                                                                        row_one_rect.center().x
                                                                            - vw * 0.5,
                                                                        vh * 0.5
                                                                            - row_one_rect
                                                                                .center()
                                                                                .y,
                                                                        2.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        (0.125
                                                                            + 0.044
                                                                                * anim_state.0[0]
                                                                            - 0.025
                                                                                * anim_state.1[0])
                                                                            * nav_t
                                                                            + (0.865
                                                                                + 0.035
                                                                                    * anim_state.0
                                                                                        [0]
                                                                                - 0.045
                                                                                    * anim_state.1
                                                                                        [0])
                                                                                * (1.0 - nav_t),
                                                                        (0.125
                                                                            + 0.044
                                                                                * anim_state.0[0]
                                                                            - 0.025
                                                                                * anim_state.1[0])
                                                                            * nav_t
                                                                            + (0.865
                                                                                + 0.035
                                                                                    * anim_state.0
                                                                                        [0]
                                                                                - 0.045
                                                                                    * anim_state.1
                                                                                        [0])
                                                                                * (1.0 - nav_t),
                                                                        (0.125
                                                                            + 0.044
                                                                                * anim_state.0[0]
                                                                            - 0.025
                                                                                * anim_state.1[0])
                                                                            * nav_t
                                                                            + (0.865
                                                                                + 0.035
                                                                                    * anim_state.0
                                                                                        [0]
                                                                                - 0.045
                                                                                    * anim_state.1
                                                                                        [0])
                                                                                * (1.0 - nav_t),
                                                                        menu_t * (1.0 - ws_t),
                                                                    ),
                                                                    menu_t > 0.0 && ws_t < 1.0,
                                                                ),
                                                                (
                                                                    e[9],
                                                                    row_one_rect.size(),
                                                                    bevy::math::Vec3::new(
                                                                        row_one_rect.center().x
                                                                            - vw * 0.5,
                                                                        vh * 0.5
                                                                            - row_one_rect
                                                                                .center()
                                                                                .y,
                                                                        2.5,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.520,
                                                                        0.520,
                                                                        0.520,
                                                                        anim_state.0[0]
                                                                            * menu_t
                                                                            * (1.0 - ws_t),
                                                                    ),
                                                                    menu_t > 0.0
                                                                        && ws_t < 1.0
                                                                        && anim_state.0[0] > 0.0,
                                                                ),
                                                                (
                                                                    e[10],
                                                                    bevy::math::Vec2::splat(36.0),
                                                                    bevy::math::Vec3::new(
                                                                        42.0 - vw * 0.5,
                                                                        vh * 0.5 - 116.0,
                                                                        3.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.176
                                                                            + 0.030
                                                                                * anim_state.0[0]
                                                                            - 0.020
                                                                                * anim_state.1[0]
                                                                            - 0.015 * (1.0 - nav_t),
                                                                        0.176
                                                                            + 0.030
                                                                                * anim_state.0[0]
                                                                            - 0.020
                                                                                * anim_state.1[0]
                                                                            - 0.015 * (1.0 - nav_t),
                                                                        0.176
                                                                            + 0.030
                                                                                * anim_state.0[0]
                                                                            - 0.020
                                                                                * anim_state.1[0]
                                                                            - 0.015 * (1.0 - nav_t),
                                                                        menu_t * (1.0 - ws_t),
                                                                    ),
                                                                    menu_t > 0.0 && ws_t < 1.0,
                                                                ),
                                                                (
                                                                    e[11],
                                                                    bevy::math::Vec2::splat(22.0),
                                                                    bevy::math::Vec3::new(
                                                                        42.0 - vw * 0.5,
                                                                        vh * 0.5 - 116.0,
                                                                        4.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        (0.780
                                                                            + 0.157
                                                                                * anim_state.0[0])
                                                                            * nav_t
                                                                            + 0.937 * (1.0 - nav_t),
                                                                        (0.780
                                                                            + 0.157
                                                                                * anim_state.0[0])
                                                                            * nav_t
                                                                            + 0.937 * (1.0 - nav_t),
                                                                        (0.780
                                                                            + 0.157
                                                                                * anim_state.0[0])
                                                                            * nav_t
                                                                            + 0.937 * (1.0 - nav_t),
                                                                        menu_t * (1.0 - ws_t),
                                                                    ),
                                                                    menu_t > 0.0 && ws_t < 1.0,
                                                                ),
                                                                (
                                                                    e[13],
                                                                    row_two_rect.size(),
                                                                    bevy::math::Vec3::new(
                                                                        row_two_rect.center().x
                                                                            - vw * 0.5,
                                                                        vh * 0.5
                                                                            - row_two_rect
                                                                                .center()
                                                                                .y,
                                                                        2.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        (0.125
                                                                            + 0.044
                                                                                * anim_state.0[1]
                                                                            - 0.025
                                                                                * anim_state.1[1])
                                                                            * (1.0 - nav_t)
                                                                            + (0.865
                                                                                + 0.035
                                                                                    * anim_state.0
                                                                                        [1]
                                                                                - 0.045
                                                                                    * anim_state.1
                                                                                        [1])
                                                                                * nav_t,
                                                                        (0.125
                                                                            + 0.044
                                                                                * anim_state.0[1]
                                                                            - 0.025
                                                                                * anim_state.1[1])
                                                                            * (1.0 - nav_t)
                                                                            + (0.865
                                                                                + 0.035
                                                                                    * anim_state.0
                                                                                        [1]
                                                                                - 0.045
                                                                                    * anim_state.1
                                                                                        [1])
                                                                                * nav_t,
                                                                        (0.125
                                                                            + 0.044
                                                                                * anim_state.0[1]
                                                                            - 0.025
                                                                                * anim_state.1[1])
                                                                            * (1.0 - nav_t)
                                                                            + (0.865
                                                                                + 0.035
                                                                                    * anim_state.0
                                                                                        [1]
                                                                                - 0.045
                                                                                    * anim_state.1
                                                                                        [1])
                                                                                * nav_t,
                                                                        menu_t * (1.0 - ws_t),
                                                                    ),
                                                                    menu_t > 0.0 && ws_t < 1.0,
                                                                ),
                                                                (
                                                                    e[14],
                                                                    row_two_rect.size(),
                                                                    bevy::math::Vec3::new(
                                                                        row_two_rect.center().x
                                                                            - vw * 0.5,
                                                                        vh * 0.5
                                                                            - row_two_rect
                                                                                .center()
                                                                                .y,
                                                                        2.5,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.520,
                                                                        0.520,
                                                                        0.520,
                                                                        anim_state.0[1]
                                                                            * menu_t
                                                                            * (1.0 - ws_t),
                                                                    ),
                                                                    menu_t > 0.0
                                                                        && ws_t < 1.0
                                                                        && anim_state.0[1] > 0.0,
                                                                ),
                                                                (
                                                                    e[15],
                                                                    bevy::math::Vec2::splat(36.0),
                                                                    bevy::math::Vec3::new(
                                                                        42.0 - vw * 0.5,
                                                                        vh * 0.5 - 172.0,
                                                                        3.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.176
                                                                            + 0.030
                                                                                * anim_state.0[1]
                                                                            - 0.020
                                                                                * anim_state.1[1]
                                                                            - 0.015 * nav_t,
                                                                        0.176
                                                                            + 0.030
                                                                                * anim_state.0[1]
                                                                            - 0.020
                                                                                * anim_state.1[1]
                                                                            - 0.015 * nav_t,
                                                                        0.176
                                                                            + 0.030
                                                                                * anim_state.0[1]
                                                                            - 0.020
                                                                                * anim_state.1[1]
                                                                            - 0.015 * nav_t,
                                                                        menu_t * (1.0 - ws_t),
                                                                    ),
                                                                    menu_t > 0.0 && ws_t < 1.0,
                                                                ),
                                                                (
                                                                    e[16],
                                                                    bevy::math::Vec2::splat(22.0),
                                                                    bevy::math::Vec3::new(
                                                                        42.0 - vw * 0.5,
                                                                        vh * 0.5 - 172.0,
                                                                        4.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        (0.780
                                                                            + 0.157
                                                                                * anim_state.0[1])
                                                                            * (1.0 - nav_t)
                                                                            + 0.937 * nav_t,
                                                                        (0.780
                                                                            + 0.157
                                                                                * anim_state.0[1])
                                                                            * (1.0 - nav_t)
                                                                            + 0.937 * nav_t,
                                                                        (0.780
                                                                            + 0.157
                                                                                * anim_state.0[1])
                                                                            * (1.0 - nav_t)
                                                                            + 0.937 * nav_t,
                                                                        menu_t * (1.0 - ws_t),
                                                                    ),
                                                                    menu_t > 0.0 && ws_t < 1.0,
                                                                ),
                                                                (
                                                                    e[18],
                                                                    row_one_rect.size(),
                                                                    bevy::math::Vec3::new(
                                                                        row_one_rect.center().x
                                                                            - vw * 0.5,
                                                                        vh * 0.5
                                                                            - row_one_rect
                                                                                .center()
                                                                                .y,
                                                                        2.1,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.145
                                                                            + 0.050
                                                                                * anim_state.0[3]
                                                                            - 0.030
                                                                                * anim_state.1[3],
                                                                        0.145
                                                                            + 0.050
                                                                                * anim_state.0[3]
                                                                            - 0.030
                                                                                * anim_state.1[3],
                                                                        0.145
                                                                            + 0.050
                                                                                * anim_state.0[3]
                                                                            - 0.030
                                                                                * anim_state.1[3],
                                                                        menu_t * ws_t,
                                                                    ),
                                                                    menu_t > 0.0 && ws_t > 0.0,
                                                                ),
                                                                (
                                                                    e[19],
                                                                    row_one_rect.size(),
                                                                    bevy::math::Vec3::new(
                                                                        row_one_rect.center().x
                                                                            - vw * 0.5,
                                                                        vh * 0.5
                                                                            - row_one_rect
                                                                                .center()
                                                                                .y,
                                                                        2.6,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.275
                                                                            + 0.280
                                                                                * anim_state.0[3],
                                                                        0.275
                                                                            + 0.280
                                                                                * anim_state.0[3],
                                                                        0.275
                                                                            + 0.280
                                                                                * anim_state.0[3],
                                                                        menu_t * ws_t,
                                                                    ),
                                                                    menu_t > 0.0 && ws_t > 0.0,
                                                                ),
                                                                (
                                                                    e[20],
                                                                    bevy::math::Vec2::splat(36.0),
                                                                    bevy::math::Vec3::new(
                                                                        42.0 - vw * 0.5,
                                                                        vh * 0.5 - 116.0,
                                                                        3.1,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.200
                                                                            + 0.045
                                                                                * anim_state.0[3]
                                                                            - 0.025
                                                                                * anim_state.1[3],
                                                                        0.200
                                                                            + 0.045
                                                                                * anim_state.0[3]
                                                                            - 0.025
                                                                                * anim_state.1[3],
                                                                        0.200
                                                                            + 0.045
                                                                                * anim_state.0[3]
                                                                            - 0.025
                                                                                * anim_state.1[3],
                                                                        menu_t * ws_t,
                                                                    ),
                                                                    menu_t > 0.0 && ws_t > 0.0,
                                                                ),
                                                                (
                                                                    e[21],
                                                                    bevy::math::Vec2::splat(20.0),
                                                                    bevy::math::Vec3::new(
                                                                        42.0 - vw * 0.5,
                                                                        vh * 0.5 - 116.0,
                                                                        4.1,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.880
                                                                            + 0.090
                                                                                * anim_state.0[3],
                                                                        0.880
                                                                            + 0.090
                                                                                * anim_state.0[3],
                                                                        0.880
                                                                            + 0.090
                                                                                * anim_state.0[3],
                                                                        menu_t * ws_t,
                                                                    ),
                                                                    menu_t > 0.0 && ws_t > 0.0,
                                                                ),
                                                                (
                                                                    e[23],
                                                                    panel_rect.size(),
                                                                    bevy::math::Vec3::new(
                                                                        panel_rect.center().x
                                                                            - vw * 0.5,
                                                                        vh * 0.5
                                                                            - panel_rect.center().y,
                                                                        1.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.180, 0.180, 0.180,
                                                                        panel_t,
                                                                    ),
                                                                    metrics_state.0 > 0.0
                                                                        && panel_t > 0.0,
                                                                ),
                                                                (
                                                                    e[24],
                                                                    bevy::math::Vec2::new(
                                                                        (panel_rect.width() - 2.0)
                                                                            .max(1.0),
                                                                        (panel_rect.height() - 2.0)
                                                                            .max(1.0),
                                                                    ),
                                                                    bevy::math::Vec3::new(
                                                                        panel_rect.center().x
                                                                            - vw * 0.5,
                                                                        vh * 0.5
                                                                            - panel_rect.center().y,
                                                                        1.1,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.063, 0.063, 0.063,
                                                                        panel_t,
                                                                    ),
                                                                    metrics_state.0 > 0.0
                                                                        && panel_t > 0.0,
                                                                ),
                                                                (
                                                                    e[25],
                                                                    bevy::math::Vec2::splat(44.0),
                                                                    bevy::math::Vec3::new(
                                                                        panel_rect.min.x + 50.0
                                                                            - vw * 0.5,
                                                                        vh * 0.5 - 60.0,
                                                                        2.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.133, 0.133, 0.133,
                                                                        panel_t,
                                                                    ),
                                                                    metrics_state.0 > 0.0
                                                                        && panel_t > 0.0,
                                                                ),
                                                                (
                                                                    e[26],
                                                                    bevy::math::Vec2::splat(25.0),
                                                                    bevy::math::Vec3::new(
                                                                        panel_rect.min.x + 50.0
                                                                            - vw * 0.5,
                                                                        vh * 0.5 - 60.0,
                                                                        3.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.937,
                                                                        0.937,
                                                                        0.937,
                                                                        panel_t * (1.0 - ws_t),
                                                                    ),
                                                                    metrics_state.0 > 0.0
                                                                        && panel_t > 0.0
                                                                        && ws_t < 1.0,
                                                                ),
                                                                (
                                                                    e[27],
                                                                    bevy::math::Vec2::splat(26.0),
                                                                    bevy::math::Vec3::new(
                                                                        panel_rect.min.x + 50.0
                                                                            - vw * 0.5,
                                                                        vh * 0.5 - 60.0,
                                                                        3.1,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.937,
                                                                        0.937,
                                                                        0.937,
                                                                        panel_t * ws_t,
                                                                    ),
                                                                    metrics_state.0 > 0.0
                                                                        && panel_t > 0.0
                                                                        && ws_t > 0.0,
                                                                ),
                                                                (
                                                                    e[30],
                                                                    bevy::math::Vec2::new(
                                                                        (panel_rect.width() - 56.0)
                                                                            .max(1.0),
                                                                        1.0,
                                                                    ),
                                                                    bevy::math::Vec3::new(
                                                                        panel_rect.center().x
                                                                            - vw * 0.5,
                                                                        vh * 0.5 - 98.5,
                                                                        2.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.176, 0.176, 0.176,
                                                                        panel_t,
                                                                    ),
                                                                    metrics_state.0 > 0.0
                                                                        && panel_t > 0.0,
                                                                ),
                                                                (
                                                                    e[31],
                                                                    card_rect.size(),
                                                                    bevy::math::Vec3::new(
                                                                        card_rect.center().x
                                                                            - vw * 0.5,
                                                                        vh * 0.5
                                                                            - card_rect.center().y,
                                                                        2.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.188,
                                                                        0.188,
                                                                        0.188,
                                                                        panel_t * (1.0 - ws_t),
                                                                    ),
                                                                    metrics_state.0 > 0.0
                                                                        && panel_t > 0.0
                                                                        && ws_t < 1.0,
                                                                ),
                                                                (
                                                                    e[32],
                                                                    bevy::math::Vec2::new(
                                                                        (card_rect.width() - 2.0)
                                                                            .max(1.0),
                                                                        (card_rect.height() - 2.0)
                                                                            .max(1.0),
                                                                    ),
                                                                    bevy::math::Vec3::new(
                                                                        card_rect.center().x
                                                                            - vw * 0.5,
                                                                        vh * 0.5
                                                                            - card_rect.center().y,
                                                                        2.1,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.090,
                                                                        0.090,
                                                                        0.090,
                                                                        panel_t * (1.0 - ws_t),
                                                                    ),
                                                                    metrics_state.0 > 0.0
                                                                        && panel_t > 0.0
                                                                        && ws_t < 1.0,
                                                                ),
                                                                (
                                                                    e[33],
                                                                    bevy::math::Vec2::splat(42.0),
                                                                    bevy::math::Vec3::new(
                                                                        card_rect.min.x + 39.0
                                                                            - vw * 0.5,
                                                                        vh * 0.5 - 161.0,
                                                                        3.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.157,
                                                                        0.157,
                                                                        0.157,
                                                                        panel_t * (1.0 - ws_t),
                                                                    ),
                                                                    metrics_state.0 > 0.0
                                                                        && panel_t > 0.0
                                                                        && ws_t < 1.0,
                                                                ),
                                                                (
                                                                    e[34],
                                                                    bevy::math::Vec2::splat(25.0),
                                                                    bevy::math::Vec3::new(
                                                                        card_rect.min.x + 39.0
                                                                            - vw * 0.5,
                                                                        vh * 0.5 - 161.0,
                                                                        4.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.937,
                                                                        0.937,
                                                                        0.937,
                                                                        panel_t * (1.0 - ws_t),
                                                                    ),
                                                                    metrics_state.0 > 0.0
                                                                        && panel_t > 0.0
                                                                        && ws_t < 1.0,
                                                                ),
                                                                (
                                                                    e[36],
                                                                    bevy::math::Vec2::new(
                                                                        (card_rect.width() - 36.0)
                                                                            .max(1.0),
                                                                        1.0,
                                                                    ),
                                                                    bevy::math::Vec3::new(
                                                                        card_rect.center().x
                                                                            - vw * 0.5,
                                                                        vh * 0.5 - 198.5,
                                                                        3.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.176,
                                                                        0.176,
                                                                        0.176,
                                                                        panel_t * (1.0 - ws_t),
                                                                    ),
                                                                    metrics_state.0 > 0.0
                                                                        && panel_t > 0.0
                                                                        && ws_t < 1.0,
                                                                ),
                                                                (
                                                                    e[37],
                                                                    launch_rect.size(),
                                                                    bevy::math::Vec3::new(
                                                                        launch_rect.center().x
                                                                            - vw * 0.5,
                                                                        vh * 0.5
                                                                            - launch_rect
                                                                                .center()
                                                                                .y,
                                                                        3.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.215
                                                                            + 0.060
                                                                                * anim_state.0[2]
                                                                            - 0.035
                                                                                * anim_state.1[2],
                                                                        0.215
                                                                            + 0.060
                                                                                * anim_state.0[2]
                                                                            - 0.035
                                                                                * anim_state.1[2],
                                                                        0.215
                                                                            + 0.060
                                                                                * anim_state.0[2]
                                                                            - 0.035
                                                                                * anim_state.1[2],
                                                                        panel_t * (1.0 - ws_t),
                                                                    ),
                                                                    metrics_state.0 > 0.0
                                                                        && panel_t > 0.0
                                                                        && ws_t < 1.0,
                                                                ),
                                                                (
                                                                    e[38],
                                                                    launch_rect.size(),
                                                                    bevy::math::Vec3::new(
                                                                        launch_rect.center().x
                                                                            - vw * 0.5,
                                                                        vh * 0.5
                                                                            - launch_rect
                                                                                .center()
                                                                                .y,
                                                                        3.5,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.360
                                                                            + 0.360
                                                                                * anim_state.0[2],
                                                                        0.360
                                                                            + 0.360
                                                                                * anim_state.0[2],
                                                                        0.360
                                                                            + 0.360
                                                                                * anim_state.0[2],
                                                                        panel_t * (1.0 - ws_t),
                                                                    ),
                                                                    metrics_state.0 > 0.0
                                                                        && panel_t > 0.0
                                                                        && ws_t < 1.0,
                                                                ),
                                                                (
                                                                    e[39],
                                                                    bevy::math::Vec2::splat(32.0),
                                                                    bevy::math::Vec3::new(
                                                                        launch_rect.min.x + 22.0
                                                                            - vw * 0.5,
                                                                        vh * 0.5 - 235.0,
                                                                        4.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.265
                                                                            + 0.055
                                                                                * anim_state.0[2]
                                                                            - 0.030
                                                                                * anim_state.1[2],
                                                                        0.265
                                                                            + 0.055
                                                                                * anim_state.0[2]
                                                                            - 0.030
                                                                                * anim_state.1[2],
                                                                        0.265
                                                                            + 0.055
                                                                                * anim_state.0[2]
                                                                            - 0.030
                                                                                * anim_state.1[2],
                                                                        panel_t * (1.0 - ws_t),
                                                                    ),
                                                                    metrics_state.0 > 0.0
                                                                        && panel_t > 0.0
                                                                        && ws_t < 1.0,
                                                                ),
                                                                (
                                                                    e[40],
                                                                    bevy::math::Vec2::splat(20.0),
                                                                    bevy::math::Vec3::new(
                                                                        launch_rect.min.x + 22.0
                                                                            - vw * 0.5,
                                                                        vh * 0.5 - 235.0,
                                                                        5.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.910
                                                                            + 0.075
                                                                                * anim_state.0[2],
                                                                        0.910
                                                                            + 0.075
                                                                                * anim_state.0[2],
                                                                        0.910
                                                                            + 0.075
                                                                                * anim_state.0[2],
                                                                        panel_t * (1.0 - ws_t),
                                                                    ),
                                                                    metrics_state.0 > 0.0
                                                                        && panel_t > 0.0
                                                                        && ws_t < 1.0,
                                                                ),
                                                                (
                                                                    e[42],
                                                                    bevy::math::Vec2::new(vw, vh),
                                                                    bevy::math::Vec3::new(
                                                                        0.0, 0.0, 12.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.0,
                                                                        0.0,
                                                                        0.0,
                                                                        0.58 * modal_t,
                                                                    ),
                                                                    modal_t > 0.0,
                                                                ),
                                                                (
                                                                    e[43],
                                                                    modal_rect.size() * m_scale,
                                                                    bevy::math::Vec3::new(
                                                                        0.0, 0.0, 13.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.245, 0.245, 0.245,
                                                                        modal_t,
                                                                    ),
                                                                    modal_t > 0.0,
                                                                ),
                                                                (
                                                                    e[44],
                                                                    (modal_rect.size()
                                                                        - bevy::math::Vec2::splat(
                                                                            2.0,
                                                                        ))
                                                                    .max(bevy::math::Vec2::ONE)
                                                                        * m_scale,
                                                                    bevy::math::Vec3::new(
                                                                        0.0, 0.0, 13.1,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.094, 0.094, 0.094,
                                                                        modal_t,
                                                                    ),
                                                                    modal_t > 0.0,
                                                                ),
                                                                    (
                                                                        e[45],
                                                                        bevy::math::Vec2::splat(44.0),
                                                                        bevy::math::Vec3::new(modal_rect.min.x + 50.0 - vw * 0.5, vh * 0.5 - modal_rect.min.y - 34.0, 14.0),
                                                                        bevy::color::Color::srgba(0.133, 0.133, 0.133, modal_t),
                                                                        modal_t > 0.0,
                                                                    ),
                                                                    (
                                                                        e[46],
                                                                        bevy::math::Vec2::splat(24.0),
                                                                        bevy::math::Vec3::new(modal_rect.min.x + 50.0 - vw * 0.5, vh * 0.5 - modal_rect.min.y - 34.0, 15.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, modal_t),
                                                                        modal_t > 0.0,
                                                                    ),
                                                                    (
                                                                        e[48],
                                                                        bevy::math::Vec2::new((modal_rect.width() - 56.0).max(1.0), 1.0),
                                                                        bevy::math::Vec3::new(modal_rect.center().x - vw * 0.5, vh * 0.5 - modal_rect.min.y - 68.0, 14.0),
                                                                        bevy::color::Color::srgba(0.176, 0.176, 0.176, modal_t),
                                                                        modal_t > 0.0,
                                                                    ),
                                                                    (
                                                                        e[50],
                                                                        bevy::math::Vec2::splat(68.0),
                                                                        bevy::math::Vec3::new(modal_rect.min.x + 62.0 - vw * 0.5, vh * 0.5 - modal_rect.min.y - 150.0, 14.0),
                                                                        bevy::color::Color::srgba(0.125 + 0.135 * f32::from(wizard_state.1 == 0u8), 0.125 + 0.135 * f32::from(wizard_state.1 == 0u8), 0.125 + 0.135 * f32::from(wizard_state.1 == 0u8), modal_t * (1.0 - anim_state.2[5])),
                                                                        modal_t > 0.0 && anim_state.2[5] < 1.0,
                                                                    ),
                                                                    (
                                                                        e[51],
                                                                        bevy::math::Vec2::splat(40.0),
                                                                        bevy::math::Vec3::new(modal_rect.min.x + 62.0 - vw * 0.5, vh * 0.5 - modal_rect.min.y - 150.0, 15.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, modal_t * (1.0 - anim_state.2[5])),
                                                                        modal_t > 0.0 && anim_state.2[5] < 1.0,
                                                                    ),
                                                                    (
                                                                        e[52],
                                                                        bevy::math::Vec2::splat(68.0),
                                                                        bevy::math::Vec3::new(modal_rect.min.x + 146.0 - vw * 0.5, vh * 0.5 - modal_rect.min.y - 150.0, 14.0),
                                                                        bevy::color::Color::srgba(0.125 + 0.135 * f32::from(wizard_state.1 == 1u8), 0.125 + 0.135 * f32::from(wizard_state.1 == 1u8), 0.125 + 0.135 * f32::from(wizard_state.1 == 1u8), modal_t * (1.0 - anim_state.2[5])),
                                                                        modal_t > 0.0 && anim_state.2[5] < 1.0,
                                                                    ),
                                                                    (
                                                                        e[53],
                                                                        bevy::math::Vec2::splat(40.0),
                                                                        bevy::math::Vec3::new(modal_rect.min.x + 146.0 - vw * 0.5, vh * 0.5 - modal_rect.min.y - 150.0, 15.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, modal_t * (1.0 - anim_state.2[5])),
                                                                        modal_t > 0.0 && anim_state.2[5] < 1.0,
                                                                    ),
                                                                    (
                                                                        e[55],
                                                                        bevy::math::Vec2::new((modal_rect.width() - 56.0).max(1.0), 46.0),
                                                                        bevy::math::Vec3::new(modal_rect.center().x - vw * 0.5, vh * 0.5 - modal_rect.min.y - 237.0, 14.0),
                                                                        bevy::color::Color::srgba(0.125, 0.125, 0.125, modal_t * (1.0 - anim_state.2[5])),
                                                                        modal_t > 0.0 && anim_state.2[5] < 1.0,
                                                                    ),
                                                                    (
                                                                        e[56],
                                                                        bevy::math::Vec2::new((modal_rect.width() - 56.0).max(1.0), 46.0),
                                                                        bevy::math::Vec3::new(modal_rect.center().x - vw * 0.5, vh * 0.5 - modal_rect.min.y - 237.0, 14.5),
                                                                        bevy::color::Color::srgba(0.78, 0.78, 0.78, modal_t * (1.0 - anim_state.2[5]) * f32::from(draft_state.2 || feedback_state.0 != 0u8)),
                                                                        modal_t > 0.0 && anim_state.2[5] < 1.0,
                                                                    ),
                                                                    (
                                                                        e[58],
                                                                        bevy::math::Vec2::new(2.0, 20.0),
                                                                        bevy::math::Vec3::new(modal_rect.min.x + 44.0 + layout_info.get(e[57]).map(|info: &bevy::text::TextLayoutInfo| info.glyphs.get(usize::from(draft_state.1)).map(|glyph: &bevy::text::PositionedGlyph| glyph.position.x).unwrap_or(info.size.x) / info.scale_factor.max(1.0)).unwrap_or(0.0) - vw * 0.5, vh * 0.5 - modal_rect.min.y - 237.0, 15.5),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, modal_t * (1.0 - anim_state.2[5])),
                                                                        modal_t > 0.0 && anim_state.2[5] < 1.0 && draft_state.2,
                                                                    ),
                                                                    (
                                                                        e[60],
                                                                        bevy::math::Vec2::new(120.0, 44.0),
                                                                        bevy::math::Vec3::new(modal_rect.max.x - 88.0 - vw * 0.5, vh * 0.5 - modal_rect.max.y + 50.0, 14.0),
                                                                        bevy::color::Color::srgba(0.260 + 0.605 * f32::from(wizard_state.0 == 1u8 && wizard_state.2 != 0u8 && (wizard_state.2 != 3u8 || wizard_state.3 != 0u8) || wizard_state.0 == 0u8 && feedback_state.0 == 0u8), 0.260 + 0.605 * f32::from(wizard_state.0 == 1u8 && wizard_state.2 != 0u8 && (wizard_state.2 != 3u8 || wizard_state.3 != 0u8) || wizard_state.0 == 0u8 && feedback_state.0 == 0u8), 0.260 + 0.605 * f32::from(wizard_state.0 == 1u8 && wizard_state.2 != 0u8 && (wizard_state.2 != 3u8 || wizard_state.3 != 0u8) || wizard_state.0 == 0u8 && feedback_state.0 == 0u8), modal_t),
                                                                        modal_t > 0.0,
                                                                    ),
                                                                    (
                                                                        e[62],
                                                                        bevy::math::Vec2::new((modal_rect.width() - 56.0).max(1.0), 46.0),
                                                                        bevy::math::Vec3::new(modal_rect.center().x - vw * 0.5, vh * 0.5 - modal_rect.min.y - 108.0, 14.0),
                                                                        bevy::color::Color::srgba(0.145 + 0.115 * f32::from(wizard_state.2 == 1u8), 0.145 + 0.115 * f32::from(wizard_state.2 == 1u8), 0.145 + 0.115 * f32::from(wizard_state.2 == 1u8), modal_t * anim_state.2[5]),
                                                                        modal_t > 0.0 && anim_state.2[5] > 0.0,
                                                                    ),
                                                                    (
                                                                        e[64],
                                                                        bevy::math::Vec2::new((modal_rect.width() - 56.0).max(1.0), 46.0),
                                                                        bevy::math::Vec3::new(modal_rect.center().x - vw * 0.5, vh * 0.5 - modal_rect.min.y - 162.0, 14.0),
                                                                        bevy::color::Color::srgba(0.145 + 0.115 * f32::from(wizard_state.2 == 2u8), 0.145 + 0.115 * f32::from(wizard_state.2 == 2u8), 0.145 + 0.115 * f32::from(wizard_state.2 == 2u8), modal_t * anim_state.2[5]),
                                                                        modal_t > 0.0 && anim_state.2[5] > 0.0,
                                                                    ),
                                                                    (
                                                                        e[66],
                                                                        bevy::math::Vec2::new((modal_rect.width() - 56.0).max(1.0), 46.0),
                                                                        bevy::math::Vec3::new(modal_rect.center().x - vw * 0.5, vh * 0.5 - modal_rect.min.y - 216.0, 14.0),
                                                                        bevy::color::Color::srgba(0.145 + 0.115 * f32::from(wizard_state.2 == 3u8), 0.145 + 0.115 * f32::from(wizard_state.2 == 3u8), 0.145 + 0.115 * f32::from(wizard_state.2 == 3u8), modal_t * anim_state.2[5]),
                                                                        modal_t > 0.0 && anim_state.2[5] > 0.0,
                                                                    ),
                                                                    (
                                                                        e[68],
                                                                        bevy::math::Vec2::new((modal_rect.width() - 92.0).max(1.0), 40.0),
                                                                        bevy::math::Vec3::new(modal_rect.center().x - vw * 0.5, vh * 0.5 - modal_rect.min.y - 267.0, 14.0),
                                                                        bevy::color::Color::srgba(0.125 + 0.135 * f32::from(wizard_state.3 == 1u8), 0.125 + 0.135 * f32::from(wizard_state.3 == 1u8), 0.125 + 0.135 * f32::from(wizard_state.3 == 1u8), modal_t * anim_state.2[5] * anim_state.2[6]),
                                                                        modal_t > 0.0 && anim_state.2[5] > 0.0 && anim_state.2[6] > 0.0,
                                                                    ),
                                                                    (
                                                                        e[70],
                                                                        bevy::math::Vec2::new((modal_rect.width() - 92.0).max(1.0), 40.0),
                                                                        bevy::math::Vec3::new(modal_rect.center().x - vw * 0.5, vh * 0.5 - modal_rect.min.y - 315.0, 14.0),
                                                                        bevy::color::Color::srgba(0.125 + 0.135 * f32::from(wizard_state.3 == 2u8), 0.125 + 0.135 * f32::from(wizard_state.3 == 2u8), 0.125 + 0.135 * f32::from(wizard_state.3 == 2u8), modal_t * anim_state.2[5] * anim_state.2[6]),
                                                                        modal_t > 0.0 && anim_state.2[5] > 0.0 && anim_state.2[6] > 0.0,
                                                                    ),
                                                                    (
                                                                        e[72],
                                                                        bevy::math::Vec2::new(104.0, 44.0),
                                                                        bevy::math::Vec3::new(modal_rect.min.x + 80.0 - vw * 0.5, vh * 0.5 - modal_rect.max.y + 50.0, 14.0),
                                                                        bevy::color::Color::srgba(0.215, 0.215, 0.215, modal_t * anim_state.2[5]),
                                                                        modal_t > 0.0 && anim_state.2[5] > 0.0,
                                                                    ),
                                                                    (
                                                                        e[76],
                                                                        bevy::math::Vec2::new((panel_rect.width() - 56.0).max(1.0), 48.0),
                                                                        bevy::math::Vec3::new(panel_rect.center().x - vw * 0.5, vh * 0.5 - 146.0 - 0.0, 2.0),
                                                                        bevy::color::Color::srgba(0.145 + 0.075 * f32::from(catalog_state.0.get(list_state.0 + 0usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_id == list_state.1)), 0.145 + 0.075 * f32::from(catalog_state.0.get(list_state.0 + 0usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_id == list_state.1)), 0.145 + 0.075 * f32::from(catalog_state.0.get(list_state.0 + 0usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_id == list_state.1)), ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 0usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| true),
                                                                    ),
                                                                    (
                                                                        e[77],
                                                                        bevy::math::Vec2::splat(26.0),
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 52.0 - vw * 0.5, vh * 0.5 - 146.0 - 0.0, 3.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 0usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_icon == 0u8),
                                                                    ),
                                                                    (
                                                                        e[78],
                                                                        bevy::math::Vec2::splat(26.0),
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 52.0 - vw * 0.5, vh * 0.5 - 146.0 - 0.0, 3.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 0usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_icon == 1u8),
                                                                    ),
                                                                    (
                                                                        e[81],
                                                                        bevy::math::Vec2::new((panel_rect.width() - 56.0).max(1.0), 48.0),
                                                                        bevy::math::Vec3::new(panel_rect.center().x - vw * 0.5, vh * 0.5 - 146.0 - 56.0, 2.0),
                                                                        bevy::color::Color::srgba(0.145 + 0.075 * f32::from(catalog_state.0.get(list_state.0 + 1usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_id == list_state.1)), 0.145 + 0.075 * f32::from(catalog_state.0.get(list_state.0 + 1usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_id == list_state.1)), 0.145 + 0.075 * f32::from(catalog_state.0.get(list_state.0 + 1usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_id == list_state.1)), ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 1usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| true),
                                                                    ),
                                                                    (
                                                                        e[82],
                                                                        bevy::math::Vec2::splat(26.0),
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 52.0 - vw * 0.5, vh * 0.5 - 146.0 - 56.0, 3.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 1usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_icon == 0u8),
                                                                    ),
                                                                    (
                                                                        e[83],
                                                                        bevy::math::Vec2::splat(26.0),
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 52.0 - vw * 0.5, vh * 0.5 - 146.0 - 56.0, 3.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 1usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_icon == 1u8),
                                                                    ),
                                                                    (
                                                                        e[86],
                                                                        bevy::math::Vec2::new((panel_rect.width() - 56.0).max(1.0), 48.0),
                                                                        bevy::math::Vec3::new(panel_rect.center().x - vw * 0.5, vh * 0.5 - 146.0 - 112.0, 2.0),
                                                                        bevy::color::Color::srgba(0.145 + 0.075 * f32::from(catalog_state.0.get(list_state.0 + 2usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_id == list_state.1)), 0.145 + 0.075 * f32::from(catalog_state.0.get(list_state.0 + 2usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_id == list_state.1)), 0.145 + 0.075 * f32::from(catalog_state.0.get(list_state.0 + 2usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_id == list_state.1)), ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 2usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| true),
                                                                    ),
                                                                    (
                                                                        e[87],
                                                                        bevy::math::Vec2::splat(26.0),
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 52.0 - vw * 0.5, vh * 0.5 - 146.0 - 112.0, 3.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 2usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_icon == 0u8),
                                                                    ),
                                                                    (
                                                                        e[88],
                                                                        bevy::math::Vec2::splat(26.0),
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 52.0 - vw * 0.5, vh * 0.5 - 146.0 - 112.0, 3.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 2usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_icon == 1u8),
                                                                    ),
                                                                    (
                                                                        e[91],
                                                                        bevy::math::Vec2::new((panel_rect.width() - 56.0).max(1.0), 48.0),
                                                                        bevy::math::Vec3::new(panel_rect.center().x - vw * 0.5, vh * 0.5 - 146.0 - 168.0, 2.0),
                                                                        bevy::color::Color::srgba(0.145 + 0.075 * f32::from(catalog_state.0.get(list_state.0 + 3usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_id == list_state.1)), 0.145 + 0.075 * f32::from(catalog_state.0.get(list_state.0 + 3usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_id == list_state.1)), 0.145 + 0.075 * f32::from(catalog_state.0.get(list_state.0 + 3usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_id == list_state.1)), ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 3usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| true),
                                                                    ),
                                                                    (
                                                                        e[92],
                                                                        bevy::math::Vec2::splat(26.0),
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 52.0 - vw * 0.5, vh * 0.5 - 146.0 - 168.0, 3.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 3usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_icon == 0u8),
                                                                    ),
                                                                    (
                                                                        e[93],
                                                                        bevy::math::Vec2::splat(26.0),
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 52.0 - vw * 0.5, vh * 0.5 - 146.0 - 168.0, 3.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 3usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_icon == 1u8),
                                                                    ),
                                                                    (
                                                                        e[96],
                                                                        bevy::math::Vec2::new((panel_rect.width() - 56.0).max(1.0), 48.0),
                                                                        bevy::math::Vec3::new(panel_rect.center().x - vw * 0.5, vh * 0.5 - 146.0 - 224.0, 2.0),
                                                                        bevy::color::Color::srgba(0.145 + 0.075 * f32::from(catalog_state.0.get(list_state.0 + 4usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_id == list_state.1)), 0.145 + 0.075 * f32::from(catalog_state.0.get(list_state.0 + 4usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_id == list_state.1)), 0.145 + 0.075 * f32::from(catalog_state.0.get(list_state.0 + 4usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_id == list_state.1)), ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 4usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| true),
                                                                    ),
                                                                    (
                                                                        e[97],
                                                                        bevy::math::Vec2::splat(26.0),
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 52.0 - vw * 0.5, vh * 0.5 - 146.0 - 224.0, 3.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 4usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_icon == 0u8),
                                                                    ),
                                                                    (
                                                                        e[98],
                                                                        bevy::math::Vec2::splat(26.0),
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 52.0 - vw * 0.5, vh * 0.5 - 146.0 - 224.0, 3.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 4usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_icon == 1u8),
                                                                    ),
                                                                    (
                                                                        e[101],
                                                                        bevy::math::Vec2::new((panel_rect.width() - 56.0).max(1.0), 48.0),
                                                                        bevy::math::Vec3::new(panel_rect.center().x - vw * 0.5, vh * 0.5 - 146.0 - 280.0, 2.0),
                                                                        bevy::color::Color::srgba(0.145 + 0.075 * f32::from(catalog_state.0.get(list_state.0 + 5usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_id == list_state.1)), 0.145 + 0.075 * f32::from(catalog_state.0.get(list_state.0 + 5usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_id == list_state.1)), 0.145 + 0.075 * f32::from(catalog_state.0.get(list_state.0 + 5usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_id == list_state.1)), ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 5usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| true),
                                                                    ),
                                                                    (
                                                                        e[102],
                                                                        bevy::math::Vec2::splat(26.0),
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 52.0 - vw * 0.5, vh * 0.5 - 146.0 - 280.0, 3.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 5usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_icon == 0u8),
                                                                    ),
                                                                    (
                                                                        e[103],
                                                                        bevy::math::Vec2::splat(26.0),
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 52.0 - vw * 0.5, vh * 0.5 - 146.0 - 280.0, 3.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 5usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| *_icon == 1u8),
                                                                    ),
                                                                    (
                                                                        e[106],
                                                                        bevy::math::Vec2::splat(32.0),
                                                                        bevy::math::Vec3::new(panel_rect.max.x - 104.0 - vw * 0.5, vh * 0.5 - 486.0, 2.0),
                                                                        bevy::color::Color::srgba(0.176, 0.176, 0.176, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.len() > 6usize,
                                                                    ),
                                                                    (
                                                                        e[108],
                                                                        bevy::math::Vec2::splat(32.0),
                                                                        bevy::math::Vec3::new(panel_rect.max.x - 64.0 - vw * 0.5, vh * 0.5 - 486.0, 2.0),
                                                                        bevy::color::Color::srgba(0.176, 0.176, 0.176, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.len() > 6usize,
                                                                    ),
                                                            ]
                                                            .into_iter()
                                                            .for_each(
                                                                |(
                                                                    entity,
                                                                    size,
                                                                    pos,
                                                                    colour,
                                                                    shown,
                                                                ): (
                                                                    bevy::ecs::entity::Entity,
                                                                    bevy::math::Vec2,
                                                                    bevy::math::Vec3,
                                                                    bevy::color::Color,
                                                                    bool,
                                                                )| {
                                                                    sprites
                                                                        .get(entity)
                                                                        .into_iter()
                                                                        .for_each(
                                                                            |(
                                                                                sprite,
                                                                                transform,
                                                                                visibility,
                                                                            ): (
                                                                                &bevy::sprite::Sprite,
                                                                                &bevy::transform::components::Transform,
                                                                                &bevy::camera::visibility::Visibility,
                                                                            )| {
                                                                                if sprite.color
                                                                                    != colour
                                                                                    || sprite
                                                                                        .custom_size
                                                                                        != std::option::Option::Some(
                                                                                            size,
                                                                                        )
                                                                                {
                                                                                    commands
                                                                                        .entity(
                                                                                            entity,
                                                                                        )
                                                                                        .insert(
                                                                                            bevy::sprite::Sprite {
                                                                                                color: colour,
                                                                                                custom_size: std::option::Option::Some(size),
                                                                                                ..sprite.clone()
                                                                                            },
                                                                                        );
                                                                                }
                                                                                if transform
                                                                                    .translation
                                                                                    != pos
                                                                                {
                                                                                    commands
                                                                                        .entity(
                                                                                            entity,
                                                                                        )
                                                                                        .insert(
                                                                                            bevy::transform::components::Transform::from_translation(pos),
                                                                                        );
                                                                                }
                                                                                if (*visibility
                                                                                    == bevy::camera::visibility::Visibility::Inherited)
                                                                                    != shown
                                                                                {
                                                                                    commands
                                                                                        .entity(
                                                                                            entity,
                                                                                        )
                                                                                        .insert(
                                                                                            if shown {
                                                                                                bevy::camera::visibility::Visibility::Inherited
                                                                                            } else {
                                                                                                bevy::camera::visibility::Visibility::Hidden
                                                                                            },
                                                                                        );
                                                                                }
                                                                            },
                                                                        );
                                                                },
                                                            );

                                                            [
                                                                (
                                                                    e[5],
                                                                    bevy::math::Vec3::new(
                                                                        76.0 - vw * 0.5,
                                                                        vh * 0.5 - 44.0,
                                                                        3.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.937,
                                                                        0.937,
                                                                        0.937,
                                                                        menu_t * (1.0 - ws_t),
                                                                    ),
                                                                    menu_t > 0.0 && ws_t < 1.0,
                                                                ),
                                                                (
                                                                    e[6],
                                                                    bevy::math::Vec3::new(
                                                                        76.0 - vw * 0.5,
                                                                        vh * 0.5 - 44.0,
                                                                        3.1,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.937,
                                                                        0.937,
                                                                        0.937,
                                                                        menu_t * ws_t,
                                                                    ),
                                                                    menu_t > 0.0 && ws_t > 0.0,
                                                                ),
                                                                (
                                                                    e[12],
                                                                    bevy::math::Vec3::new(
                                                                        72.0 - vw * 0.5,
                                                                        vh * 0.5 - 116.0,
                                                                        4.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        (0.780
                                                                            + 0.157
                                                                                * anim_state.0[0])
                                                                            * nav_t
                                                                            + 0.086 * (1.0 - nav_t),
                                                                        (0.780
                                                                            + 0.157
                                                                                * anim_state.0[0])
                                                                            * nav_t
                                                                            + 0.086 * (1.0 - nav_t),
                                                                        (0.780
                                                                            + 0.157
                                                                                * anim_state.0[0])
                                                                            * nav_t
                                                                            + 0.086 * (1.0 - nav_t),
                                                                        menu_t * (1.0 - ws_t),
                                                                    ),
                                                                    menu_t > 0.0 && ws_t < 1.0,
                                                                ),
                                                                (
                                                                    e[17],
                                                                    bevy::math::Vec3::new(
                                                                        72.0 - vw * 0.5,
                                                                        vh * 0.5 - 172.0,
                                                                        4.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        (0.780
                                                                            + 0.157
                                                                                * anim_state.0[1])
                                                                            * (1.0 - nav_t)
                                                                            + 0.086 * nav_t,
                                                                        (0.780
                                                                            + 0.157
                                                                                * anim_state.0[1])
                                                                            * (1.0 - nav_t)
                                                                            + 0.086 * nav_t,
                                                                        (0.780
                                                                            + 0.157
                                                                                * anim_state.0[1])
                                                                            * (1.0 - nav_t)
                                                                            + 0.086 * nav_t,
                                                                        menu_t * (1.0 - ws_t),
                                                                    ),
                                                                    menu_t > 0.0 && ws_t < 1.0,
                                                                ),
                                                                (
                                                                    e[22],
                                                                    bevy::math::Vec3::new(
                                                                        72.0 - vw * 0.5,
                                                                        vh * 0.5 - 116.0,
                                                                        4.1,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.880
                                                                            + 0.090
                                                                                * anim_state.0[3],
                                                                        0.880
                                                                            + 0.090
                                                                                * anim_state.0[3],
                                                                        0.880
                                                                            + 0.090
                                                                                * anim_state.0[3],
                                                                        menu_t * ws_t,
                                                                    ),
                                                                    menu_t > 0.0 && ws_t > 0.0,
                                                                ),
                                                                (
                                                                    e[28],
                                                                    bevy::math::Vec3::new(
                                                                        panel_rect.min.x + 86.0
                                                                            - vw * 0.5,
                                                                        vh * 0.5 - 60.0,
                                                                        3.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.937,
                                                                        0.937,
                                                                        0.937,
                                                                        panel_t * (1.0 - ws_t),
                                                                    ),
                                                                    metrics_state.0 > 0.0
                                                                        && panel_t > 0.0
                                                                        && ws_t < 1.0,
                                                                ),
                                                                (
                                                                    e[29],
                                                                    bevy::math::Vec3::new(
                                                                        panel_rect.min.x + 86.0
                                                                            - vw * 0.5,
                                                                        vh * 0.5 - 60.0,
                                                                        3.1,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.937,
                                                                        0.937,
                                                                        0.937,
                                                                        panel_t * ws_t,
                                                                    ),
                                                                    metrics_state.0 > 0.0
                                                                        && panel_t > 0.0
                                                                        && ws_t > 0.0,
                                                                ),
                                                                (
                                                                    e[35],
                                                                    bevy::math::Vec3::new(
                                                                        card_rect.min.x + 74.0
                                                                            - vw * 0.5,
                                                                        vh * 0.5 - 161.0,
                                                                        4.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.937,
                                                                        0.937,
                                                                        0.937,
                                                                        panel_t * (1.0 - ws_t),
                                                                    ),
                                                                    metrics_state.0 > 0.0
                                                                        && panel_t > 0.0
                                                                        && ws_t < 1.0,
                                                                ),
                                                                (
                                                                    e[41],
                                                                    bevy::math::Vec3::new(
                                                                        launch_rect.min.x + 48.0
                                                                            - vw * 0.5,
                                                                        vh * 0.5 - 235.0,
                                                                        5.0,
                                                                    ),
                                                                    bevy::color::Color::srgba(
                                                                        0.910
                                                                            + 0.075
                                                                                * anim_state.0[2],
                                                                        0.910
                                                                            + 0.075
                                                                                * anim_state.0[2],
                                                                        0.910
                                                                            + 0.075
                                                                                * anim_state.0[2],
                                                                        panel_t * (1.0 - ws_t),
                                                                    ),
                                                                    metrics_state.0 > 0.0
                                                                        && panel_t > 0.0
                                                                        && ws_t < 1.0,
                                                                ),
                                                                    (
                                                                        e[47],
                                                                        bevy::math::Vec3::new(modal_rect.min.x + 86.0 - vw * 0.5, vh * 0.5 - modal_rect.min.y - 34.0, 15.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, modal_t),
                                                                        modal_t > 0.0,
                                                                    ),
                                                                    (
                                                                        e[49],
                                                                        bevy::math::Vec3::new(modal_rect.min.x + 28.0 - vw * 0.5, vh * 0.5 - modal_rect.min.y - 104.0, 15.0),
                                                                        bevy::color::Color::srgba(0.62, 0.62, 0.62, modal_t * (1.0 - anim_state.2[5])),
                                                                        modal_t > 0.0 && anim_state.2[5] < 1.0,
                                                                    ),
                                                                    (
                                                                        e[54],
                                                                        bevy::math::Vec3::new(modal_rect.min.x + 28.0 - vw * 0.5, vh * 0.5 - modal_rect.min.y - 200.0, 15.0),
                                                                        bevy::color::Color::srgba(0.62, 0.62, 0.62, modal_t * (1.0 - anim_state.2[5])),
                                                                        modal_t > 0.0 && anim_state.2[5] < 1.0,
                                                                    ),
                                                                    (
                                                                        e[57],
                                                                        bevy::math::Vec3::new(modal_rect.min.x + 44.0 - vw * 0.5, vh * 0.5 - modal_rect.min.y - 237.0, 15.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, modal_t * (1.0 - anim_state.2[5])),
                                                                        modal_t > 0.0 && anim_state.2[5] < 1.0,
                                                                    ),
                                                                    (
                                                                        e[59],
                                                                        bevy::math::Vec3::new(modal_rect.min.x + 28.0 - vw * 0.5, vh * 0.5 - modal_rect.min.y - 276.0, 15.0),
                                                                        bevy::color::Color::srgba(0.7, 0.7, 0.7, modal_t * (1.0 - anim_state.2[5])),
                                                                        modal_t > 0.0 && anim_state.2[5] < 1.0 && feedback_state.0 != 0u8,
                                                                    ),
                                                                    (
                                                                        e[61],
                                                                        bevy::math::Vec3::new(modal_rect.max.x - 88.0 - vw * 0.5, vh * 0.5 - modal_rect.max.y + 50.0, 15.0),
                                                                        bevy::color::Color::srgba(0.086 + 0.534 * f32::from(!(wizard_state.0 == 1u8 && wizard_state.2 != 0u8 && (wizard_state.2 != 3u8 || wizard_state.3 != 0u8) || wizard_state.0 == 0u8 && feedback_state.0 == 0u8)), 0.086 + 0.534 * f32::from(!(wizard_state.0 == 1u8 && wizard_state.2 != 0u8 && (wizard_state.2 != 3u8 || wizard_state.3 != 0u8) || wizard_state.0 == 0u8 && feedback_state.0 == 0u8)), 0.086 + 0.534 * f32::from(!(wizard_state.0 == 1u8 && wizard_state.2 != 0u8 && (wizard_state.2 != 3u8 || wizard_state.3 != 0u8) || wizard_state.0 == 0u8 && feedback_state.0 == 0u8)), modal_t),
                                                                        modal_t > 0.0,
                                                                    ),
                                                                    (
                                                                        e[63],
                                                                        bevy::math::Vec3::new(modal_rect.min.x + 46.0 - vw * 0.5, vh * 0.5 - modal_rect.min.y - 108.0, 15.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, modal_t * anim_state.2[5]),
                                                                        modal_t > 0.0 && anim_state.2[5] > 0.0,
                                                                    ),
                                                                    (
                                                                        e[65],
                                                                        bevy::math::Vec3::new(modal_rect.min.x + 46.0 - vw * 0.5, vh * 0.5 - modal_rect.min.y - 162.0, 15.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, modal_t * anim_state.2[5]),
                                                                        modal_t > 0.0 && anim_state.2[5] > 0.0,
                                                                    ),
                                                                    (
                                                                        e[67],
                                                                        bevy::math::Vec3::new(modal_rect.min.x + 46.0 - vw * 0.5, vh * 0.5 - modal_rect.min.y - 216.0, 15.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, modal_t * anim_state.2[5]),
                                                                        modal_t > 0.0 && anim_state.2[5] > 0.0,
                                                                    ),
                                                                    (
                                                                        e[69],
                                                                        bevy::math::Vec3::new(modal_rect.min.x + 64.0 - vw * 0.5, vh * 0.5 - modal_rect.min.y - 267.0, 15.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, modal_t * anim_state.2[5] * anim_state.2[6]),
                                                                        modal_t > 0.0 && anim_state.2[5] > 0.0 && anim_state.2[6] > 0.0,
                                                                    ),
                                                                    (
                                                                        e[71],
                                                                        bevy::math::Vec3::new(modal_rect.min.x + 64.0 - vw * 0.5, vh * 0.5 - modal_rect.min.y - 315.0, 15.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, modal_t * anim_state.2[5] * anim_state.2[6]),
                                                                        modal_t > 0.0 && anim_state.2[5] > 0.0 && anim_state.2[6] > 0.0,
                                                                    ),
                                                                    (
                                                                        e[73],
                                                                        bevy::math::Vec3::new(modal_rect.min.x + 80.0 - vw * 0.5, vh * 0.5 - modal_rect.max.y + 50.0, 15.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, modal_t * anim_state.2[5]),
                                                                        modal_t > 0.0 && anim_state.2[5] > 0.0,
                                                                    ),
                                                                    (
                                                                        e[74],
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 28.0 - vw * 0.5, vh * 0.5 - 140.0, 3.0),
                                                                        bevy::color::Color::srgba(0.62, 0.62, 0.62, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.is_empty() && feedback_state.1 == 0u8,
                                                                    ),
                                                                    (
                                                                        e[75],
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 28.0 - vw * 0.5, vh * 0.5 - 140.0, 3.0),
                                                                        bevy::color::Color::srgba(0.78, 0.78, 0.78, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && feedback_state.1 != 0u8,
                                                                    ),
                                                                    (
                                                                        e[79],
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 76.0 - vw * 0.5, vh * 0.5 - 138.0, 3.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 0usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| true),
                                                                    ),
                                                                    (
                                                                        e[80],
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 76.0 - vw * 0.5, vh * 0.5 - 156.0, 3.0),
                                                                        bevy::color::Color::srgba(0.62, 0.62, 0.62, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 0usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| true),
                                                                    ),
                                                                    (
                                                                        e[84],
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 76.0 - vw * 0.5, vh * 0.5 - 194.0, 3.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 1usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| true),
                                                                    ),
                                                                    (
                                                                        e[85],
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 76.0 - vw * 0.5, vh * 0.5 - 212.0, 3.0),
                                                                        bevy::color::Color::srgba(0.62, 0.62, 0.62, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 1usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| true),
                                                                    ),
                                                                    (
                                                                        e[89],
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 76.0 - vw * 0.5, vh * 0.5 - 250.0, 3.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 2usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| true),
                                                                    ),
                                                                    (
                                                                        e[90],
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 76.0 - vw * 0.5, vh * 0.5 - 268.0, 3.0),
                                                                        bevy::color::Color::srgba(0.62, 0.62, 0.62, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 2usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| true),
                                                                    ),
                                                                    (
                                                                        e[94],
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 76.0 - vw * 0.5, vh * 0.5 - 306.0, 3.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 3usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| true),
                                                                    ),
                                                                    (
                                                                        e[95],
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 76.0 - vw * 0.5, vh * 0.5 - 324.0, 3.0),
                                                                        bevy::color::Color::srgba(0.62, 0.62, 0.62, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 3usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| true),
                                                                    ),
                                                                    (
                                                                        e[99],
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 76.0 - vw * 0.5, vh * 0.5 - 362.0, 3.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 4usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| true),
                                                                    ),
                                                                    (
                                                                        e[100],
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 76.0 - vw * 0.5, vh * 0.5 - 380.0, 3.0),
                                                                        bevy::color::Color::srgba(0.62, 0.62, 0.62, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 4usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| true),
                                                                    ),
                                                                    (
                                                                        e[104],
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 76.0 - vw * 0.5, vh * 0.5 - 418.0, 3.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 5usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| true),
                                                                    ),
                                                                    (
                                                                        e[105],
                                                                        bevy::math::Vec3::new(panel_rect.min.x + 76.0 - vw * 0.5, vh * 0.5 - 436.0, 3.0),
                                                                        bevy::color::Color::srgba(0.62, 0.62, 0.62, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.get(list_state.0 + 5usize).map_or(false, |(_id, _icon, _name, _kind, _dim): &(u64, u8, std::string::String, u8, u8)| true),
                                                                    ),
                                                                    (
                                                                        e[107],
                                                                        bevy::math::Vec3::new(panel_rect.max.x - 104.0 - vw * 0.5, vh * 0.5 - 486.0, 3.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.len() > 6usize,
                                                                    ),
                                                                    (
                                                                        e[109],
                                                                        bevy::math::Vec3::new(panel_rect.max.x - 64.0 - vw * 0.5, vh * 0.5 - 486.0, 3.0),
                                                                        bevy::color::Color::srgba(0.937, 0.937, 0.937, ws_t * panel_t),
                                                                        metrics_state.0 > 0.0 && panel_t > 0.0 && ws_t > 0.0 && final_mode == 2u8 && !final_modal && catalog_state.0.len() > 6usize,
                                                                    ),
                                                            ]
                                                            .into_iter()
                                                            .for_each(
                                                                |(
                                                                    entity,
                                                                    pos,
                                                                    colour,
                                                                    shown,
                                                                ): (
                                                                    bevy::ecs::entity::Entity,
                                                                    bevy::math::Vec3,
                                                                    bevy::color::Color,
                                                                    bool,
                                                                )| {
                                                                    texts
                                                                        .get(entity)
                                                                        .into_iter()
                                                                        .for_each(
                                                                            |(
                                                                                text_colour,
                                                                                transform,
                                                                                visibility,
                                                                            ): (
                                                                                &bevy::text::TextColor,
                                                                                &bevy::transform::components::Transform,
                                                                                &bevy::camera::visibility::Visibility,
                                                                            )| {
                                                                                if text_colour.0
                                                                                    != colour
                                                                                {
                                                                                    commands
                                                                                        .entity(
                                                                                            entity,
                                                                                        )
                                                                                        .insert(
                                                                                            bevy::text::TextColor(colour),
                                                                                        );
                                                                                }
                                                                                if transform
                                                                                    .translation
                                                                                    != pos
                                                                                {
                                                                                    commands
                                                                                        .entity(
                                                                                            entity,
                                                                                        )
                                                                                        .insert(
                                                                                            bevy::transform::components::Transform::from_translation(pos),
                                                                                        );
                                                                                }
                                                                                if (*visibility
                                                                                    == bevy::camera::visibility::Visibility::Inherited)
                                                                                    != shown
                                                                                {
                                                                                    commands
                                                                                        .entity(
                                                                                            entity,
                                                                                        )
                                                                                        .insert(
                                                                                            if shown {
                                                                                                bevy::camera::visibility::Visibility::Inherited
                                                                                            } else {
                                                                                                bevy::camera::visibility::Visibility::Hidden
                                                                                            },
                                                                                        );
                                                                                }
                                                                            },
                                                                        );
                                                                },
                                                            );
                                                        },
                                                    );
                                                },
                                            );


                                            ui_entities
                                                .as_ref()
                                                .copied()
                                                .into_iter()
                                                .for_each(
                                                    |e: [bevy::ecs::entity::Entity; 110]| {
                                                    [(
                                                e[57],
                                                draft_state.0.clone(),
                                            ), (
                                                e[59],
                                                std::string::String::from(match feedback_state.0 {
                                                    1u8 => "Enter a name",
                                                    2u8 => "Names are limited to 64 characters",
                                                    3u8 => "Names cannot contain control characters",
                                                    4u8 => "That name is already in the workspace",
                                                    5u8 => "Choose a project type",
                                                    6u8 => "Choose 2D or 3D for a graphical project",
                                                    7u8 => "Workspace storage is unavailable",
                                                    8u8 => "The project could not be saved",
                                                    _ => "",
                                                }),
                                            ), (
                                                e[75],
                                                std::string::String::from(match feedback_state.1 {
                                                    1u8 => "Application data directory unavailable",
                                                    2u8 => "Saved workspace data is damaged; original file kept",
                                                    3u8 => "Workspace data uses a newer format; original file kept",
                                                    5u8 => "Workspace data could not be read",
                                                    _ => "",
                                                }),
                                            )]
                                            .into_iter()
                                            .chain(
                                                [0usize, 1usize, 2usize, 3usize, 4usize, 5usize]
                                                    .into_iter()
                                                    .flat_map(|slot: usize| {
                                                        [
                                                            (
                                                                e[79usize + 5usize * slot],
                                                                catalog_state
                                                                    .0
                                                                    .get(list_state.0 + slot)
                                                                    .map_or(
                                                                        std::string::String::new(),
                                                                        |(_id, _icon, name, _kind, _dim): &(
                                                                            u64,
                                                                            u8,
                                                                            std::string::String,
                                                                            u8,
                                                                            u8,
                                                                        )| name.clone(),
                                                                    ),
                                                            ),
                                                            (
                                                                e[80usize + 5usize * slot],
                                                                catalog_state
                                                                    .0
                                                                    .get(list_state.0 + slot)
                                                                    .map_or(
                                                                        std::string::String::new(),
                                                                        |(_id, _icon, _name, kind, dim): &(
                                                                            u64,
                                                                            u8,
                                                                            std::string::String,
                                                                            u8,
                                                                            u8,
                                                                        )| {
                                                                            std::string::String::from(
                                                                                if *kind == 1u8 {
                                                                                    "Operating System"
                                                                                } else if *kind == 2u8 {
                                                                                    "Command Line Interface"
                                                                                } else if *dim == 1u8 {
                                                                                    "Graphical User Interface - 2D"
                                                                                } else {
                                                                                    "Graphical User Interface - 3D"
                                                                                },
                                                                            )
                                                                        },
                                                                    ),
                                                            ),
                                                        ]
                                                    }),
                                            )
                                            .for_each(
                                                |(entity, wanted): (
                                                    bevy::ecs::entity::Entity,
                                                    std::string::String,
                                                )| {
                                                    match std::iter::once(wanted)
                                                        .map(|wanted: std::string::String| {
                                                            (
                                                                wanted.clone(),
                                                                dynamic_texts.get(entity).map_or(
                                                                    true,
                                                                    |body: &bevy::sprite::Text2d| {
                                                                        body.0 != wanted
                                                                    },
                                                                ),
                                                            )
                                                        })
                                                        .next()
                                                    {
                                                        std::option::Option::Some((wanted, true)) => {
                                                            commands
                                                                .entity(entity)
                                                                .insert(bevy::sprite::Text2d(wanted));
                                                        }
                                                        _ => {}
                                                    }
                                                },
                                            );
                                                },
                                            );
                                            (*dot_state).into_iter().for_each(
                                                |(dot_entity, cur_dot_pos, cur_dot_vel, _): (
                                                    bevy::ecs::entity::Entity,
                                                    bevy::math::Vec2,
                                                    bevy::math::Vec2,
                                                    bevy::math::Vec2,
                                                )| {
                                                    sprites.get(dot_entity).into_iter().for_each(
                                                        |(
                                                            _sprite,
                                                            transform,
                                                            visibility,
                                                        ): (
                                                            &bevy::sprite::Sprite,
                                                            &bevy::transform::components::Transform,
                                                            &bevy::camera::visibility::Visibility,
                                                        )| {
                                                            if transform.translation
                                                                != bevy::math::Vec3::new(
                                                                    cur_dot_pos.x - vw * 0.5,
                                                                    vh * 0.5 - cur_dot_pos.y,
                                                                    20.0,
                                                                )
                                                            {
                                                                commands.entity(dot_entity).insert(
                                                                    bevy::transform::components::Transform::from_xyz(
                                                                        cur_dot_pos.x - vw * 0.5,
                                                                        vh * 0.5 - cur_dot_pos.y,
                                                                        20.0,
                                                                    ),
                                                                );
                                                            }
                                                            if (*visibility
                                                                == bevy::camera::visibility::Visibility::Inherited)
                                                                != window.focused
                                                            {
                                                                commands.entity(dot_entity).insert(
                                                                    if window.focused {
                                                                        bevy::camera::visibility::Visibility::Inherited
                                                                    } else {
                                                                        bevy::camera::visibility::Visibility::Hidden
                                                                    },
                                                                );
                                                            }
                                                        },
                                                    );

                                                    std::iter::once(
                                                        if focus_state.2 < 4u8
                                                            || metrics_state.0 <= 0.0
                                                            || (window.focused
                                                                && cur_dot_vel
                                                                    != bevy::math::Vec2::ZERO)
                                                            || anim_state
                                                                .0
                                                                .iter()
                                                                .zip(0u8..17u8)
                                                                .any(|(v, idx): (&f32, u8)| {
                                                                    (*v - f32::from(u8::from(
                                                                        hovered
                                                                            == idx
                                                                                .saturating_add(1),
                                                                    )))
                                                                    .abs()
                                                                        > 0.0
                                                                })
                                                            || anim_state
                                                                .1
                                                                .iter()
                                                                .zip(0u8..17u8)
                                                                .any(|(v, idx): (&f32, u8)| {
                                                                    (*v - f32::from(u8::from(
                                                                        (hovered
                                                                            == idx
                                                                                .saturating_add(1))
                                                                            && focus_state.3
                                                                            && keys.pressed(
                                                                                bevy::input::keyboard::KeyCode::KeyE,
                                                                            )
                                                                            && (focus_state.1
                                                                                & 64u16)
                                                                                == 0,
                                                                    )))
                                                                    .abs()
                                                                        > 0.0
                                                                })
                                                            || (anim_state.2[0]
                                                                - if final_mode == 0u8 {
                                                                    0.0
                                                                } else {
                                                                    1.0
                                                                })
                                                                .abs()
                                                                > 0.0
                                                            || (anim_state.2[1]
                                                                - if final_mode == 0u8 {
                                                                    0.0
                                                                } else {
                                                                    1.0
                                                                })
                                                                .abs()
                                                                > 0.0
                                                            || (anim_state.2[2]
                                                                - if final_mode == 2u8 {
                                                                    1.0
                                                                } else {
                                                                    0.0
                                                                })
                                                                .abs()
                                                                > 0.0
                                                            || (anim_state.2[3]
                                                                - if final_mode == 2u8
                                                                    && final_modal
                                                                {
                                                                    1.0
                                                                } else {
                                                                    0.0
                                                                })
                                                                .abs()
                                                                > 0.0
                                                            || (anim_state.2[4]
                                                                - if final_menu {
                                                                    1.0
                                                                } else {
                                                                    0.0
                                                                })
                                                                .abs()
                                                                > 0.0
                                                            || (anim_state.2[5]
                                                                - if final_mode == 2u8
                                                                    && final_modal
                                                                    && wizard_state.0 == 1u8
                                                                {
                                                                    1.0
                                                                } else {
                                                                    0.0
                                                                })
                                                                .abs()
                                                                > 0.0
                                                            || (anim_state.2[6]
                                                                - if final_mode == 2u8
                                                                    && final_modal
                                                                    && wizard_state.0 == 1u8
                                                                    && wizard_state.2 == 3u8
                                                                {
                                                                    1.0
                                                                } else {
                                                                    0.0
                                                                })
                                                                .abs()
                                                                > 0.0
                                                            || (window.focused
                                                                && escape_state.0.is_some())
                                                        {
                                                            bevy::winit::UpdateMode::Continuous
                                                        } else {
                                                            bevy::winit::UpdateMode::reactive_low_power(
                                                                std::time::Duration::MAX,
                                                            )
                                                        },
                                                    )
                                                    .for_each(
                                                        |desired_focused_mode: bevy::winit::UpdateMode| {
                                                            if settings.focused_mode
                                                                != desired_focused_mode
                                                            {
                                                                settings.focused_mode =
                                                                    desired_focused_mode;
                                                            }
                                                            if settings.unfocused_mode
                                                                != bevy::winit::UpdateMode::reactive_low_power(
                                                                    std::time::Duration::MAX,
                                                                )
                                                            {
                                                                settings.unfocused_mode =
                                                                    bevy::winit::UpdateMode::reactive_low_power(
                                                                        std::time::Duration::MAX,
                                                                    );
                                                            }
                                                        },
                                                    );
                                                },
                                            );
                                        },
                                    );
                                });
                            });
                        },
                    );
                });
            },
        )
        .run();
}
