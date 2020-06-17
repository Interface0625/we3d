use nalgebra as na;
use na::{Vector2, Vector3, Vector4, Matrix4};
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use super::common_funcs as cf;
use super::shaders;


pub fn uniform_v1(gl: &GL, location_ptr: &WebGlUniformLocation, value: na::Vector1<f32>) { gl.uniform1f( Some(location_ptr), value.x ) }
pub fn uniform_v2(gl: &GL, location_ptr: &WebGlUniformLocation, value: na::Vector2<f32>) { gl.uniform2f( Some(location_ptr), value.x, value.y ) }
pub fn uniform_v3(gl: &GL, location_ptr: &WebGlUniformLocation, value: na::Vector3<f32>) { gl.uniform3f( Some(location_ptr), value.x, value.y, value.z ) }
pub fn uniform_v4(gl: &GL, location_ptr: &WebGlUniformLocation, value: na::Vector4<f32>) { gl.uniform4f( Some(location_ptr), value.x, value.y, value.z, value.w ) }

pub fn uniform_matrix4(gl: &GL, location_ptr: &WebGlUniformLocation, value: na::Matrix4<f32>) { gl.uniform_matrix4fv_with_f32_array(Some(location_ptr), false, &value.as_slice())}