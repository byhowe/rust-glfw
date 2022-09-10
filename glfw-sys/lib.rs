#![allow(bad_style)]

// ** generated using **
// let bindings = bindgen::Builder::default()
//     .header_contents("no_opengl", "#define GLFW_INCLUDE_NONE")
//     .header("GLFW/glfw3.h")
//     .use_core()
//     .ctypes_prefix("::core::ffi")
//     .allowlist_var("GLFW_.*")
//     .allowlist_function("glfw.*")
//     .allowlist_type("GLFW.*")
//     .blocklist_type("__uint32_t")
//     .blocklist_type("__uint64_t")
//     .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
//     .generate_comments(false)
//     .layout_tests(false)
//     .generate()
//     .expect("unable to generate bindings");

// init
pub const GLFW_TRUE: i32 = 1;
pub const GLFW_FALSE: i32 = 0;

// input
pub const GLFW_RELEASE: i32 = 0;
pub const GLFW_PRESS: i32 = 1;
pub const GLFW_REPEAT: i32 = 2;

// hat states
pub const GLFW_HAT_CENTERED: i32 = 0;
pub const GLFW_HAT_UP: i32 = 1;
pub const GLFW_HAT_RIGHT: i32 = 2;
pub const GLFW_HAT_DOWN: i32 = 4;
pub const GLFW_HAT_LEFT: i32 = 8;
pub const GLFW_HAT_RIGHT_UP: i32 = 3;
pub const GLFW_HAT_RIGHT_DOWN: i32 = 6;
pub const GLFW_HAT_LEFT_UP: i32 = 9;
pub const GLFW_HAT_LEFT_DOWN: i32 = 12;

// key codes
pub const GLFW_KEY_UNKNOWN: i32 = -1;
pub const GLFW_KEY_SPACE: i32 = 32;
pub const GLFW_KEY_APOSTROPHE: i32 = 39;
pub const GLFW_KEY_COMMA: i32 = 44;
pub const GLFW_KEY_MINUS: i32 = 45;
pub const GLFW_KEY_PERIOD: i32 = 46;
pub const GLFW_KEY_SLASH: i32 = 47;
pub const GLFW_KEY_0: i32 = 48;
pub const GLFW_KEY_1: i32 = 49;
pub const GLFW_KEY_2: i32 = 50;
pub const GLFW_KEY_3: i32 = 51;
pub const GLFW_KEY_4: i32 = 52;
pub const GLFW_KEY_5: i32 = 53;
pub const GLFW_KEY_6: i32 = 54;
pub const GLFW_KEY_7: i32 = 55;
pub const GLFW_KEY_8: i32 = 56;
pub const GLFW_KEY_9: i32 = 57;
pub const GLFW_KEY_SEMICOLON: i32 = 59;
pub const GLFW_KEY_EQUAL: i32 = 61;
pub const GLFW_KEY_A: i32 = 65;
pub const GLFW_KEY_B: i32 = 66;
pub const GLFW_KEY_C: i32 = 67;
pub const GLFW_KEY_D: i32 = 68;
pub const GLFW_KEY_E: i32 = 69;
pub const GLFW_KEY_F: i32 = 70;
pub const GLFW_KEY_G: i32 = 71;
pub const GLFW_KEY_H: i32 = 72;
pub const GLFW_KEY_I: i32 = 73;
pub const GLFW_KEY_J: i32 = 74;
pub const GLFW_KEY_K: i32 = 75;
pub const GLFW_KEY_L: i32 = 76;
pub const GLFW_KEY_M: i32 = 77;
pub const GLFW_KEY_N: i32 = 78;
pub const GLFW_KEY_O: i32 = 79;
pub const GLFW_KEY_P: i32 = 80;
pub const GLFW_KEY_Q: i32 = 81;
pub const GLFW_KEY_R: i32 = 82;
pub const GLFW_KEY_S: i32 = 83;
pub const GLFW_KEY_T: i32 = 84;
pub const GLFW_KEY_U: i32 = 85;
pub const GLFW_KEY_V: i32 = 86;
pub const GLFW_KEY_W: i32 = 87;
pub const GLFW_KEY_X: i32 = 88;
pub const GLFW_KEY_Y: i32 = 89;
pub const GLFW_KEY_Z: i32 = 90;
pub const GLFW_KEY_LEFT_BRACKET: i32 = 91;
pub const GLFW_KEY_BACKSLASH: i32 = 92;
pub const GLFW_KEY_RIGHT_BRACKET: i32 = 93;
pub const GLFW_KEY_GRAVE_ACCENT: i32 = 96;
pub const GLFW_KEY_WORLD_1: i32 = 161;
pub const GLFW_KEY_WORLD_2: i32 = 162;

// function keys
pub const GLFW_KEY_ESCAPE: i32 = 256;
pub const GLFW_KEY_ENTER: i32 = 257;
pub const GLFW_KEY_TAB: i32 = 258;
pub const GLFW_KEY_BACKSPACE: i32 = 259;
pub const GLFW_KEY_INSERT: i32 = 260;
pub const GLFW_KEY_DELETE: i32 = 261;
pub const GLFW_KEY_RIGHT: i32 = 262;
pub const GLFW_KEY_LEFT: i32 = 263;
pub const GLFW_KEY_DOWN: i32 = 264;
pub const GLFW_KEY_UP: i32 = 265;
pub const GLFW_KEY_PAGE_UP: i32 = 266;
pub const GLFW_KEY_PAGE_DOWN: i32 = 267;
pub const GLFW_KEY_HOME: i32 = 268;
pub const GLFW_KEY_END: i32 = 269;
pub const GLFW_KEY_CAPS_LOCK: i32 = 280;
pub const GLFW_KEY_SCROLL_LOCK: i32 = 281;
pub const GLFW_KEY_NUM_LOCK: i32 = 282;
pub const GLFW_KEY_PRINT_SCREEN: i32 = 283;
pub const GLFW_KEY_PAUSE: i32 = 284;
pub const GLFW_KEY_F1: i32 = 290;
pub const GLFW_KEY_F2: i32 = 291;
pub const GLFW_KEY_F3: i32 = 292;
pub const GLFW_KEY_F4: i32 = 293;
pub const GLFW_KEY_F5: i32 = 294;
pub const GLFW_KEY_F6: i32 = 295;
pub const GLFW_KEY_F7: i32 = 296;
pub const GLFW_KEY_F8: i32 = 297;
pub const GLFW_KEY_F9: i32 = 298;
pub const GLFW_KEY_F10: i32 = 299;
pub const GLFW_KEY_F11: i32 = 300;
pub const GLFW_KEY_F12: i32 = 301;
pub const GLFW_KEY_F13: i32 = 302;
pub const GLFW_KEY_F14: i32 = 303;
pub const GLFW_KEY_F15: i32 = 304;
pub const GLFW_KEY_F16: i32 = 305;
pub const GLFW_KEY_F17: i32 = 306;
pub const GLFW_KEY_F18: i32 = 307;
pub const GLFW_KEY_F19: i32 = 308;
pub const GLFW_KEY_F20: i32 = 309;
pub const GLFW_KEY_F21: i32 = 310;
pub const GLFW_KEY_F22: i32 = 311;
pub const GLFW_KEY_F23: i32 = 312;
pub const GLFW_KEY_F24: i32 = 313;
pub const GLFW_KEY_F25: i32 = 314;
pub const GLFW_KEY_KP_0: i32 = 320;
pub const GLFW_KEY_KP_1: i32 = 321;
pub const GLFW_KEY_KP_2: i32 = 322;
pub const GLFW_KEY_KP_3: i32 = 323;
pub const GLFW_KEY_KP_4: i32 = 324;
pub const GLFW_KEY_KP_5: i32 = 325;
pub const GLFW_KEY_KP_6: i32 = 326;
pub const GLFW_KEY_KP_7: i32 = 327;
pub const GLFW_KEY_KP_8: i32 = 328;
pub const GLFW_KEY_KP_9: i32 = 329;
pub const GLFW_KEY_KP_DECIMAL: i32 = 330;
pub const GLFW_KEY_KP_DIVIDE: i32 = 331;
pub const GLFW_KEY_KP_MULTIPLY: i32 = 332;
pub const GLFW_KEY_KP_SUBTRACT: i32 = 333;
pub const GLFW_KEY_KP_ADD: i32 = 334;
pub const GLFW_KEY_KP_ENTER: i32 = 335;
pub const GLFW_KEY_KP_EQUAL: i32 = 336;
pub const GLFW_KEY_LEFT_SHIFT: i32 = 340;
pub const GLFW_KEY_LEFT_CONTROL: i32 = 341;
pub const GLFW_KEY_LEFT_ALT: i32 = 342;
pub const GLFW_KEY_LEFT_SUPER: i32 = 343;
pub const GLFW_KEY_RIGHT_SHIFT: i32 = 344;
pub const GLFW_KEY_RIGHT_CONTROL: i32 = 345;
pub const GLFW_KEY_RIGHT_ALT: i32 = 346;
pub const GLFW_KEY_RIGHT_SUPER: i32 = 347;
pub const GLFW_KEY_MENU: i32 = 348;
pub const GLFW_KEY_LAST: i32 = 348;

// modifier keys
pub const GLFW_MOD_SHIFT: i32 = 1;
pub const GLFW_MOD_CONTROL: i32 = 2;
pub const GLFW_MOD_ALT: i32 = 4;
pub const GLFW_MOD_SUPER: i32 = 8;
pub const GLFW_MOD_CAPS_LOCK: i32 = 16;
pub const GLFW_MOD_NUM_LOCK: i32 = 32;

// mouse buttons
pub const GLFW_MOUSE_BUTTON_1: i32 = 0;
pub const GLFW_MOUSE_BUTTON_2: i32 = 1;
pub const GLFW_MOUSE_BUTTON_3: i32 = 2;
pub const GLFW_MOUSE_BUTTON_4: i32 = 3;
pub const GLFW_MOUSE_BUTTON_5: i32 = 4;
pub const GLFW_MOUSE_BUTTON_6: i32 = 5;
pub const GLFW_MOUSE_BUTTON_7: i32 = 6;
pub const GLFW_MOUSE_BUTTON_8: i32 = 7;
pub const GLFW_MOUSE_BUTTON_LAST: i32 = 7;
pub const GLFW_MOUSE_BUTTON_LEFT: i32 = 0;
pub const GLFW_MOUSE_BUTTON_RIGHT: i32 = 1;
pub const GLFW_MOUSE_BUTTON_MIDDLE: i32 = 2;

// joystick codes
pub const GLFW_JOYSTICK_1: i32 = 0;
pub const GLFW_JOYSTICK_2: i32 = 1;
pub const GLFW_JOYSTICK_3: i32 = 2;
pub const GLFW_JOYSTICK_4: i32 = 3;
pub const GLFW_JOYSTICK_5: i32 = 4;
pub const GLFW_JOYSTICK_6: i32 = 5;
pub const GLFW_JOYSTICK_7: i32 = 6;
pub const GLFW_JOYSTICK_8: i32 = 7;
pub const GLFW_JOYSTICK_9: i32 = 8;
pub const GLFW_JOYSTICK_10: i32 = 9;
pub const GLFW_JOYSTICK_11: i32 = 10;
pub const GLFW_JOYSTICK_12: i32 = 11;
pub const GLFW_JOYSTICK_13: i32 = 12;
pub const GLFW_JOYSTICK_14: i32 = 13;
pub const GLFW_JOYSTICK_15: i32 = 14;
pub const GLFW_JOYSTICK_16: i32 = 15;
pub const GLFW_JOYSTICK_LAST: i32 = 15;

// gamepad key codes
pub const GLFW_GAMEPAD_BUTTON_A: i32 = 0;
pub const GLFW_GAMEPAD_BUTTON_B: i32 = 1;
pub const GLFW_GAMEPAD_BUTTON_X: i32 = 2;
pub const GLFW_GAMEPAD_BUTTON_Y: i32 = 3;
pub const GLFW_GAMEPAD_BUTTON_LEFT_BUMPER: i32 = 4;
pub const GLFW_GAMEPAD_BUTTON_RIGHT_BUMPER: i32 = 5;
pub const GLFW_GAMEPAD_BUTTON_BACK: i32 = 6;
pub const GLFW_GAMEPAD_BUTTON_START: i32 = 7;
pub const GLFW_GAMEPAD_BUTTON_GUIDE: i32 = 8;
pub const GLFW_GAMEPAD_BUTTON_LEFT_THUMB: i32 = 9;
pub const GLFW_GAMEPAD_BUTTON_RIGHT_THUMB: i32 = 10;
pub const GLFW_GAMEPAD_BUTTON_DPAD_UP: i32 = 11;
pub const GLFW_GAMEPAD_BUTTON_DPAD_RIGHT: i32 = 12;
pub const GLFW_GAMEPAD_BUTTON_DPAD_DOWN: i32 = 13;
pub const GLFW_GAMEPAD_BUTTON_DPAD_LEFT: i32 = 14;
pub const GLFW_GAMEPAD_BUTTON_LAST: i32 = 14;
pub const GLFW_GAMEPAD_BUTTON_CROSS: i32 = 0;
pub const GLFW_GAMEPAD_BUTTON_CIRCLE: i32 = 1;
pub const GLFW_GAMEPAD_BUTTON_SQUARE: i32 = 2;
pub const GLFW_GAMEPAD_BUTTON_TRIANGLE: i32 = 3;

// gamepad axes
pub const GLFW_GAMEPAD_AXIS_LEFT_X: i32 = 0;
pub const GLFW_GAMEPAD_AXIS_LEFT_Y: i32 = 1;
pub const GLFW_GAMEPAD_AXIS_RIGHT_X: i32 = 2;
pub const GLFW_GAMEPAD_AXIS_RIGHT_Y: i32 = 3;
pub const GLFW_GAMEPAD_AXIS_LEFT_TRIGGER: i32 = 4;
pub const GLFW_GAMEPAD_AXIS_RIGHT_TRIGGER: i32 = 5;
pub const GLFW_GAMEPAD_AXIS_LAST: i32 = 5;

// errors
pub const GLFW_NO_ERROR: i32 = 0;
pub const GLFW_NOT_INITIALIZED: i32 = 65537;
pub const GLFW_NO_CURRENT_CONTEXT: i32 = 65538;
pub const GLFW_INVALID_ENUM: i32 = 65539;
pub const GLFW_INVALID_VALUE: i32 = 65540;
pub const GLFW_OUT_OF_MEMORY: i32 = 65541;
pub const GLFW_API_UNAVAILABLE: i32 = 65542;
pub const GLFW_VERSION_UNAVAILABLE: i32 = 65543;
pub const GLFW_PLATFORM_ERROR: i32 = 65544;
pub const GLFW_FORMAT_UNAVAILABLE: i32 = 65545;
pub const GLFW_NO_WINDOW_CONTEXT: i32 = 65546;

// window hints and attributes
pub const GLFW_FOCUSED: i32 = 131073;
pub const GLFW_ICONIFIED: i32 = 131074;
pub const GLFW_RESIZABLE: i32 = 131075;
pub const GLFW_VISIBLE: i32 = 131076;
pub const GLFW_DECORATED: i32 = 131077;
pub const GLFW_AUTO_ICONIFY: i32 = 131078;
pub const GLFW_FLOATING: i32 = 131079;
pub const GLFW_MAXIMIZED: i32 = 131080;
pub const GLFW_CENTER_CURSOR: i32 = 131081;
pub const GLFW_TRANSPARENT_FRAMEBUFFER: i32 = 131082;
pub const GLFW_HOVERED: i32 = 131083;
pub const GLFW_FOCUS_ON_SHOW: i32 = 131084;
pub const GLFW_RED_BITS: i32 = 135169;
pub const GLFW_GREEN_BITS: i32 = 135170;
pub const GLFW_BLUE_BITS: i32 = 135171;
pub const GLFW_ALPHA_BITS: i32 = 135172;
pub const GLFW_DEPTH_BITS: i32 = 135173;
pub const GLFW_STENCIL_BITS: i32 = 135174;
pub const GLFW_ACCUM_RED_BITS: i32 = 135175;
pub const GLFW_ACCUM_GREEN_BITS: i32 = 135176;
pub const GLFW_ACCUM_BLUE_BITS: i32 = 135177;
pub const GLFW_ACCUM_ALPHA_BITS: i32 = 135178;
pub const GLFW_AUX_BUFFERS: i32 = 135179;
pub const GLFW_STEREO: i32 = 135180;
pub const GLFW_SAMPLES: i32 = 135181;
pub const GLFW_SRGB_CAPABLE: i32 = 135182;
pub const GLFW_REFRESH_RATE: i32 = 135183;
pub const GLFW_DOUBLEBUFFER: i32 = 135184;
pub const GLFW_CLIENT_API: i32 = 139265;
pub const GLFW_CONTEXT_VERSION_MAJOR: i32 = 139266;
pub const GLFW_CONTEXT_VERSION_MINOR: i32 = 139267;
pub const GLFW_CONTEXT_REVISION: i32 = 139268;
pub const GLFW_CONTEXT_ROBUSTNESS: i32 = 139269;
pub const GLFW_OPENGL_FORWARD_COMPAT: i32 = 139270;
pub const GLFW_OPENGL_DEBUG_CONTEXT: i32 = 139271;
pub const GLFW_OPENGL_PROFILE: i32 = 139272;
pub const GLFW_CONTEXT_RELEASE_BEHAVIOR: i32 = 139273;
pub const GLFW_CONTEXT_NO_ERROR: i32 = 139274;
pub const GLFW_CONTEXT_CREATION_API: i32 = 139275;
pub const GLFW_SCALE_TO_MONITOR: i32 = 139276;

pub const GLFW_COCOA_RETINA_FRAMEBUFFER: i32 = 143361;
pub const GLFW_COCOA_FRAME_NAME: i32 = 143362;
pub const GLFW_COCOA_GRAPHICS_SWITCHING: i32 = 143363;

pub const GLFW_X11_CLASS_NAME: i32 = 147457;
pub const GLFW_X11_INSTANCE_NAME: i32 = 147458;

pub const GLFW_NO_API: i32 = 0;
pub const GLFW_OPENGL_API: i32 = 196609;
pub const GLFW_OPENGL_ES_API: i32 = 196610;

pub const GLFW_NO_ROBUSTNESS: i32 = 0;
pub const GLFW_NO_RESET_NOTIFICATION: i32 = 200705;
pub const GLFW_LOSE_CONTEXT_ON_RESET: i32 = 200706;

pub const GLFW_OPENGL_ANY_PROFILE: i32 = 0;
pub const GLFW_OPENGL_CORE_PROFILE: i32 = 204801;
pub const GLFW_OPENGL_COMPAT_PROFILE: i32 = 204802;

pub const GLFW_CURSOR: i32 = 208897;
pub const GLFW_STICKY_KEYS: i32 = 208898;
pub const GLFW_STICKY_MOUSE_BUTTONS: i32 = 208899;
pub const GLFW_LOCK_KEY_MODS: i32 = 208900;
pub const GLFW_RAW_MOUSE_MOTION: i32 = 208901;

pub const GLFW_CURSOR_NORMAL: i32 = 212993;
pub const GLFW_CURSOR_HIDDEN: i32 = 212994;
pub const GLFW_CURSOR_DISABLED: i32 = 212995;

pub const GLFW_ANY_RELEASE_BEHAVIOR: i32 = 0;
pub const GLFW_RELEASE_BEHAVIOR_FLUSH: i32 = 217089;
pub const GLFW_RELEASE_BEHAVIOR_NONE: i32 = 217090;

pub const GLFW_NATIVE_CONTEXT_API: i32 = 221185;
pub const GLFW_EGL_CONTEXT_API: i32 = 221186;
pub const GLFW_OSMESA_CONTEXT_API: i32 = 221187;

// cursor shapes
pub const GLFW_ARROW_CURSOR: i32 = 221185;
pub const GLFW_IBEAM_CURSOR: i32 = 221186;
pub const GLFW_CROSSHAIR_CURSOR: i32 = 221187;
pub const GLFW_HAND_CURSOR: i32 = 221188;
pub const GLFW_HRESIZE_CURSOR: i32 = 221189;
pub const GLFW_VRESIZE_CURSOR: i32 = 221190;

pub const GLFW_CONNECTED: i32 = 262145;
pub const GLFW_DISCONNECTED: i32 = 262146;

pub const GLFW_JOYSTICK_HAT_BUTTONS: i32 = 327681;
pub const GLFW_COCOA_CHDIR_RESOURCES: i32 = 331777;
pub const GLFW_COCOA_MENUBAR: i32 = 331778;
pub const GLFW_DONT_CARE: i32 = -1;


pub type GLFWglproc = ::core::option::Option<unsafe extern "C" fn()>;

pub type GLFWvkproc = ::core::option::Option<unsafe extern "C" fn()>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GLFWmonitor {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GLFWwindow {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GLFWcursor {
    _unused: [u8; 0],
}

pub type GLFWerrorfun = ::core::option::Option<
    unsafe extern "C" fn(error_code: ::core::ffi::c_int, description: *const ::core::ffi::c_char),
>;

pub type GLFWwindowposfun = ::core::option::Option<
    unsafe extern "C" fn(
        window: *mut GLFWwindow,
        xpos: ::core::ffi::c_int,
        ypos: ::core::ffi::c_int,
    ),
>;

pub type GLFWwindowsizefun = ::core::option::Option<
    unsafe extern "C" fn(
        window: *mut GLFWwindow,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ),
>;

pub type GLFWwindowclosefun = ::core::option::Option<unsafe extern "C" fn(window: *mut GLFWwindow)>;

pub type GLFWwindowrefreshfun =
    ::core::option::Option<unsafe extern "C" fn(window: *mut GLFWwindow)>;

pub type GLFWwindowfocusfun = ::core::option::Option<
    unsafe extern "C" fn(window: *mut GLFWwindow, focused: ::core::ffi::c_int),
>;

pub type GLFWwindowiconifyfun = ::core::option::Option<
    unsafe extern "C" fn(window: *mut GLFWwindow, iconified: ::core::ffi::c_int),
>;

pub type GLFWwindowmaximizefun = ::core::option::Option<
    unsafe extern "C" fn(window: *mut GLFWwindow, maximized: ::core::ffi::c_int),
>;

pub type GLFWframebuffersizefun = ::core::option::Option<
    unsafe extern "C" fn(
        window: *mut GLFWwindow,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ),
>;

pub type GLFWwindowcontentscalefun =
    ::core::option::Option<unsafe extern "C" fn(window: *mut GLFWwindow, xscale: f32, yscale: f32)>;

pub type GLFWmousebuttonfun = ::core::option::Option<
    unsafe extern "C" fn(
        window: *mut GLFWwindow,
        button: ::core::ffi::c_int,
        action: ::core::ffi::c_int,
        mods: ::core::ffi::c_int,
    ),
>;

pub type GLFWcursorposfun =
    ::core::option::Option<unsafe extern "C" fn(window: *mut GLFWwindow, xpos: f64, ypos: f64)>;

pub type GLFWcursorenterfun = ::core::option::Option<
    unsafe extern "C" fn(window: *mut GLFWwindow, entered: ::core::ffi::c_int),
>;

pub type GLFWscrollfun = ::core::option::Option<
    unsafe extern "C" fn(window: *mut GLFWwindow, xoffset: f64, yoffset: f64),
>;

pub type GLFWkeyfun = ::core::option::Option<
    unsafe extern "C" fn(
        window: *mut GLFWwindow,
        key: ::core::ffi::c_int,
        scancode: ::core::ffi::c_int,
        action: ::core::ffi::c_int,
        mods: ::core::ffi::c_int,
    ),
>;

pub type GLFWcharfun = ::core::option::Option<
    unsafe extern "C" fn(window: *mut GLFWwindow, codepoint: ::core::ffi::c_uint),
>;

pub type GLFWcharmodsfun = ::core::option::Option<
    unsafe extern "C" fn(
        window: *mut GLFWwindow,
        codepoint: ::core::ffi::c_uint,
        mods: ::core::ffi::c_int,
    ),
>;

pub type GLFWdropfun = ::core::option::Option<
    unsafe extern "C" fn(
        window: *mut GLFWwindow,
        path_count: ::core::ffi::c_int,
        paths: *mut *const ::core::ffi::c_char,
    ),
>;

pub type GLFWmonitorfun = ::core::option::Option<
    unsafe extern "C" fn(monitor: *mut GLFWmonitor, event: ::core::ffi::c_int),
>;

pub type GLFWjoystickfun = ::core::option::Option<
    unsafe extern "C" fn(jid: ::core::ffi::c_int, event: ::core::ffi::c_int),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GLFWvidmode {
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub redBits: ::core::ffi::c_int,
    pub greenBits: ::core::ffi::c_int,
    pub blueBits: ::core::ffi::c_int,
    pub refreshRate: ::core::ffi::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GLFWgammaramp {
    pub red: *mut ::core::ffi::c_ushort,
    pub green: *mut ::core::ffi::c_ushort,
    pub blue: *mut ::core::ffi::c_ushort,
    pub size: ::core::ffi::c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GLFWimage {
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub pixels: *mut ::core::ffi::c_uchar,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GLFWgamepadstate {
    pub buttons: [::core::ffi::c_uchar; 15usize],
    pub axes: [f32; 6usize],
}

extern "C" {
    pub fn glfwInit() -> ::core::ffi::c_int;
    pub fn glfwTerminate();
    pub fn glfwInitHint(hint: ::core::ffi::c_int, value: ::core::ffi::c_int);
    pub fn glfwGetVersion(
        major: *mut ::core::ffi::c_int,
        minor: *mut ::core::ffi::c_int,
        rev: *mut ::core::ffi::c_int,
    );
    pub fn glfwGetVersionString() -> *const ::core::ffi::c_char;
    pub fn glfwGetError(description: *mut *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn glfwSetErrorCallback(callback: GLFWerrorfun) -> GLFWerrorfun;
    pub fn glfwGetMonitors(count: *mut ::core::ffi::c_int) -> *mut *mut GLFWmonitor;
    pub fn glfwGetPrimaryMonitor() -> *mut GLFWmonitor;
    pub fn glfwGetMonitorPos(
        monitor: *mut GLFWmonitor,
        xpos: *mut ::core::ffi::c_int,
        ypos: *mut ::core::ffi::c_int,
    );
    pub fn glfwGetMonitorWorkarea(
        monitor: *mut GLFWmonitor,
        xpos: *mut ::core::ffi::c_int,
        ypos: *mut ::core::ffi::c_int,
        width: *mut ::core::ffi::c_int,
        height: *mut ::core::ffi::c_int,
    );
    pub fn glfwGetMonitorPhysicalSize(
        monitor: *mut GLFWmonitor,
        widthMM: *mut ::core::ffi::c_int,
        heightMM: *mut ::core::ffi::c_int,
    );
    pub fn glfwGetMonitorContentScale(
        monitor: *mut GLFWmonitor,
        xscale: *mut f32,
        yscale: *mut f32,
    );
    pub fn glfwGetMonitorName(monitor: *mut GLFWmonitor) -> *const ::core::ffi::c_char;
    pub fn glfwSetMonitorUserPointer(monitor: *mut GLFWmonitor, pointer: *mut ::core::ffi::c_void);
    pub fn glfwGetMonitorUserPointer(monitor: *mut GLFWmonitor) -> *mut ::core::ffi::c_void;
    pub fn glfwSetMonitorCallback(callback: GLFWmonitorfun) -> GLFWmonitorfun;
    pub fn glfwGetVideoModes(
        monitor: *mut GLFWmonitor,
        count: *mut ::core::ffi::c_int,
    ) -> *const GLFWvidmode;
    pub fn glfwGetVideoMode(monitor: *mut GLFWmonitor) -> *const GLFWvidmode;
    pub fn glfwSetGamma(monitor: *mut GLFWmonitor, gamma: f32);
    pub fn glfwGetGammaRamp(monitor: *mut GLFWmonitor) -> *const GLFWgammaramp;
    pub fn glfwSetGammaRamp(monitor: *mut GLFWmonitor, ramp: *const GLFWgammaramp);
    pub fn glfwDefaultWindowHints();
    pub fn glfwWindowHint(hint: ::core::ffi::c_int, value: ::core::ffi::c_int);
    pub fn glfwWindowHintString(hint: ::core::ffi::c_int, value: *const ::core::ffi::c_char);
    pub fn glfwCreateWindow(
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        title: *const ::core::ffi::c_char,
        monitor: *mut GLFWmonitor,
        share: *mut GLFWwindow,
    ) -> *mut GLFWwindow;
    pub fn glfwDestroyWindow(window: *mut GLFWwindow);
    pub fn glfwWindowShouldClose(window: *mut GLFWwindow) -> ::core::ffi::c_int;
    pub fn glfwSetWindowShouldClose(window: *mut GLFWwindow, value: ::core::ffi::c_int);
    pub fn glfwSetWindowTitle(window: *mut GLFWwindow, title: *const ::core::ffi::c_char);
    pub fn glfwSetWindowIcon(
        window: *mut GLFWwindow,
        count: ::core::ffi::c_int,
        images: *const GLFWimage,
    );
    pub fn glfwGetWindowPos(
        window: *mut GLFWwindow,
        xpos: *mut ::core::ffi::c_int,
        ypos: *mut ::core::ffi::c_int,
    );
    pub fn glfwSetWindowPos(
        window: *mut GLFWwindow,
        xpos: ::core::ffi::c_int,
        ypos: ::core::ffi::c_int,
    );
    pub fn glfwGetWindowSize(
        window: *mut GLFWwindow,
        width: *mut ::core::ffi::c_int,
        height: *mut ::core::ffi::c_int,
    );
    pub fn glfwSetWindowSizeLimits(
        window: *mut GLFWwindow,
        minwidth: ::core::ffi::c_int,
        minheight: ::core::ffi::c_int,
        maxwidth: ::core::ffi::c_int,
        maxheight: ::core::ffi::c_int,
    );
    pub fn glfwSetWindowAspectRatio(
        window: *mut GLFWwindow,
        numer: ::core::ffi::c_int,
        denom: ::core::ffi::c_int,
    );
    pub fn glfwSetWindowSize(
        window: *mut GLFWwindow,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
    pub fn glfwGetFramebufferSize(
        window: *mut GLFWwindow,
        width: *mut ::core::ffi::c_int,
        height: *mut ::core::ffi::c_int,
    );
    pub fn glfwGetWindowFrameSize(
        window: *mut GLFWwindow,
        left: *mut ::core::ffi::c_int,
        top: *mut ::core::ffi::c_int,
        right: *mut ::core::ffi::c_int,
        bottom: *mut ::core::ffi::c_int,
    );
    pub fn glfwGetWindowContentScale(window: *mut GLFWwindow, xscale: *mut f32, yscale: *mut f32);
    pub fn glfwGetWindowOpacity(window: *mut GLFWwindow) -> f32;
    pub fn glfwSetWindowOpacity(window: *mut GLFWwindow, opacity: f32);
    pub fn glfwIconifyWindow(window: *mut GLFWwindow);
    pub fn glfwRestoreWindow(window: *mut GLFWwindow);
    pub fn glfwMaximizeWindow(window: *mut GLFWwindow);
    pub fn glfwShowWindow(window: *mut GLFWwindow);
    pub fn glfwHideWindow(window: *mut GLFWwindow);
    pub fn glfwFocusWindow(window: *mut GLFWwindow);
    pub fn glfwRequestWindowAttention(window: *mut GLFWwindow);
    pub fn glfwGetWindowMonitor(window: *mut GLFWwindow) -> *mut GLFWmonitor;
    pub fn glfwSetWindowMonitor(
        window: *mut GLFWwindow,
        monitor: *mut GLFWmonitor,
        xpos: ::core::ffi::c_int,
        ypos: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        refreshRate: ::core::ffi::c_int,
    );
    pub fn glfwGetWindowAttrib(
        window: *mut GLFWwindow,
        attrib: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn glfwSetWindowAttrib(
        window: *mut GLFWwindow,
        attrib: ::core::ffi::c_int,
        value: ::core::ffi::c_int,
    );
    pub fn glfwSetWindowUserPointer(window: *mut GLFWwindow, pointer: *mut ::core::ffi::c_void);
    pub fn glfwGetWindowUserPointer(window: *mut GLFWwindow) -> *mut ::core::ffi::c_void;
    pub fn glfwSetWindowPosCallback(
        window: *mut GLFWwindow,
        callback: GLFWwindowposfun,
    ) -> GLFWwindowposfun;
    pub fn glfwSetWindowSizeCallback(
        window: *mut GLFWwindow,
        callback: GLFWwindowsizefun,
    ) -> GLFWwindowsizefun;
    pub fn glfwSetWindowCloseCallback(
        window: *mut GLFWwindow,
        callback: GLFWwindowclosefun,
    ) -> GLFWwindowclosefun;
    pub fn glfwSetWindowRefreshCallback(
        window: *mut GLFWwindow,
        callback: GLFWwindowrefreshfun,
    ) -> GLFWwindowrefreshfun;
    pub fn glfwSetWindowFocusCallback(
        window: *mut GLFWwindow,
        callback: GLFWwindowfocusfun,
    ) -> GLFWwindowfocusfun;
    pub fn glfwSetWindowIconifyCallback(
        window: *mut GLFWwindow,
        callback: GLFWwindowiconifyfun,
    ) -> GLFWwindowiconifyfun;
    pub fn glfwSetWindowMaximizeCallback(
        window: *mut GLFWwindow,
        callback: GLFWwindowmaximizefun,
    ) -> GLFWwindowmaximizefun;
    pub fn glfwSetFramebufferSizeCallback(
        window: *mut GLFWwindow,
        callback: GLFWframebuffersizefun,
    ) -> GLFWframebuffersizefun;
    pub fn glfwSetWindowContentScaleCallback(
        window: *mut GLFWwindow,
        callback: GLFWwindowcontentscalefun,
    ) -> GLFWwindowcontentscalefun;
    pub fn glfwPollEvents();
    pub fn glfwWaitEvents();
    pub fn glfwWaitEventsTimeout(timeout: f64);
    pub fn glfwPostEmptyEvent();
    pub fn glfwGetInputMode(
        window: *mut GLFWwindow,
        mode: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn glfwSetInputMode(
        window: *mut GLFWwindow,
        mode: ::core::ffi::c_int,
        value: ::core::ffi::c_int,
    );
    pub fn glfwRawMouseMotionSupported() -> ::core::ffi::c_int;
    pub fn glfwGetKeyName(
        key: ::core::ffi::c_int,
        scancode: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    pub fn glfwGetKeyScancode(key: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn glfwGetKey(window: *mut GLFWwindow, key: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn glfwGetMouseButton(
        window: *mut GLFWwindow,
        button: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn glfwGetCursorPos(window: *mut GLFWwindow, xpos: *mut f64, ypos: *mut f64);
    pub fn glfwSetCursorPos(window: *mut GLFWwindow, xpos: f64, ypos: f64);
    pub fn glfwCreateCursor(
        image: *const GLFWimage,
        xhot: ::core::ffi::c_int,
        yhot: ::core::ffi::c_int,
    ) -> *mut GLFWcursor;
    pub fn glfwCreateStandardCursor(shape: ::core::ffi::c_int) -> *mut GLFWcursor;
    pub fn glfwDestroyCursor(cursor: *mut GLFWcursor);
    pub fn glfwSetCursor(window: *mut GLFWwindow, cursor: *mut GLFWcursor);
    pub fn glfwSetKeyCallback(window: *mut GLFWwindow, callback: GLFWkeyfun) -> GLFWkeyfun;
    pub fn glfwSetCharCallback(window: *mut GLFWwindow, callback: GLFWcharfun) -> GLFWcharfun;
    pub fn glfwSetCharModsCallback(
        window: *mut GLFWwindow,
        callback: GLFWcharmodsfun,
    ) -> GLFWcharmodsfun;
    pub fn glfwSetMouseButtonCallback(
        window: *mut GLFWwindow,
        callback: GLFWmousebuttonfun,
    ) -> GLFWmousebuttonfun;
    pub fn glfwSetCursorPosCallback(
        window: *mut GLFWwindow,
        callback: GLFWcursorposfun,
    ) -> GLFWcursorposfun;
    pub fn glfwSetCursorEnterCallback(
        window: *mut GLFWwindow,
        callback: GLFWcursorenterfun,
    ) -> GLFWcursorenterfun;
    pub fn glfwSetScrollCallback(window: *mut GLFWwindow, callback: GLFWscrollfun)
        -> GLFWscrollfun;
    pub fn glfwSetDropCallback(window: *mut GLFWwindow, callback: GLFWdropfun) -> GLFWdropfun;
    pub fn glfwJoystickPresent(jid: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn glfwGetJoystickAxes(
        jid: ::core::ffi::c_int,
        count: *mut ::core::ffi::c_int,
    ) -> *const f32;
    pub fn glfwGetJoystickButtons(
        jid: ::core::ffi::c_int,
        count: *mut ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    pub fn glfwGetJoystickHats(
        jid: ::core::ffi::c_int,
        count: *mut ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    pub fn glfwGetJoystickName(jid: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    pub fn glfwGetJoystickGUID(jid: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    pub fn glfwSetJoystickUserPointer(jid: ::core::ffi::c_int, pointer: *mut ::core::ffi::c_void);
    pub fn glfwGetJoystickUserPointer(jid: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    pub fn glfwJoystickIsGamepad(jid: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn glfwSetJoystickCallback(callback: GLFWjoystickfun) -> GLFWjoystickfun;
    pub fn glfwUpdateGamepadMappings(string: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn glfwGetGamepadName(jid: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    pub fn glfwGetGamepadState(
        jid: ::core::ffi::c_int,
        state: *mut GLFWgamepadstate,
    ) -> ::core::ffi::c_int;
    pub fn glfwSetClipboardString(window: *mut GLFWwindow, string: *const ::core::ffi::c_char);
    pub fn glfwGetClipboardString(window: *mut GLFWwindow) -> *const ::core::ffi::c_char;
    pub fn glfwGetTime() -> f64;
    pub fn glfwSetTime(time: f64);
    pub fn glfwGetTimerValue() -> u64;
    pub fn glfwGetTimerFrequency() -> u64;
    pub fn glfwMakeContextCurrent(window: *mut GLFWwindow);
    pub fn glfwGetCurrentContext() -> *mut GLFWwindow;
    pub fn glfwSwapBuffers(window: *mut GLFWwindow);
    pub fn glfwSwapInterval(interval: ::core::ffi::c_int);
    pub fn glfwExtensionSupported(extension: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn glfwGetProcAddress(procname: *const ::core::ffi::c_char) -> GLFWglproc;
    pub fn glfwVulkanSupported() -> ::core::ffi::c_int;
    pub fn glfwGetRequiredInstanceExtensions(count: *mut u32) -> *mut *const ::core::ffi::c_char;
}

#[doc(hidden)]
pub fn vendored() -> bool {
    cfg!(glfw_vendored)
}
