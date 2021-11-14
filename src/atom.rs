#![allow(non_camel_case_types)]

macro_rules! iterable_key_enum {

    ( $name:ident => $( $val:ident ),* ) => {
        use std::slice::Iter;

        #[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
        pub enum $name {
            $( $val ),*
        }

        impl $name {
            pub fn variants() -> Iter<'static, $name> {
                static VARIANTS: &'static [$name] =
                        &[$($name::$val),*];
                VARIANTS.iter()
            }

            pub fn num_variants() -> usize {
                [$($name::$val),*].len()
            }
        }
    };

}

iterable_key_enum! {
    Atom =>
        UTF8_STRING,

        WM_PROTOCOLS,
        WM_DELETE_WINDOW,
        WM_TRANSIENT_FOR,
        WM_CHANGE_STATE,
        WM_STATE,
        _NET_WM_STATE,
        _NET_WM_STATE_MODAL,
        _NET_WM_STATE_STICKY,
        _NET_WM_STATE_MAXIMIZED_VERT,
        _NET_WM_STATE_MAXIMIZED_HORZ,
        _NET_WM_STATE_SHADED,
        _NET_WM_STATE_SKIP_TASKBAR,
        _NET_WM_STATE_SKIP_PAGER,
        _NET_WM_STATE_HIDDEN,
        _NET_WM_STATE_FULLSCREEN,
        _NET_WM_STATE_ABOVE,
        _NET_WM_STATE_BELOW,
        _NET_WM_STATE_DEMANDS_ATTENTION,
        _NET_WM_STATE_FOCUSED,
        _NET_WM_NAME
}
