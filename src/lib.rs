#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(all(not(feature = "windows"), feature = "icu_segmenter"))]
use icu_segmenter::WordSegmenter;
#[cfg(all(not(feature = "windows"), feature = "icu_segmenter"))]
use itertools::Itertools;
use std::ffi::CStr;
use std::os::raw;
use std::ptr;
#[cfg(feature = "windows")]
use std::sync::LazyLock;
#[cfg(feature = "windows")]
use windows::Data::Text::SelectableWordsSegmenter;
#[cfg(feature = "windows")]
use windows::core::HSTRING;
#[cfg(feature = "windows")]
use windows::core::h;

#[cfg(feature = "windows")]
static segmenter: LazyLock<SelectableWordsSegmenter> =
    LazyLock::new(|| SelectableWordsSegmenter::CreateWithLanguage(h!("zh-CN")).unwrap());

// icu segmenter seems unable to be initialized in a static variable
// Rc is used inside, so it is not Sync

#[unsafe(no_mangle)]
#[allow(non_upper_case_globals)]
pub static plugin_is_GPL_compatible: libc::c_int = 1;

// ordinarily the Rust compiler will mangle funciton names. we don't want to do
// that, since the C code won't know what code to call.
// we'll also want to use `unsafe` because we need access to raw pointers
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_module_init(runtime: *mut emacs_runtime) -> libc::c_int {
    unsafe {
        let env = (*runtime).get_environment.unwrap_unchecked()(runtime);

        let intern = (*env).intern.unwrap_unchecked();
        let funcall = (*env).funcall.unwrap_unchecked();
        let make_function = (*env).make_function.unwrap_unchecked();

        let Qfset = intern(env, c"fset".as_ptr());

        let func_emt__do_split_helper_description = c"This function takes a string and return an array of bounds. A bound is a cons with the starting position and the ending position of a word.";
        let func_emt__word_at_point_or_forward_description = c"This functions takes a string and a position, and returns the bound of the word at the position. If the position is at bound of two words, it returns the word at the right side of that position. This function does not tokenize the whole string, so it is faster in some cases.";
        let func_emt__do_split_helper_name = c"emt--do-split-helper";
        let func_emt__word_at_point_or_forward_name = c"emt--word-at-point-or-forward-helper";
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
}

unsafe extern "C" fn Femt__do_split_helper(
    env: *mut emacs_env,
    nargs: isize,
    args: *mut emacs_value,
    data: *mut raw::c_void,
) -> emacs_value {
    unsafe {
        let intern = (*env).intern.unwrap_unchecked();
        let funcall = (*env).funcall.unwrap_unchecked();
        let make_integer = (*env).make_integer.unwrap_unchecked();
        let copy_string_contents = (*env).copy_string_contents.unwrap_unchecked();

        let Qcons = intern(env, c"cons".as_ptr());
        let Qvector = intern(env, c"vector".as_ptr());

        let mut len: isize = 0;
        let is_ok = copy_string_contents(env, *args, ptr::null_mut(), &mut len);
        let mut buf = vec![0u8; len as usize];
        let is_ok =
            copy_string_contents(env, *args, buf.as_mut_ptr() as *mut raw::c_char, &mut len);

        let param_u8 =
            std::str::from_utf8_unchecked(CStr::from_bytes_with_nul_unchecked(&buf).to_bytes());
        // let param_u8 = CStr::from_bytes_with_nul(&buf).unwrap().to_str().unwrap();

        #[cfg(feature = "windows")]
        let mut consCell = {
            let param_hstring = HSTRING::from(param_u8);
            let res = segmenter.GetTokens(&param_hstring).unwrap();

            let iConsCell = res.into_iter().map(|i| {
                let segment = i.SourceTextSegment().unwrap();
                let l = make_integer(env, segment.StartPosition.into());
                let r = make_integer(env, (segment.StartPosition + segment.Length).into());
                funcall(env, Qcons, 2, [l, r].as_mut_ptr())
            });
            iConsCell.collect::<Vec<_>>()
        };
        #[cfg(all(not(feature = "windows"), feature = "icu_segmenter"))]
        let mut consCell = {
            let segmenter_icu = WordSegmenter::new_auto();
            let segments = segmenter_icu
                .segment_str(param_u8)
                .tuple_windows()
                .map(|(i, j)| &param_u8[i..j]);
            let ss = segments.map(|s| s.chars().count());
            // we need prefix sum
            // from: [4, 1, 4]
            // to [(0, 4), (4, 5), (5, 9)]
            let iConsCell = ss
                .scan(0, |acc, x| {
                    let res = Some((*acc, *acc + x));
                    *acc += x;
                    res
                })
                .map(|(l, r)| {
                    let l = make_integer(env, l as i64);
                    let r = make_integer(env, r as i64);
                    funcall(env, Qcons, 2, [l, r].as_mut_ptr())
                });
            iConsCell.collect::<Vec<_>>()
        };
        let l = consCell.len();
        let ddd = consCell.as_mut_ptr();
        funcall(env, Qvector, l as isize, ddd)
    }
}

unsafe extern "C" fn Femt__word_at_point_or_forward(
    env: *mut emacs_env,
    nargs: isize,
    args: *mut emacs_value,
    data: *mut raw::c_void,
) -> emacs_value {
    unsafe {
        let intern = (*env).intern.unwrap_unchecked();
        let funcall = (*env).funcall.unwrap_unchecked();
        let make_integer = (*env).make_integer.unwrap_unchecked();
        let extract_integer = (*env).extract_integer.unwrap_unchecked();
        let copy_string_contents = (*env).copy_string_contents.unwrap_unchecked();

        let Qcons = intern(env, c"cons".as_ptr());

        let mut len: isize = 0;
        let is_ok = copy_string_contents(env, *args, ptr::null_mut(), &mut len);
        let mut buf = vec![0u8; len as usize];
        let is_ok =
            copy_string_contents(env, *args, buf.as_mut_ptr() as *mut raw::c_char, &mut len);

        let param_u8 =
            std::str::from_utf8_unchecked(CStr::from_bytes_with_nul_unchecked(&buf).to_bytes());
        // let param_u8 = CStr::from_bytes_with_nul(&buf).unwrap().to_str().unwrap();

        let n = extract_integer(env, *args.offset(1));

        #[cfg(feature = "windows")]
        let (l, r) = {
            let param_hstring = HSTRING::from(param_u8);
            let res = segmenter
                .GetTokenAt(&param_hstring, n.try_into().unwrap())
                .unwrap();

            let segment = res.SourceTextSegment().unwrap();
            let l = make_integer(env, segment.StartPosition.into());
            let r = make_integer(env, (segment.StartPosition + segment.Length).into());
            (l, r)
        };
        #[cfg(all(not(feature = "windows"), feature = "icu_segmenter"))]
        let (l, r) = {
            // Sadly WordSegmenter does not provide a way to get the nth token
            let segmenter_icu = WordSegmenter::new_auto();
            let segments = segmenter_icu
                .segment_str(param_u8)
                .tuple_windows()
                .map(|(i, j)| &param_u8[i..j]);
            let mut ss = segments.map(|s| s.chars().count());
            // we need prefix sum
            // from: [4, 1, 4], 4
            // to [(4, 5)]
            // from: [4, 1, 4], 6
            // to [(5, 9)]
            let iConsCell = ss.try_fold(0, |acc, x| {
                let r = acc + x;
                let l = acc;
                if n < r.try_into().unwrap() {
                    Err((l, r))
                } else {
                    Ok(r)
                }
            });
            match iConsCell {
                // Seems all the program will be panic if we reach here
                Ok(_) => unreachable!(),
                Err((l, r)) => {
                    let l = make_integer(env, l as i64);
                    let r = make_integer(env, r as i64);
                    (l, r)
                }
            }
        };
        funcall(env, Qcons, 2, [l, r].as_mut_ptr())
    }
}
