use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use windows::Win32::Foundation::POINT;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    self, GetAsyncKeyState, VIRTUAL_KEY, VK_LBUTTON, VK_MBUTTON, VK_RBUTTON, VK_XBUTTON1,
    VK_XBUTTON2,
};
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

#[derive(Debug, Clone)]
pub struct DeviceState;

#[derive(Debug, Clone)]
pub struct MouseState {
    pub point: POINT,
    pub buttons: Vec<(VIRTUAL_KEY, bool)>,
}

impl DeviceState {
    pub fn new() -> Self {
        Self {}
    }

    pub fn query_pointer(&self) -> MouseState {
        let point = &mut POINT { x: 0, y: 0 };
        let button1pressed;
        let button2pressed;
        let button3pressed;
        let button4pressed;
        let button5pressed;

        unsafe {
            let _ = GetCursorPos(point);

            button1pressed =
                GetAsyncKeyState(KeyboardAndMouse::VK_LBUTTON.0 as i32) as u32 & 0x8000 != 0;
            button2pressed =
                GetAsyncKeyState(KeyboardAndMouse::VK_RBUTTON.0 as i32) as u32 & 0x8000 != 0;
            button3pressed =
                GetAsyncKeyState(KeyboardAndMouse::VK_MBUTTON.0 as i32) as u32 & 0x8000 != 0;
            button4pressed =
                GetAsyncKeyState(KeyboardAndMouse::VK_XBUTTON1.0 as i32) as u32 & 0x8000 != 0;
            button5pressed =
                GetAsyncKeyState(KeyboardAndMouse::VK_XBUTTON2.0 as i32) as u32 & 0x8000 != 0;
        }

        let buttons = vec![
            (VK_LBUTTON, button1pressed),
            (VK_RBUTTON, button2pressed),
            (VK_MBUTTON, button3pressed),
            (VK_XBUTTON1, button4pressed),
            (VK_XBUTTON2, button5pressed),
        ];

        MouseState {
            point: *point,
            buttons,
        }
    }

    pub fn query_keymap(&self) -> Vec<VIRTUAL_KEY> {
        let mut pressed_keys = Vec::new();
        for key in 1..256u16 {
            if unsafe { GetAsyncKeyState(key as i32) } & 0x01 != 0 {
                pressed_keys.push(VIRTUAL_KEY(key));
            }
        }
        pressed_keys
    }
}

pub fn prompt_for_key(prompt: &str) -> VIRTUAL_KEY {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let device_state = DeviceState::new();
    let mut last_pressed_keys: Vec<VIRTUAL_KEY> = device_state.query_keymap();
    let mut last_mouse_state = device_state.query_pointer();

    loop {
        // Delay to prevent high CPU usage and allow state change
        thread::sleep(Duration::from_millis(100));

        // Check for key press
        let pressed_keys = device_state.query_keymap();
        for &vk in &pressed_keys {
            if !last_pressed_keys.contains(&vk) {
                println!("{:?} -> 0x{:X}", vk.0, vk.0);
                return vk;
            }
        }
        last_pressed_keys = pressed_keys;

        // Check for mouse button press
        let mouse_state = device_state.query_pointer();
        for (i, (vk, pressed)) in mouse_state.buttons.iter().enumerate() {
            if *pressed && !last_mouse_state.buttons[i].1 {
                println!("{:?} -> 0x{:X}", vk, vk.0);
                return *vk;
            }
        }

        last_mouse_state = mouse_state;
    }
}
