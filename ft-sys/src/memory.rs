/// Allocate memory into the wasm linear memory
/// and return the offset to the start of the block.
/// # Safety
///
/// This function is unsafe because it dereferences the pointer. There is no way to
/// make this function unsafe.
#[unsafe(no_mangle)]
#[allow(clippy::uninit_vec)]
pub fn alloc(len: i32) -> i32 {
    // create a new mutable buffer with capacity len
    // we allocate 4 more bytes than asked and store len on the first 4 bytes
    let mut buf: Vec<u8> = Vec::with_capacity(len as usize + 4);

    unsafe {
        buf.set_len(len as usize + 4);
    }

    let b = len.to_ne_bytes();
    buf[0] = b[0];
    buf[1] = b[1];
    buf[2] = b[2];
    buf[3] = b[3];

    // take a mutable pointer to the buffer
    let ptr = buf.as_mut_ptr();
    // // take ownership of the memory block and
    // // ensure that its destructor is not
    // // called when the object goes out of scope
    // // at the end of the function
    std::mem::forget(buf);
    ptr as i32
}

/// de-allocating the memory
/// # Safety
///
/// This function is unsafe because it dereferences the pointer. There is no way to
/// make this function unsafe.
#[unsafe(no_mangle)]
pub unsafe fn dealloc(ptr: i32) {
    let size = unsafe { ptr_len(ptr) };

    let data = unsafe { Vec::from_raw_parts(ptr as *mut u8, size as usize + 4, size as usize + 4) };
    drop(data);
}

/// de-allocating the memory
/// # Safety
///
/// This function is unsafe because it dereferences the pointer. There is no way to
/// make this function unsafe.
#[unsafe(no_mangle)]
pub unsafe fn dealloc_with_len(ptr: i32, len: i32) {
    let data = unsafe { Vec::from_raw_parts(ptr as *mut u8, len as usize, len as usize) };
    drop(data);
}

/// # Safety
///
/// This function is unsafe because it dereferences the pointer. There is no way to
/// make this function unsafe.
pub unsafe fn ptr_len(ptr: i32) -> i32 {
    let len_bytes = unsafe { Vec::from_raw_parts(ptr as *mut u8, 4, 4) };
    let len = i32::from_ne_bytes([len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3]]);
    std::mem::forget(len_bytes);
    len
}

/// # Safety
///
/// This function is unsafe because it dereferences the pointer. There is no way to
/// make this function unsafe.
pub(crate) fn string_from_ptr(ptr: i32) -> String {
    unsafe { String::from_utf8_unchecked(bytes_from_ptr(ptr).into()) }
}

/// # Safety
///
/// This function is unsafe because it dereferences the pointer. There is no way to
/// make this function unsafe.
fn bytes_from_ptr(ptr: i32) -> bytes::Bytes {
    unsafe {
        let len = ptr_len(ptr);
        let v = Vec::from_raw_parts(ptr as *mut u8, len as usize + 4, len as usize + 4);
        let bytes = bytes::Bytes::from(v);
        bytes.slice(4..)
    }
}

pub fn bytes_to_ptr(mut d: Vec<u8>) -> (i32, i32) {
    let len = d.len() as i32;
    let data = d.as_mut_ptr() as i32;

    std::mem::forget(d);

    (data, len)
}

pub fn string_to_bytes_ptr(s: String) -> (i32, i32) {
    bytes_to_ptr(s.into_bytes())
}

pub fn json_ptr(d: impl serde::Serialize) -> (i32, i32) {
    let bytes = serde_json::to_vec(&d)
        .inspect_err(|e| ft_sys::println!("failed to serialise: {e:?}"))
        .unwrap();
    bytes_to_ptr(bytes)
}

pub fn json_from_ptr<T: serde::de::DeserializeOwned>(ptr: i32) -> T {
    let bytes = bytes_from_ptr(ptr);
    match serde_json::from_slice(&bytes) {
        Ok(v) => v,
        Err(e) => {
            ft_sys::println!(
                "got error when deserializing: {e:?}, json: {}",
                String::from_utf8_lossy(&bytes)
            );
            panic!()
        }
    }
}
