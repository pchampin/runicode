
#[repr(C)]
/// A type representing a sage Unicode string
pub struct Ustr {
    // pointer to the characters
    chars: *const u8,
    // length in bytes
    lenb: usize,
    // whether this Ustr owns its data
    own: bool,
}

#[no_mangle]
/// Create a new [`Ustr`] from the given data.
///
/// IMPORTANT:
/// * if the data pointed to by `chars` is not valid UTF-8,
///   the returned [`Ustr`] will have a length of 0,
///   regardless of `lenb`.
/// * the data pointed to by `chars` is expected to live
///   as long as the returned [`Ustr`].
pub extern "C" fn ustr_new(chars: *const u8, lenb: usize) -> Ustr {
    Ustr::new(chars, lenb)
}

#[no_mangle]
/// Frees an existing [`Ustr`].
///
/// IMPORTANT:
/// * if the [`Ustr`] was created with [`ustr_new`],
///   the passed pointer `chars` must still be freed by caller;
/// * if the [`Ustr`] was created with another method
///   (e.g. [`ustr_cat`]), then it will be entirely freed.
pub extern "C" fn ustr_free(ustr: Ustr) {
    if ustr.own {
        std::mem::drop(ustr.to_string())
    }
}

#[no_mangle]
/// Convert to a C string
///
/// IMPORTANT:
/// freeing the given data becomes the responsibility of the caller.
pub extern "C" fn ustr_to_c_string(ustr: Ustr) -> *mut u8 {
    let txt = ustr.as_str();
    let mut v = Vec::with_capacity(txt.len()+1);
    v.extend_from_slice(txt.as_bytes());
    v.push(0);
    let ret = &mut v[0] as *mut u8;
    std::mem::forget(v);
    ret
}

/// Return the length in bytes of this string
#[no_mangle]
pub extern "C" fn ustr_lenb(ustr: &Ustr) -> usize {
    ustr.len()
}

/// Return the length in characters of this string
#[no_mangle]
pub extern "C" fn ustr_lenc(ustr: &Ustr) -> usize {
    ustr.as_str().chars().count()
}

/// Concatenate two Ustr into a new one.
#[no_mangle]
pub extern "C" fn ustr_cat(ustr1: &Ustr, ustr2: &Ustr) -> Ustr {
    let mut cat = String::with_capacity(ustr1.len() + ustr2.len());
    cat.push_str(ustr1.as_str());
    cat.push_str(ustr2.as_str());
    Ustr::from_string(cat)
}

impl Ustr {
    fn new(chars: *const u8, lenb: usize) -> Self {
        let s = unsafe { std::slice::from_raw_parts(chars, lenb) };
        let own = false;
        if let Ok(_) = std::str::from_utf8(s) {
            Ustr { chars, lenb, own }
        } else {
            Ustr { chars: &EMPTY as *const u8, lenb: 0, own }
        }
    }

    fn from_string(s: String) -> Self {
        let chars = &s.as_bytes()[0] as *const u8;
        let lenb = s.len();
        let own = true;
        std::mem::forget(s);
        Ustr { chars, lenb, own }
    }

    fn as_str(&self) -> &str {
        unsafe {
            let s = std::slice::from_raw_parts(self.chars, self.lenb);
            std::str::from_utf8_unchecked(s)
        }
    }

    fn to_string(self) -> String {
        unsafe {
            String::from_raw_parts(self.chars as *mut u8, self.lenb, self.lenb)
        }
    }

    fn len(&self) -> usize {
        self.lenb
    }

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        self.lenb == 0
    }
}

#[no_mangle]
pub static EMPTY: u8 = 0;


