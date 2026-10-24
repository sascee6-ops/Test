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
                        "Linux-Subsystem-for-Arxumbra"
                    } else if cfg!(target_os = "macos") {
                        "macOS-Subsystem-for-Arxumbra"
                    } else {
                        "Windows-Subsystem-for-Arxumbra"
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
             mut focus_state: bevy::ecs::system::Local<'_, (bool, u16, u8, bool)>,
             mut escape_state: bevy::ecs::system::Local<
                '_,
                (std::option::Option<std::time::Instant>, bool, bool, bool),
            >,
             mut app_state: bevy::ecs::system::Local<'_, [u8; 4]>,
             mut dot_state: bevy::ecs::system::Local<
                '_,
                std::option::Option<(
                    bevy::ecs::entity::Entity,
                    bevy::math::Vec2,
                    bevy::math::Vec2,
                    bevy::math::Vec2,
                )>,
            >,
             mut metrics_state: bevy::ecs::system::Local<
                '_,
                (f32, f32, bevy::math::Vec2, f32),
            >,
             mut anim_state: bevy::ecs::system::Local<'_, ([f32; 4], [f32; 4], [f32; 5])>,
             mut asset_handles: bevy::ecs::system::Local<
                '_,
                std::option::Option<[bevy::asset::Handle<bevy::image::Image>; 12]>,
            >,
             mut ui_entities: bevy::ecs::system::Local<
                '_,
                std::option::Option<[bevy::ecs::entity::Entity; 45]>,
            >,
             (mut tap_state, mut key_events): (
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
                    },
                );
                windows.single().into_iter().for_each(|window: &bevy::window::Window| {
                    if focus_state.2 == 0u8 {
                        anim_state.2 = [0.0, 0.0, 0.0, 0.0, 1.0];
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
                                        244, 0, 0, 1, 51, 73, 68, 65, 84, 120, 218, 213, 87, 203,
                                        13, 131, 48, 12, 69, 140, 128, 184, 32, 36, 6, 96, 40, 78,
                                        28, 88, 128, 29, 152, 2, 134, 96, 2, 102, 65, 12, 208, 11,
                                        23, 132, 220, 188, 42, 7, 72, 13, 229, 19, 20, 215, 210,
                                        147, 80, 113, 252, 94, 147, 23, 147, 120, 222, 63, 70, 16,
                                        4, 190, 66, 162, 144, 106, 224, 217, 127, 154, 52, 86, 40,
                                        21, 58, 133, 81, 129, 12, 140, 250, 29, 114, 98, 219, 196,
                                        141, 194, 196, 144, 110, 97, 210, 99, 226, 187, 228, 153,
                                        194, 235, 4, 177, 9, 140, 205, 174, 146, 87, 55, 136, 77,
                                        84, 86, 200, 163, 40, 162, 162, 40, 168, 109, 91, 234, 251,
                                        158, 230, 121, 254, 0, 207, 248, 13, 239, 144, 115, 75, 132,
                                        158, 246, 175, 2, 121, 158, 211, 48, 12, 244, 43, 144, 131,
                                        220, 13, 17, 217, 17, 195, 173, 214, 60, 12, 67, 170, 235,
                                        154, 206, 6, 198, 96, 44, 227, 137, 120, 79, 64, 99, 170,
                                        190, 66, 190, 20, 193, 204, 66, 179, 247, 239, 39, 115, 218,
                                        239, 6, 179, 28, 19, 59, 11, 186, 129, 172, 12, 119, 100,
                                        205, 143, 120, 130, 49, 102, 201, 9, 232, 150, 73, 112, 180,
                                        173, 64, 45, 67, 64, 199, 245, 246, 85, 123, 197, 182, 178,
                                        21, 168, 197, 180, 109, 127, 41, 32, 49, 205, 130, 189, 109,
                                        43, 80, 139, 49, 99, 178, 20, 144, 154, 9, 104, 48, 182, 2,
                                        181, 24, 1, 169, 40, 1, 206, 151, 192, 173, 9, 157, 111, 67,
                                        41, 141, 200, 109, 43, 118, 254, 49, 18, 241, 57, 118, 126,
                                        32, 17, 113, 36, 19, 113, 40, 21, 113, 44, 23, 113, 49, 17,
                                        113, 53, 19, 115, 57, 125, 42, 222, 204, 155, 112, 199, 253,
                                        152, 129, 95, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
                                    ],
                                    bevy::image::ImageType::Extension("png"),
                                    bevy::image::CompressedImageFormats::NONE,
                                    true,
                                    bevy::image::ImageSampler::Default,
                                    bevy::asset::RenderAssetUsages::RENDER_WORLD,
                                )
                                .expect("embedded UI texture must decode"),
                            )
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
                            ]: [bevy::asset::Handle<bevy::image::Image>; 12]| {
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
                                                image: right_round_solid,
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
                                                image: app_icon,
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
                                                    "Linux-Subsystem-for-Arxumbra"
                                                } else if cfg!(target_os = "macos") {
                                                    "macOS-Subsystem-for-Arxumbra"
                                                } else {
                                                    "Windows-Subsystem-for-Arxumbra"
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
                                                image: home_icon,
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
                                                image: plus_icon,
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
                                                image: registry_icon,
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
                                                image: workspace_icon,
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
                                                image: flask_icon,
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
                                                image: round_outline,
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
                                                image: round_solid,
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
                                                image: rocket_icon,
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
                                                image: panel_round_solid,
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
                                            if window.focused {
                                                bevy::camera::visibility::Visibility::Inherited
                                            } else {
                                                bevy::camera::visibility::Visibility::Hidden
                                            },
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
                        .for_each(|entities: &[bevy::ecs::entity::Entity; 45]| {
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
                                (metrics_state.2.x * 0.38)
                                    .clamp(360.0, 560.0)
                                    .min((metrics_state.2.x - 48.0).max(180.0)),
                                (metrics_state.2.y * 0.36)
                                    .clamp(240.0, 360.0)
                                    .min((metrics_state.2.y - 48.0).max(140.0)),
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
                                            std::iter::once(
                                                match if !window.focused {
                                                    std::option::Option::None
                                                } else if cur_modal && cur_mode == 2u8 {
                                                    if modal_rect.contains(dot_pos)
                                                        && ((dot_pos.x - modal_rect.center().x)
                                                            .abs()
                                                            - (modal_rect.width() * 0.5 - 14.0))
                                                            .max(0.0)
                                                            .hypot(
                                                                ((dot_pos.y
                                                                    - modal_rect.center().y)
                                                                    .abs()
                                                                    - (modal_rect.height() * 0.5
                                                                        - 14.0))
                                                                    .max(0.0),
                                                            )
                                                            <= 14.0
                                                    {
                                                        std::option::Option::None
                                                    } else {
                                                        std::option::Option::Some(4u8)
                                                    }
                                                } else if cur_mode != 2u8 {
                                                    if cur_menu
                                                        && row_one_rect.contains(dot_pos)
                                                        && ((dot_pos.x - row_one_rect.center().x)
                                                            .abs()
                                                            - (row_one_rect.width() * 0.5 - 9.0))
                                                            .max(0.0)
                                                            .hypot(
                                                                ((dot_pos.y
                                                                    - row_one_rect.center().y)
                                                                    .abs()
                                                                    - (row_one_rect.height() * 0.5
                                                                        - 9.0))
                                                                    .max(0.0),
                                                            )
                                                            <= 9.0
                                                    {
                                                        std::option::Option::Some(0u8)
                                                    } else if cur_menu
                                                        && row_two_rect.contains(dot_pos)
                                                        && ((dot_pos.x - row_two_rect.center().x)
                                                            .abs()
                                                            - (row_two_rect.width() * 0.5 - 9.0))
                                                            .max(0.0)
                                                            .hypot(
                                                                ((dot_pos.y
                                                                    - row_two_rect.center().y)
                                                                    .abs()
                                                                    - (row_two_rect.height() * 0.5
                                                                        - 9.0))
                                                                    .max(0.0),
                                                            )
                                                            <= 9.0
                                                    {
                                                        std::option::Option::Some(1u8)
                                                    } else if cur_mode == 1u8
                                                        && metrics_state.0 > 0.0
                                                        && launch_rect.contains(dot_pos)
                                                        && ((dot_pos.x - launch_rect.center().x)
                                                            .abs()
                                                            - (launch_rect.width() * 0.5 - 9.0))
                                                            .max(0.0)
                                                            .hypot(
                                                                ((dot_pos.y
                                                                    - launch_rect.center().y)
                                                                    .abs()
                                                                    - (launch_rect.height() * 0.5
                                                                        - 9.0))
                                                                    .max(0.0),
                                                            )
                                                            <= 9.0
                                                    {
                                                        std::option::Option::Some(2u8)
                                                    } else {
                                                        std::option::Option::None
                                                    }
                                                } else if cur_menu
                                                    && row_one_rect.contains(dot_pos)
                                                    && ((dot_pos.x - row_one_rect.center().x)
                                                        .abs()
                                                        - (row_one_rect.width() * 0.5 - 9.0))
                                                        .max(0.0)
                                                        .hypot(
                                                            ((dot_pos.y - row_one_rect.center().y)
                                                                .abs()
                                                                - (row_one_rect.height() * 0.5
                                                                    - 9.0))
                                                                .max(0.0),
                                                        )
                                                        <= 9.0
                                                {
                                                    std::option::Option::Some(3u8)
                                                } else {
                                                    std::option::Option::None
                                                } {
                                                    std::option::Option::Some(0u8 | 1u8 | 3u8)
                                                        if !(bevy::math::Rect::new(
                                                            0.0, 0.0, sw, vh,
                                                        )
                                                        .contains(dot_pos)
                                                            && (dot_pos.x - (sw - 14.0))
                                                                .max(0.0)
                                                                .hypot(
                                                                    ((dot_pos.y - vh * 0.5).abs()
                                                                        - (vh * 0.5 - 14.0))
                                                                        .max(0.0),
                                                                )
                                                                <= 14.0) =>
                                                    {
                                                        std::option::Option::None
                                                    }
                                                    other => other,
                                                },
                                            )
                                            .for_each(|hit_target: std::option::Option<u8>| {
                                                std::iter::once(
                                                    if !escape_state.3
                                                        && focus_state.3
                                                        && tap_state.1
                                                        && (focus_state.1 & 64u16) == 0
                                                    {
                                                        match hit_target {
                                                            std::option::Option::Some(0u8) => {
                                                                (0u8, cur_menu, false)
                                                            }
                                                            std::option::Option::Some(1u8) => {
                                                                (1u8, cur_menu, false)
                                                            }
                                                            std::option::Option::Some(2u8)
                                                                if cur_mode == 1u8
                                                                    && !cur_modal =>
                                                            {
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
                                                            std::option::Option::Some(3u8)
                                                                if cur_mode == 2u8
                                                                    && !cur_modal =>
                                                            {
                                                                (2u8, cur_menu, true)
                                                            }
                                                            std::option::Option::Some(4u8)
                                                                if cur_mode == 2u8
                                                                    && cur_modal =>
                                                            {
                                                                (2u8, cur_menu, false)
                                                            }
                                                            _ => (cur_mode, cur_menu, cur_modal),
                                                        }
                                                    } else {
                                                        (cur_mode, cur_menu, cur_modal)
                                                    },
                                                )
                                                .for_each(
                                                    |(next_mode, next_menu, next_modal): (
                                                        u8,
                                                        bool,
                                                        bool,
                                                    )| {
                                                        *app_state = [
                                                            next_mode,
                                                            u8::from(!next_menu),
                                                            u8::from(next_modal),
                                                            if !window.focused || next_modal {
                                                                0u8
                                                            } else {
                                                                match hit_target {
                                                                    std::option::Option::Some(0u8)
                                                                        if next_mode != 2u8
                                                                            && next_menu =>
                                                                    {
                                                                        1u8
                                                                    }
                                                                    std::option::Option::Some(1u8)
                                                                        if next_mode != 2u8
                                                                            && next_menu =>
                                                                    {
                                                                        2u8
                                                                    }
                                                                    std::option::Option::Some(2u8)
                                                                        if next_mode == 1u8 =>
                                                                    {
                                                                        3u8
                                                                    }
                                                                    std::option::Option::Some(3u8)
                                                                        if next_mode == 2u8
                                                                            && next_menu =>
                                                                    {
                                                                        4u8
                                                                    }
                                                                    _ => 0u8,
                                                                }
                                                            },
                                                        ];
                                                    },
                                                );
                                            });
                                        },
                                    );

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
                                            [0u8, 1u8, 2u8, 3u8].into_iter().for_each(
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
                                                    if final_mode == 0u8 { 0.0f32 } else { 1.0f32 },
                                                    18.0f32,
                                                ),
                                                (
                                                    1usize,
                                                    if final_mode == 0u8 { 0.0f32 } else { 1.0f32 },
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
                                                |e: [bevy::ecs::entity::Entity; 45]| {
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
                                                                .zip([0u8, 1u8, 2u8, 3u8])
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
                                                                .zip([0u8, 1u8, 2u8, 3u8])
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
