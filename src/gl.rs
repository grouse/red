#![allow(dead_code)]

use std::os::raw::{c_void, c_char};

pub const TRUE  : i32 = 1;
pub const FALSE : i32 = 0;

pub const MAJOR_VERSION : u32 = 0x821B;
pub const MINOR_VERSION : u32 = 0x821C;

pub type GLenum     = u32;
pub type GLboolean  = i32;
pub type GLsizei    = i32;
pub type GLbitfield = u32;

pub type GLchar  = c_char;
pub type GLbyte  = i8;
pub type GLshort = i16;
pub type GLint   = i32;
pub type GLint64 = i64;

pub type GLubyte  = u8;
pub type GLushort = u16;
pub type GLuint   = u32;
pub type GLuint64 = u64;

pub type GLfloat  = f32;
pub type GLclmapf = f32;
pub type GLdouble = f64;
pub type GLclampd = f64;

pub const COLOR_BUFFER_BIT : GLbitfield = 0x00004000;

pub const NO_ERROR : GLenum = 0;
pub const TRIANGLES : GLenum = 0x0004;

// caps enums
pub const DEBUG_OUTPUT : GLenum = 0x92E;

type DEBUGPROC = unsafe extern fn (
    source    : GLenum,
    dbg_type  : GLenum,
    id        : GLuint,
    severity  : GLenum,
    length    : GLsizei,
    message   : *const GLchar,
    userParam : *const c_void);

#[link(name = "opengl32")]
extern {
    #[link_name = "glGetBooleanv"]
    pub fn GetBooleanv(pname : GLenum, data : *mut GLboolean);

    #[link_name = "glGetDoublev"]
    pub fn GetDoublev(pname : GLenum, data : *mut GLdouble);

    #[link_name = "glGetFloatv"]
    pub fn GetFloatv(pname : GLenum, data : *mut GLfloat);

    #[link_name = "glGetIntegerv"]
    pub fn GetIntegerv(pname : GLenum, data : *mut GLint);

    #[link_name = "glGetInteger64v"]
    pub fn GetInteger64v(pname : GLenum, data : *mut GLint64);

    #[link_name = "glGetBooleani_v"]
    pub fn GetBooleani_v(target : GLenum, index : GLuint, data : *mut GLboolean);

    #[link_name = "glGetIntegeri_v"]
    pub fn GetIntegeri_v(target : GLenum, index : GLuint, data : *mut GLint);

    #[link_name = "glGetFloati_v"]
    pub fn GetFloati_v(target : GLenum, index : GLuint, data : *mut GLfloat);

    #[link_name = "glGetDoublei_v"]
    pub fn GetDoublei_v(target : GLenum, index : GLuint, data : *mut GLdouble);

    #[link_name = "glGetInteger64i_v"]
    pub fn GetInteger64i_v(target : GLenum, index : GLuint, data : *mut GLint64);

    #[link_name = "glClear"]
    pub fn Clear(mask : GLbitfield );

    #[link_name = "glClearColor"]
    pub fn ClearColor(red : GLfloat, green : GLfloat, blue : GLfloat, alpha : GLfloat);

    #[link_name = "glBegin"]
    pub fn Begin(mode : GLenum);

    #[link_name = "glEnd"]
    pub fn End();

    #[link_name = "glVertex3f"]
    pub fn Vertex3f(x : GLfloat, y : GLfloat, z : GLfloat);

    #[link_name = "glColor3f"]
    pub fn Color3f(red : GLfloat, green : GLfloat, blue : GLfloat);

    #[link_name = "glLoadIdentity"]
    pub fn LoadIdentity();

    #[link_name = "glGetError"]
    pub fn GetError() -> GLenum;


    #[link_name = "glEnable"]
    pub fn Enable(cap : GLenum);

    #[link_name = "glDisable"]
    pub fn Disable(cap : GLenum);
}

pub type DebugMessageCallback_t = unsafe extern fn (
    callback : DEBUGPROC,
    userParam : *mut c_void);

#[allow(unused_variables)]
unsafe extern fn DebugMessageCallback_stub(
    callback : DEBUGPROC,
    userParam : *mut c_void)
{
    unimplemented!();
}

pub static mut DebugMessageCallback : DebugMessageCallback_t = DebugMessageCallback_stub;


