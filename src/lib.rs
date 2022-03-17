#![feature(proc_macro_hygiene)]
#![feature(c_variadic)]
use skyline::libc::{c_char, c_float, c_int, c_uint, c_ulong, c_ushort, c_void, printf};
use skyline::{hook, hooks::InlineCtx, install_hook, nn};
use std::ffi::{CStr, CString};
use std::fs;
use std::path::Path;
use std::ptr::null_mut;
pub mod constants;
use constants::*;
#[macro_use]
extern crate lazy_static;

#[repr(C)]
#[derive(Clone)]
pub struct Color {
    pub r: c_float,
    pub g: c_float,
    pub b: c_float,
    pub a: c_float,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Font {
    unk: c_ushort,  // 0x0 -
    pad: [u8; 0x2], //  0x3
    unk1: c_uint,   // 0x4
    pad1: [u8; 0x4],
    unk2: *const c_void,  //0x8
    unk3: c_uint,         //0x10
    unk4: *const c_void,  // 0x18
    unk5: c_uint,         //0x20
    unk6: *const c_ulong, //0x28
}

extern "C" {
    #[link_name = "_ZN13Entity_Player14AddCollectibleE16eCollectibleTypeib"]
    pub fn AddCollectible(
        this: *const c_void,
        ItemType: c_int,
        Charge: c_int,
        FirstTimePickedUp: bool,
    );
    #[link_name = "_ZN4KAGE5Input15InputDeviceBase17IsButtonTriggeredEj"]
    pub fn isButtonTriggered(this: *const c_void, button: Buttons) -> bool;
    #[link_name = "_Z11IsaacUpdatev"]
    pub fn Update();
    #[link_name = "_ZN4Game5StartE11ePlayerType10eChallenge5SeedsNS_11eDifficultyE"]
    pub fn GameStart(
        this: *const c_void,
        player_type: c_int,
        challenge_type: c_int,
        seeds: c_int,
        difficulty: c_int,
    );
    #[link_name = "_ZN4KAGE7Filesys18BufferedFileStream8OpenReadEPKc"]
    pub fn OpenRead(this: *const c_void, path: *const c_char);
    #[link_name = "_ZN13Entity_PlayerC1Ev"]
    pub fn Entity_Player_Constructor(this: *const c_void);
    #[link_name = "_ZN4KAGE5Input13DeviceGamepadC2Ev"]
    pub fn InputDeviceGamepad_Constructor(this: *const c_void);
    #[link_name = "_ZN4KAGE8Graphics4FontC1Ev"]
    pub fn Font_Constructor(this: *const Font);
    #[link_name = "_ZN4KAGE8Graphics4Font10DrawStringEPKcffNS0_5ColorEjb"]
    pub fn DrawString(
        this: *const c_void,
        msg: *const c_char,
        x: c_float,
        y: c_float,
        color: Color,
        allign: c_uint,
        center: bool,
    );
    #[link_name = "KAGE_LogMessage"]
    pub fn LogMessage(num: c_uint, msg: *const c_char);
    // #[link_name = "_ZN4KAGE8Graphics5ColorC2Effff"]
    //pub fn Color_cConstructor(this: *const Color, R: c_float, G: c_float, B: c_float, A: c_float);
    #[link_name = "_ZN4KAGE8Graphics4Font4LoadEPKc"]
    pub fn FontLoad(this: *const c_void, path: *const c_char);
    #[link_name = "_ZN4KAGE8Graphics5ColorC2ERKS1_"]
    pub fn Color_Constructor(new: *const Color, original: *const Color);
    #[link_name = "_ZN4KAGE7Filesys18BufferedFileStreamC2Ev"]
    pub fn BufferedFileSystem_Constructor(this: *const c_void);
}
static mut Entity_Player: *const c_void = null_mut();
static mut InputDeviceGamepad: *const c_void = null_mut();
static mut Font: *const c_void = null_mut();
static mut FileBuffer: *const c_void = null_mut();

static mut curr_id: c_int = 0;
static mut initialized: bool = false;
static mut test: bool = true;
static mut got_font: bool = false;

//  #[hook(replace = InputDeviceGamepad_Constructor)]
//  pub fn GetGamepad(this: *const c_void) {
//      // unsafe { InputDeviceGamepad = this;}
//      call_original!(this);
//  }

#[hook(replace = Color_Constructor)]
pub fn MakeColor(mut new: *const Color, old: *const Color) {
    //new = old.clone();
    call_original!(new, old);
}

#[hook(replace = BufferedFileSystem_Constructor)]
pub fn GetFileBuffer(this: *const c_void) {
    unsafe { FileBuffer = this };
}

#[hook(replace = Entity_Player_Constructor)]
pub fn GetPlayer(this: *const c_void) {
    unsafe {
        Entity_Player = this;
    }
    call_original!(this);
}

#[hook(replace = isButtonTriggered)]
pub fn GetGamepad(this: *const c_void, btn: c_uint) {
    unsafe {
        if !initialized {
            InputDeviceGamepad = this;
        }
    }
    return call_original!(this, btn);
}

#[hook(replace = Font_Constructor)]
pub fn GetFont(this: *const c_void) {
    unsafe {
        //  if !got_font {
        // Font = this.clone();
        Font = this;
        //     got_font = true;
        //}
        call_original!(this);
    }
}

#[hook(replace = LogMessage)]
unsafe extern "C" fn LogMessage_to_stdout(num: c_uint, msg: *const c_char, args: ...) {
    // let msg = unsafe { CStr::from_ptr(msg as *const i8).to_string_lossy() };
    //printf(msg);
}

#[hook(replace = Update)]
pub fn Update_replace() {
    main_loop();
    call_original!();
}
#[hook(replace = GameStart)]
pub fn initialized_replacement(player_type: i32, challenge_type: u32, seeds: i32, difficulty: i32) {
    unsafe { initialized = true };
    call_original!(player_type, challenge_type, seeds, difficulty)
}

#[hook(replace = nn::fs::OpenFile)]
pub fn open_file_replace(File: nnsdk::fs::FileHandle, path: *const c_char, idk: i32) {
    let c_str = unsafe { CStr::from_ptr(path as *const i8) };
    println!("opening file: {}", c_str.to_str().unwrap());
    call_original!(File, path, idk);
}
#[skyline::main(name = "isek_stuff")]
pub fn main() {
    //install_hook!(open_file_replace); // print file opening info
    //install_hook!(GetFileBuffer);
    //install_hook!(LogMessage_to_stdout); // print log to stdout instead of whatever KAGE::LogMessage does.
    install_hook!(MakeColor);
    install_hook!(Update_replace); // call MainLoop each tick
    //install_hook!(GetFont); // get a base font for cloning
    install_hook!(GetPlayer); // get player entity
    install_hook!(GetGamepad); // get gamepad addr used by game
    install_hook!(initialized_replacement); // set `initialize` variable
    println!("Hello from Skyline Rust Plugin!");
}

fn print_item(id: c_int) {
    println!("ID: {}", id);
    let result = match get_item_text(id, false) {
        Ok(String) => String,
        Err(msg) => format!("Couldn't print item: {}", msg),
    };

    println!("{}", result)
}

fn get_item_text(id: c_int, show_desc: bool) -> Result<String, String> {
    if id > NUM_ITEMS {
        return Err(format!("ID {} is out of bounds!", id));
    }
    if !ITEM_NAMES.contains_key(&id) {
        return Err(format!("ID {} is not in ITEM_NAMES", id));
    }
    let mut temp_str = format!("{}", ITEM_NAMES[&id]); //print name
    if show_desc {
        temp_str.push_str(format!("{}", ITEM_DESCRIPTIONS[&id]).as_str());
    }
    Ok(temp_str)
}

pub fn chars(string: &str) -> *const c_char {
    let c_str = CString::new(string).unwrap();
    let c_word: *const c_char = c_str.as_ptr() as *const c_char;
    c_word
}

pub fn main_loop() {
    unsafe {
        if initialized {
            unsafe {
                let isek_stuff = fs::read_to_string(Path::new("sd:/wheel_of_isek/curr_run.txt"))
                    .unwrap();
                for item in isek_stuff.split('\n') {
                    let item = item.trim().parse::<i32>().unwrap();
                    print!("Adding item: ");
                    print_item(item as i32);
                    AddCollectible(Entity_Player, item, 6, true);
                }

                if isButtonTriggered(InputDeviceGamepad, Buttons::L) {
                    if curr_id - 1 > 0 {
                        curr_id -= 1;
                    }
                    print_item(curr_id);
                }
                if isButtonTriggered(InputDeviceGamepad, Buttons::ZL) {
                    if curr_id - 5 > 0 {
                        curr_id -= 5;
                        print_item(curr_id);
                    }
                }
                if isButtonTriggered(InputDeviceGamepad, Buttons::R) {
                    curr_id += 1;
                    print_item(curr_id);
                }
                if isButtonTriggered(InputDeviceGamepad, Buttons::ZR) {
                    curr_id += 5;
                    print_item(curr_id);
                }
            }
            if isButtonTriggered(InputDeviceGamepad, Buttons::X) {
                AddCollectible(Entity_Player, curr_id, 0, true);
                println!("Added Collectible");
            }
        }
    }
}
