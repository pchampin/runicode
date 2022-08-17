
#[repr(C)]
/// A type representing a sage Unicode string
pub struct Ustr {
    // pointer to the characters (must contain a terminal 0)
    chars: *mut u8,
    // length in bytes (not counting the terminal 0)
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
pub extern "C" fn ustr_new(chars: *mut u8, lenb: usize) -> Ustr {
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
/// Return the char* of this string (useful for printing in C)
pub extern "C" fn ustr_chars(ustr: &Ustr) -> *mut u8 {
    ustr.chars
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
    ustr1.cat(ustr2)
}

impl Ustr {
    fn new(chars: *mut u8, lenb: usize) -> Self {
        let s = unsafe { std::slice::from_raw_parts(chars, lenb) };
        let own = false;
        if let Ok(_) = std::str::from_utf8(s) {
            Ustr { chars, lenb, own }
        } else {
            let chars = &EMPTY as *const u8;
            let chars: *mut u8 = unsafe { std::mem::transmute(chars) };
            Ustr { chars, lenb: 0, own }
        }
    }

    #[allow(dead_code)]
    fn from_string(mut s: String) -> Self {
        let v = unsafe { s.as_mut_vec() };
        let chars = &mut v[0] as *mut u8;
        let lenb = s.len();
        let own = true;
        std::mem::forget(s);
        Ustr { chars, lenb, own }
    }

    fn cat(&self, other: &Ustr) -> Self {
        let mut v = Vec::with_capacity(self.lenb + other.lenb + 1);
        v.extend_from_slice(self.as_str().as_bytes());
        v.extend_from_slice(other.as_str().as_bytes());
        v.push(0);
        let ret = Ustr {
            chars: &mut v[0] as *mut u8,
            lenb: v.len() - 1,
            own: true,
        };
        std::mem::forget(v);
        ret
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


