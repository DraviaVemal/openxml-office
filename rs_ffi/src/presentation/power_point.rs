use crate::{
    openxml_office_fbs::presentation::{
        power_point_create, power_point_create_return, power_point_create_returnArgs,
    },
    root_from_raw, set_error, write_buffer, StatusCode,
};
use draviavemal_openxml_office::presentation_2007::{PowerPoint, PowerPointPropertiesModel};
use std::ffi::c_char;

#[no_mangle]
/// Creates a new Presentation object.
///
/// Returns a pointer to the newly created Presentation object.
/// If an error occurs, returns a null pointer.
pub extern "C" fn presentation_create(
    in_buffer: *const u8,
    in_buffer_size: usize,
    out_buffer: *mut *mut u8,
    out_buffer_size: *mut usize,
    out_error: *mut *const c_char,
) -> i8 {
    let fbs_presentation_create = match unsafe {
        root_from_raw::<power_point_create>(in_buffer, in_buffer_size, out_error)
    } {
        Ok(root) => root,
        Err(status) => return status,
    };
    let file_name = fbs_presentation_create
        .file_name()
        .map(|item| item.to_string());
    let presentation_settings = fbs_presentation_create.power_point_settings();
    let presentation_properties = PowerPointPropertiesModel {
        is_editable: presentation_settings.is_editable(),
    };
    match PowerPoint::new(file_name, presentation_properties) {
        Ok(presentation) => {
            let presentation_ptr = Box::into_raw(Box::new(presentation)) as u64;
            let mut builder = flatbuffers::FlatBufferBuilder::new();
            let presentation_create_return = power_point_create_return::create(
                &mut builder,
                &power_point_create_returnArgs {
                    power_point_ptr: presentation_ptr,
                },
            );
            builder.finish(presentation_create_return, None);
            unsafe { write_buffer(builder.finished_data(), out_buffer, out_buffer_size) };
            StatusCode::Success as i8
        }
        Err(e) => unsafe { set_error(out_error, &e, StatusCode::UnknownError) },
    }
}
