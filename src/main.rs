#![windows_subsystem = "windows"]

use std::time::{Duration, SystemTime};

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging as winmsg;

use glow::HasContext;
use imgui_glow_renderer as igr;
use imgui_sdl2_support as iss;
use sdl2::event::Event;

mod engine;
use engine::{
    data::{Data, SKILL_SLOT_COUNT},
    misc,
    skill::{Gamemode, Skill},
};

fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = misc::create_video_subsystem(&sdl_context)?;
    let (window, _gc) = misc::create_window(&video_subsystem)?;
    let gl = misc::glow_context(&window);
    let engine = engine::Engine::new()?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut imgui = misc::create_imgui()?;
    let mut platform = iss::SdlPlatform::init(&mut imgui);
    let mut renderer = igr::AutoRenderer::initialize(gl, &mut imgui).map_err(|e| e.to_string())?;

    let mut gamemode = Gamemode::Vanilla;
    let mut data = Data::default();
    let mut selection = 0usize;
    let mut rng = rand::thread_rng();
    let mut last_shuffle = SystemTime::now();

    let get_tables = |gamemode: Gamemode| -> (Vec<&Skill>, Vec<&Skill>) {
        let mut table: Vec<&Skill> = Skill::table(gamemode).iter().collect();

        let s_table: Vec<&Skill> = table.iter().filter(|x| x.id > 0).map(|&x| x).collect();

        table.sort();

        (table, s_table)
    };

    let (mut table, mut s_table) = get_tables(gamemode);

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            platform.handle_event(&mut imgui, &event);

            match event {
                Event::Quit { .. } => break 'main_loop,
                _ => {}
            }
        }

        platform.prepare_frame(&mut imgui, &window, &event_pump);
        data = engine.bulk_read(data)?;
        let mut value_changed = false;
        let ui = imgui.new_frame();

        if let Some(_token) = ui
            .window("Stats")
            .position([0.0, 0.0], imgui::Condition::FirstUseEver)
            .size([350.0, 400.0], imgui::Condition::FirstUseEver)
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .begin()
        {
            macro_rules! lock_and_input {
                ($label: expr, $field: tt) => {
                    ui.checkbox(format!("##{}", $label), &mut data.$field.locked);
                    ui.same_line();
                    ui.set_next_item_width(200.0);
                    value_changed |= ui.input_int($label, &mut data.$field.value).build();
                };
            }

            lock_and_input!("HP", hp);
            lock_and_input!("Max HP", max_hp);
            lock_and_input!("MP", mp);
            lock_and_input!("Max MP", max_mp);

            ui.separator();

            lock_and_input!("Level", level);
            lock_and_input!("EXP", exp);

            ui.separator();

            lock_and_input!("St", st);
            lock_and_input!("Ma", ma);
            lock_and_input!("Vi", vi);
            lock_and_input!("Ag", ag);
            lock_and_input!("Lu", lu);

            ui.separator();

            lock_and_input!("Macca", macca);
        }
        if let Some(_token) = ui
            .window("Skill Slots")
            .position([0.0, 400.0], imgui::Condition::FirstUseEver)
            .size([350.0, 300.0], imgui::Condition::FirstUseEver)
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .begin()
        {
            ui.columns(2, "col_skill_slots", false);
            let skill_availability = data.skill_availability();
            if skill_availability < selection as u8 {
                selection = 0;
            }

            macro_rules! radio_button {
                ($idx: expr) => {
                    let label = format!(
                        "{}##{}",
                        Skill::get(gamemode, data.skills[$idx].value as usize)?.name,
                        $idx
                    );
                    ui.radio_button(label, &mut selection, $idx);
                };
            }

            ui.checkbox("Random", &mut data.random);
            ui.separator();

            for idx in 0..SKILL_SLOT_COUNT {
                if idx == SKILL_SLOT_COUNT / 2 {
                    ui.separator();
                }
                if idx != 0 && idx % (SKILL_SLOT_COUNT / 4) == 0 {
                    ui.next_column();
                }

                if skill_availability < idx as u8 {
                    let _token_1 = ui.begin_disabled(true);
                    radio_button!(idx);
                } else {
                    radio_button!(idx);
                }
            }
        }
        if let Some(_token) = ui
            .window("Gamemode")
            .position([350.0, 0.0], imgui::Condition::FirstUseEver)
            .size([600.0, 70.0], imgui::Condition::FirstUseEver)
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .begin()
        {
            let mut clicked = false;

            ui.columns(2, "col_gamemode", false);
            clicked |= ui.radio_button("Vanilla", &mut gamemode, Gamemode::Vanilla);
            ui.next_column();
            clicked |= ui.radio_button("HardType", &mut gamemode, Gamemode::HardType);

            if clicked {
                (table, s_table) = get_tables(gamemode);
            }
        }
        if let Some(_token) = ui
            .window("Skill Table")
            .position([350.0, 70.0], imgui::Condition::FirstUseEver)
            .size([600.0, 630.0], imgui::Condition::FirstUseEver)
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .begin()
        {
            ui.columns(4, "col_skill_table", false);

            for (idx, &skill) in table.iter().enumerate() {
                if idx != 0 && idx % (table.len() / 4) == 0 {
                    ui.next_column();
                }
                if ui.button(format!("{}##{}", skill.name, idx)) {
                    value_changed = true;
                    data.skills[selection].value = skill.id as i32;
                }
            }
        }

        if data.random {
            let now = SystemTime::now();
            let d = now
                .duration_since(last_shuffle)
                .map_err(|why| why.to_string())?;

            if d > Duration::from_millis(33) {
                last_shuffle = now;
                use rand::prelude::SliceRandom;
                s_table.shuffle(&mut rng);
                for idx in 0..8 {
                    data.skills[idx].value = s_table[idx].id as i32;
                }
            }
        }

        if value_changed {
            engine.bulk_write(&data)?;
        }
        engine.write_locked_only(&data)?;

        let draw_data = imgui.render();
        unsafe {
            renderer.gl_context().clear(glow::COLOR_BUFFER_BIT);
        }
        renderer.render(draw_data)?;
        window.gl_swap_window();
    }

    Ok(())
}

fn main() {
    if let Err(why) = run() {
        unsafe {
            winmsg::MessageBoxA(
                HWND::default(),
                misc::pcstr(why),
                misc::pcstr("Error".to_string()),
                winmsg::MB_ICONERROR,
            );
        }
    }
}
