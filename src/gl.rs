#![allow(dead_code)]

use std::os::raw::c_void;

pub const TRUE  : i32 = 1;
pub const FALSE : i32 = 0;

pub const MAJOR_VERSION : u32 = 0x821B;
pub const MINOR_VERSION : u32 = 0x821C;

pub type GLenum     = u32;
pub type GLboolean  = i32;
pub type GLsizei    = i32;
pub type GLbitfield = u32;

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

#[link(name = "opengl32")]
extern {
    #[link_name = "glGetBooleanv"]
    pub fn GetBooleanv(pname : GLenum, data : *mut GLboolean) -> c_void;

    #[link_name = "glGetDoublev"]
    pub fn GetDoublev(pname : GLenum, data : *mut GLdouble) -> c_void;

    #[link_name = "glGetFloatv"]
    pub fn GetFloatv(pname : GLenum, data : *mut GLfloat) -> c_void;

    #[link_name = "glGetIntegerv"]
    pub fn GetIntegerv(pname : GLenum, data : *mut GLint) -> c_void;

    #[link_name = "glGetInteger64v"]
    pub fn GetInteger64v(pname : GLenum, data : *mut GLint64) -> c_void;

    #[link_name = "glGetBooleani_v"]
    pub fn GetBooleani_v(target : GLenum, index : GLuint, data : *mut GLboolean) -> c_void;

    #[link_name = "glGetIntegeri_v"]
    pub fn GetIntegeri_v(target : GLenum, index : GLuint, data : *mut GLint) -> c_void;

    #[link_name = "glGetFloati_v"]
    pub fn GetFloati_v(target : GLenum, index : GLuint, data : *mut GLfloat) -> c_void;

    #[link_name = "glGetDoublei_v"]
    pub fn GetDoublei_v(target : GLenum, index : GLuint, data : *mut GLdouble) -> c_void;

    #[link_name = "glGetInteger64i_v"]
    pub fn GetInteger64i_v(target : GLenum, index : GLuint, data : *mut GLint64) -> c_void;
}
