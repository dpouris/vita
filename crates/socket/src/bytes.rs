use crate::error::BytesError;

pub(crate) trait ToBytes<S>
where
    S: Clone + Copy,
{
    fn to_bytes<'a>(&self) -> &'a [u8] {
        unsafe {
            std::slice::from_raw_parts((self as *const Self) as *const u8, size_of::<S>())
        }
    }

    // This function is very unsafe. We have to take into account many things:
    //  - The endianess of the bytes
    //  - The alignment of the underlying struct
    // Although, the structs this trait is implemented on are represented as C structs meaning that the alignment, order and size will be guaranteed
    fn from_bytes(bytes: &[u8]) -> Result<S, BytesError> {
        if bytes.len() != size_of::<S>() {
            return Err(BytesError::InvalidLength);
        }

        // unsafe { *((bytes as *const [u8]) as *mut S) }
        let bytes_ptr = bytes.as_ptr();
        let self_ptr = bytes_ptr as *mut S;

        Ok(unsafe { *self_ptr })
    }
}
