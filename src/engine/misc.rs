use std::io::Read;
use windows::core::PCSTR;

pub fn create_video_subsystem(sdl_context: &sdl2::Sdl) -> Result<sdl2::VideoSubsystem, String> {
    let video_subsystem = sdl_context.video()?;
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);

    Ok(video_subsystem)
}

// discarding GLContext will cause a panic
pub fn create_window(
    video_subsystem: &sdl2::VideoSubsystem,
) -> Result<(sdl2::video::Window, sdl2::video::GLContext), String> {
    let window = video_subsystem
        .window("SMT Nocturne Memory Editor 1.5", 950, 700)
        .allow_highdpi()
        .opengl()
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let gl_context = window.gl_create_context()?;
    window.gl_make_current(&gl_context)?;
    window.subsystem().gl_set_swap_interval(1)?;

    Ok((window, gl_context))
}

pub fn create_imgui() -> Result<imgui::Context, String> {
    let bytes: Vec<u8> = std::fs::File::open("resources/arial.ttf")
        .map_err(|e| e.to_string())?
        .bytes()
        .filter_map(|byte| byte.ok())
        .collect();

    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);
    imgui.fonts().add_font(&[imgui::FontSource::TtfData {
        data: &bytes,
        size_pixels: 18.0,
        config: None,
    }]);

    Ok(imgui)
}

pub fn glow_context(window: &sdl2::video::Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

pub fn pcstr(s: String) -> (PCSTR, String) {
    let nt = s + "\0";
    (PCSTR::from_raw(nt.as_ptr()), nt)
}
