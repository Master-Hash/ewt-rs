#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CString;
use std::os::raw;
use std::ptr;
use std::sync::LazyLock;
use windows::core::h;
use windows::core::HSTRING;
use windows::Data::Text::SelectableWordsSegmenter;

static segmenter: LazyLock<SelectableWordsSegmenter> =
    LazyLock::new(|| SelectableWordsSegmenter::CreateWithLanguage(h!("zh-CN")).unwrap());

#[no_mangle]
#[allow(non_upper_case_globals)]
pub static plugin_is_GPL_compatible: libc::c_int = 1;

// ordinarily the Rust compiler will mangle funciton names. we don't want to do
// that, since the C code won't know what code to call.
// we'll also want to use `unsafe` because we need access to raw pointers
#[no_mangle]
pub unsafe extern "C" fn emacs_module_init(runtime: *mut emacs_runtime) -> libc::c_int {
    let fset_string = CString::new("fset").unwrap();

    let env = (*runtime).get_environment.unwrap()(runtime);

    let intern = (*env).intern.unwrap();
    let funcall = (*env).funcall.unwrap();
    let make_function = (*env).make_function.unwrap();

    let Qfset = intern(env, fset_string.as_ptr());

    let func_emt__do_split_helper_description = CString::new("This function takes a string and return an array of bounds. A bound is a cons with the starting position and the ending position of a word.").unwrap();
    let func_emt__word_at_point_or_forward_description = CString::new("This functions takes a string and a position, and returns the bound of the word at the position. If the position is at bound of two words, it returns the word at the right side of that position. This function does not tokenize the whole string, so it is faster in some cases.").unwrap();
    let func_emt__do_split_helper_name = CString::new("emt--do-split-helper").unwrap();
    let func_emt__word_at_point_or_forward_name =
        CString::new("emt--word-at-point-or-forward-helper").unwrap();
    let Qsplit_helper = intern(env, func_emt__do_split_helper_name.as_ptr());
    let Qword_at_point = intern(env, func_emt__word_at_point_or_forward_name.as_ptr());
    let func_emt__do_split_helper = make_function(
        env,
        1,
        1,
        Some(Femt__do_split_helper),
        func_emt__do_split_helper_description.as_ptr(),
        std::ptr::null_mut(),
    );
    funcall(
        env,
        Qfset,
        2,
        [Qsplit_helper, func_emt__do_split_helper].as_mut_ptr(),
    );
    let func_emt__do_split_helper = make_function(
        env,
        2,
        2,
        Some(Femt__word_at_point_or_forward),
        func_emt__word_at_point_or_forward_description.as_ptr(),
        std::ptr::null_mut(),
    );
    funcall(
        env,
        Qfset,
        2,
        [Qword_at_point, func_emt__do_split_helper].as_mut_ptr(),
    );
    0
}

unsafe extern "C" fn Femt__do_split_helper(
    env: *mut emacs_env,
    nargs: isize,
    args: *mut emacs_value,
    data: *mut raw::c_void,
) -> emacs_value {
    let cons_string = CString::new("cons").unwrap();
    let vector_string = CString::new("vector").unwrap();
    let intern = (*env).intern.unwrap();
    let funcall = (*env).funcall.unwrap();
    let make_integer = (*env).make_integer.unwrap();
    let copy_string_contents = (*env).copy_string_contents.unwrap();

    let Qcons = intern(env, cons_string.as_ptr());
    let Qvector = intern(env, vector_string.as_ptr());

    let mut len: isize = 0;
    let is_ok = copy_string_contents(env, *args, ptr::null_mut(), &mut len);
    let mut buf = vec![0u8; len as usize];
    let is_ok = copy_string_contents(env, *args, buf.as_mut_ptr() as *mut raw::c_char, &mut len);

    strip_trailing_zero_bytes(&mut buf);

    let param_u8 = String::from_utf8(buf).unwrap();
    let param_hstring = HSTRING::from(param_u8);
    let res = segmenter.GetTokens(&param_hstring).unwrap();

    let iConsCell = res.into_iter().map(|i| {
        let segment = i.SourceTextSegment().unwrap();
        let l = make_integer(env, segment.StartPosition.into());
        let r = make_integer(env, (segment.StartPosition + segment.Length).into());
        funcall(env, Qcons, 2, [l, r].as_mut_ptr())
    });
    let mut consCell = iConsCell.collect::<Vec<_>>();
    let l = consCell.len();
    let ddd = consCell.as_mut_ptr();
    funcall(env, Qvector, l as isize, ddd)
}

unsafe extern "C" fn Femt__word_at_point_or_forward(
    env: *mut emacs_env,
    nargs: isize,
    args: *mut emacs_value,
    data: *mut raw::c_void,
) -> emacs_value {
    let cons_string = CString::new("cons").unwrap();
    let intern = (*env).intern.unwrap();
    let funcall = (*env).funcall.unwrap();
    let make_integer = (*env).make_integer.unwrap();
    let extract_integer = (*env).extract_integer.unwrap();
    let copy_string_contents = (*env).copy_string_contents.unwrap();

    let Qcons = intern(env, cons_string.as_ptr());

    let mut len: isize = 0;
    let is_ok = copy_string_contents(env, *args, ptr::null_mut(), &mut len);
    let mut buf = vec![0u8; len as usize];
    let is_ok = copy_string_contents(env, *args, buf.as_mut_ptr() as *mut raw::c_char, &mut len);

    strip_trailing_zero_bytes(&mut buf);

    let n = extract_integer(env, *args.offset(1));

    let param_u8 = String::from_utf8(buf).unwrap();
    let param_hstring = HSTRING::from(param_u8);
    let res = segmenter
        .GetTokenAt(&param_hstring, n.try_into().unwrap())
        .unwrap();

    let segment = res.SourceTextSegment().unwrap();
    let l = make_integer(env, segment.StartPosition.into());
    let r = make_integer(env, (segment.StartPosition + segment.Length).into());
    funcall(env, Qcons, 2, [l, r].as_mut_ptr())
}

// Thank you,
// https://github.com/ubolonton/emacs-module-rs/blob/126241a79d171e8de43c7db248b277fac7f1a4e8/src/types/string.rs#L103C1-L109C2
fn strip_trailing_zero_bytes(bytes: &mut Vec<u8>) {
    let mut len = bytes.len();
    while len > 0 && bytes[len - 1] == 0 {
        bytes.pop(); // strip trailing 0-byte(s)
        len -= 1;
    }
}
