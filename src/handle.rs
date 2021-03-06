//! Contains definitions for wlc handle types.

// We get warnings for the bitflags, which are described in the crate as C-safe...
#![allow(improper_ctypes)]

extern crate libc;
use libc::{uintptr_t, c_char, c_void};

use super::pointer_to_string;
use super::types::{Geometry, ResizeEdge, Point, Size, ViewType, ViewState};

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a handle to a wlc view.
pub struct WlcView(libc::uintptr_t);

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a handle to a wlc output.
pub struct WlcOutput(libc::uintptr_t);

#[link(name = "wlc")]
extern "C" {
    fn wlc_get_outputs(memb: *mut libc::size_t) -> *const libc::uintptr_t;

    fn wlc_get_focused_output() -> uintptr_t;

    fn wlc_output_get_name(output: uintptr_t) -> *const c_char;

    fn wlc_handle_get_user_data(handle: uintptr_t) -> *mut c_void;

    // Defined in wlc-render.h
    fn wlc_output_schedule_render(output: uintptr_t);

    fn wlc_handle_set_user_data(handle: uintptr_t, userdata: *const c_void);

    fn wlc_output_get_sleep(output: uintptr_t) -> bool;

    fn wlc_output_set_sleep(output: uintptr_t, sleep: bool);

    fn wlc_output_get_resolution(output: uintptr_t) -> *const Size;

    fn wlc_output_set_resolution(output: uintptr_t, resolution: *const Size);

    fn wlc_output_get_mask(output: uintptr_t) -> u32;

    fn wlc_output_set_mask(output: uintptr_t, mask: u32);

    // TODO tricky definition here
    //fn wlc_output_get_pixels(output: WlcHandle) -> ();

    fn wlc_output_get_views(output: uintptr_t,
                            out_memb: *mut libc::size_t) -> *const uintptr_t;

    fn wlc_output_set_views(output: uintptr_t, views: *const uintptr_t, memb: libc::size_t) -> bool;

    fn wlc_output_focus(output: uintptr_t);

    // View API

    fn wlc_view_focus(view: uintptr_t);

    fn wlc_view_close(view: uintptr_t);

    // View -> Output
    fn wlc_view_get_output(view: uintptr_t) -> uintptr_t;

    // "set output. Alternatively you can use wlc_output_set_views"
    fn wlc_view_set_output(view: uintptr_t, output: uintptr_t);

    fn wlc_view_send_to_back(view: uintptr_t);

    fn wlc_view_send_below(view: uintptr_t, other: uintptr_t);

    fn wlc_view_bring_above(view: uintptr_t, other: uintptr_t);

    fn wlc_view_bring_to_front(view: uintptr_t);

    fn wlc_view_get_mask(view: uintptr_t) -> u32;

    fn wlc_view_set_mask(view: uintptr_t, mask: u32);

    fn wlc_view_get_geometry(view: uintptr_t) -> *const Geometry;

    fn wlc_view_get_visible_geometry(view: uintptr_t, geo: *mut Geometry);

    fn wlc_view_set_geometry(view: uintptr_t, edges: u32, geo: *const Geometry);

    fn wlc_view_get_type(view: uintptr_t) -> ViewType;

    fn wlc_view_set_type(view: uintptr_t, view_type: ViewType, toggle: bool);

    fn wlc_view_get_state(view: uintptr_t) -> ViewState;

    fn wlc_view_set_state(view: uintptr_t, state: ViewState, toggle: bool);

    // Parent is Option<View>
    fn wlc_view_get_parent(view: uintptr_t) -> uintptr_t;

    // Parent is Option<View>
    fn wlc_view_set_parent(view: uintptr_t, parent: uintptr_t);

    fn wlc_view_get_title(view: uintptr_t) -> *const c_char;

    fn wlc_view_get_class(view: uintptr_t) -> *const c_char;

    fn wlc_view_get_app_id(view: uintptr_t) -> *const c_char;
}

impl From<WlcView> for WlcOutput {
    fn from(view: WlcView) -> Self {
        WlcOutput(view.0)
    }
}

impl From<WlcOutput> for WlcView {
    fn from(output: WlcOutput) -> Self {
        WlcView(output.0)
    }
}


impl WlcOutput {
    /// Compatability/debugging function.
    ///
    /// wlc internally stores views and outputs under the same type.
    /// If for some reason a conversion between the two was required,
    /// this function could be called. If this is the case please submit
    /// a bug report.
    pub fn as_view(self) -> WlcView {
        return WlcView::from(self)
    }

    /// Gets user-specified data.
    ///
    /// # Unsafety
    /// The wlc implementation of this method uses `void*` pointers
    /// for raw C data. This function will internaly do a conversion
    /// between the input `T` and a `libc::c_void`.
    ///
    /// This is a highly unsafe conversion with no guarantees. As
    /// such, usage of these functions requires an understanding of
    /// what data they will have. Please review wlc's usage of these
    /// functions before attempting to use them yourself.
    pub unsafe fn get_user_data<T>(&self) -> &mut T {
        let raw_data = wlc_handle_get_user_data(self.0);
        return &mut *(raw_data as *mut T);
    }

    /// Sets user-specified data.
    ///
    /// # Unsafety
    /// The wlc implementation of this method uses `void*` pointers
    /// for raw C data. This function will internaly do a conversion
    /// between the input `T` and a `libc::c_void`.
    ///
    /// This is a highly unsafe conversion with no guarantees. As
    /// such, usage of these functions requires an understanding of
    /// what data they will have. Please review wlc's usage of these
    /// functions before attempting to use them yourself.
    pub unsafe fn set_user_data<T>(&self, data: &T) {
        let data_ptr: *const c_void = data as *const _ as *const c_void;
        wlc_handle_set_user_data(self.0, data_ptr);
    }

    /// Schedules output for rendering next frame.
    ///
    /// If the output was already scheduled, this is
    /// a no-op; if output is currently rendering,
    /// it will render immediately after.
    pub fn schedule_render(&self) {
        unsafe { wlc_output_schedule_render(self.0) };
    }

    /// Gets a list of the current outputs.
    pub fn list() -> Vec<WlcOutput> {
        unsafe {
            let mut out_memb: libc::size_t = 0;
            let outputs = wlc_get_outputs(&mut out_memb);
            if outputs.is_null() {
                return Vec::new();
            }
            let mut result = Vec::with_capacity(out_memb);
            for index in (0 as isize) .. (out_memb as isize) {
                result.push(WlcOutput(*(outputs.offset(index))));
            }
            result
        }
    }

    /// Gets the currently focused output.
    pub fn focused() -> WlcOutput {
        unsafe { WlcOutput(wlc_get_focused_output()) }
    }

    /// Gets the name of the WlcOutput.
    ///
    /// Names are usually assigned in the format WLC-n,
    /// where the first output is WLC-1.
    pub fn get_name(&self) -> String {
        let name: *const i8;
        unsafe {
            name = wlc_output_get_name(self.0);
            pointer_to_string(name)
        }
    }

    /// Gets the sleep status of the output.
    ///
    /// Returns `true` if the monitor is sleeping,
    /// such as having been set with `set_sleep`.
    pub fn get_sleep(&self) -> bool {
        unsafe { wlc_output_get_sleep(self.0) }
    }

    /// Sets the sleep status of the output.
    pub fn set_sleep(&self, sleep: bool) {
        unsafe { wlc_output_set_sleep(self.0, sleep); }
    }

    /// Gets the output resolution in pixels.
    pub fn get_resolution(&self) -> &Size {
        unsafe { &*wlc_output_get_resolution(self.0) }
    }

    /// Sets the resolution of the output.
    pub fn set_resolution(&self, size: Size) {
        unsafe { wlc_output_set_resolution(self.0, &size); }
    }

    /// Get views in stack order.
    ///
    /// This is mainly useful for wm's who need another view stack for inplace sorting.
    /// For example tiling wms, may want to use this to keep their tiling order separated
    /// from floating order.
    /// This handles `wlc_output_get_views` and `wlc_output_get_mutable_views`.
    pub fn get_views(&self) -> Vec<WlcView> {
        unsafe {
            let mut out_memb: libc::size_t = 0;
            let views = wlc_output_get_views(self.0, &mut out_memb);
            if views.is_null() {
                return Vec::new();
            }
            let mut result = Vec::with_capacity(out_memb);

            for index in (0 as isize) .. (out_memb as isize) {
                  result.push(WlcView(*(views.offset(index))));
            }
            return result;
        }
    }

    /// Gets the mask of this output
    pub fn get_mask(&self) -> u32 {
        unsafe { wlc_output_get_mask(self.0) }
    }

    /// Sets the mask for this output
    pub fn set_mask(&self, mask: u32) {
        unsafe { wlc_output_set_mask(self.0, mask) }
    }

    /// # Deprecated
    /// This function is equivalent to simply calling get_views
    pub fn get_mutable_views(&self) -> Vec<WlcView> {
        self.get_views()
    }

    /// Attempts to set the views of a given output.
    ///
    /// Returns true if the operation succeeded.
    pub fn set_views(&self, views: &mut Vec<&WlcView>) -> Result<(), &'static str> {
            let view_len = views.len() as libc::size_t;
            let view_vals: Vec<uintptr_t> = views.into_iter().map(|v| v.0).collect();
            let const_views = view_vals.as_ptr();
        unsafe {
            match wlc_output_set_views(self.0, const_views, view_len) {
                true => Ok(()),
                false => Err("Could not set views on output"),
            }
        }
    }

    /// Focuses compositor on a specific output.
    ///
    /// Pass in Option::None for no focus.
    pub fn focus(output: Option<&WlcOutput>) {
        unsafe {
            match output {
                Some(output) => wlc_output_focus(output.0),
                None => wlc_output_focus(0)
            }
        }
    }
}

impl WlcView {
    /// Compatability/debugging function.
    ///
    /// wlc internally stores views and outputs under the same type.
    /// If for some reason a conversion between the two was required,
    /// this function could be called. If this is the case please submit
    /// a bug report.
    pub fn as_output(self) -> WlcOutput {
        WlcOutput::from(self)
    }

    /// Returns a reference to the root window (desktop background).
    ///
    /// # Example
    /// ```
    /// use rustwlc::handle::WlcView;
    ///
    /// let view = WlcView::root();
    /// assert!(view.is_root());
    /// ```
    pub fn root() -> WlcView {
        WlcView(0)
    }

    /// Whether this view is the root window (desktop background).
    ///
    /// # Example
    /// ```rust
    /// use rustwlc::handle::WlcView;
    /// # // This example can be run because WlcView::root() does not interact with wlc
    /// let view = WlcView::root();
    /// assert!(view.is_root());
    /// ```
    #[inline]
    pub fn is_root(&self) -> bool {
        self.0 == 0
    }

    /// Whether this view is not the root window (desktop background).
    ///
    /// # Usage
    /// A convenience method, the opposite of `view.is_root()`.
    ///
    /// # Example
    /// ```rust
    /// use rustwlc::handle::WlcView;
    ///
    /// let view = WlcView::root();
    /// assert!(view.is_root());
    /// assert!(!view.is_window());
    /// ```
    #[inline]
    pub fn is_window(&self) -> bool {
        self.0 != 0
    }

    /// Gets user-specified data.
    ///
    /// # Unsafety
    /// The wlc implementation of this method uses `void*` pointers
    /// for raw C data. This function will internaly do a conversion
    /// between the input `T` and a `libc::c_void`.
    ///
    /// This is a highly unsafe conversion with no guarantees. As
    /// such, usage of these functions requires an understanding of
    /// what data they will have. Please review wlc's usage of these
    /// functions before attempting to use them yourself.
    pub unsafe fn get_user_data<T>(&self) -> &mut T {
        let raw_data = wlc_handle_get_user_data(self.0);
        return &mut *(raw_data as *mut T);
    }

    /// Sets user-specified data.
    ///
    /// # Unsafety
    /// The wlc implementation of this method uses `void*` pointers
    /// for raw C data. This function will internaly do a conversion
    /// between the input `T` and a `libc::c_void`.
    ///
    /// This is a highly unsafe conversion with no guarantees. As
    /// such, usage of these functions requires an understanding of
    /// what data they will have. Please review wlc's usage of these
    /// functions before attempting to use them yourself.
    pub unsafe fn set_user_data<T>(&self, data: &T) {
        let data_ptr: *const c_void = data as *const _ as *const c_void;
        wlc_handle_set_user_data(self.0, data_ptr);
    }

    /// Closes this view.
    ///
    /// For the main windows of most programs, this should close the program where applicable.
    ///
    /// # Behavior
    /// This function will not do anything if `view.is_root()`.
    pub fn close(&self) {
        if self.is_root() { return };
        unsafe { wlc_view_close(self.0); }
    }

    /// Gets the WlcOutput this view is currently part of.
    pub fn get_output(&self) -> WlcOutput {
        unsafe { WlcOutput(wlc_view_get_output(self.0)) }
    }

    /// Sets the output that the view renders on.
    ///
    /// This may not be supported by wlc at this time.
    pub fn set_output(&self, output: &WlcOutput) {
        unsafe { wlc_view_set_output(self.0, output.0) }
    }

    /// Brings this view to focus.
    ///
    /// Can be called on `WlcView::root()` to lose all focus.
    pub fn focus(&self) {
        unsafe { wlc_view_focus(self.0); }
    }

    /// Sends the view to the back of the compositor
    pub fn send_to_back(&self) {
        unsafe { wlc_view_send_to_back(self.0); }
    }

    /// Sends this view underneath another.
    pub fn send_below(&self, other: &WlcView) {
        unsafe { wlc_view_send_below(self.0, other.0); }
    }

    /// Brings this view above another.
    pub fn bring_above(&self, other: &WlcView) {
        unsafe { wlc_view_bring_above(self.0, other.0); }
    }

    /// Brings this view to the front of the stack
    /// within its WlcOutput.
    pub fn bring_to_front(&self) {
        unsafe { wlc_view_bring_to_front(self.0); }
    }

    // TODO Get masks enum working properly
    /// Gets the current visibilty bitmask for the view.
    pub fn get_mask(&self) -> u32 {
        unsafe { wlc_view_get_mask(self.0) }
    }

    // TODO Get masks enum working properly
    /// Sets the visibilty bitmask for the view.
    pub fn set_mask(&self, mask: u32) {
        unsafe { wlc_view_set_mask(self.0, mask); }
    }

    /// Gets the geometry of the view.
    pub fn get_geometry(&self) -> Option<&Geometry> {
        unsafe {
            let geometry = wlc_view_get_geometry(self.0);
            if geometry.is_null() {
                None
            } else {
                Some(&*geometry)
            }
        }
    }

    /// Gets the geometry of the view (that wlc displays).
    pub fn get_visible_geometry(&self) -> Geometry {
        let mut geo = Geometry { origin: Point { x: 0, y: 0}, size: Size { w: 0, h: 0 }};
        unsafe {
            wlc_view_get_visible_geometry(self.0, &mut geo);
        }
        return geo;
    }

    /// Sets the geometry of the view.
    ///
    /// Set edges if geometry is caused by interactive resize.
    pub fn set_geometry(&self, edges: ResizeEdge, geometry: &Geometry) {
        unsafe { wlc_view_set_geometry(self.0, edges.bits(), geometry as *const Geometry); }
    }

    /// Gets the type bitfield of the curent view
    pub fn get_type(&self) -> ViewType {
        unsafe { wlc_view_get_type(self.0) }
    }

    /// Set flag in the type field. Toggle indicates whether it is set.
    pub fn set_type(&self, view_type: ViewType, toggle: bool) {
        unsafe { wlc_view_set_type(self.0, view_type, toggle); }
    }

    // TODO get bitflags enums
    /// Get the current ViewState bitfield.
    pub fn get_state(&self) -> ViewState {
        unsafe { wlc_view_get_state(self.0) }
    }

    /// Set ViewState bit. Toggle indicates whether it is set or not.
    pub fn set_state(&self, state: ViewState, toggle: bool) {
        unsafe { wlc_view_set_state(self.0, state, toggle); }
    }

    /// Gets parent view, returns `WlcView::root()` if this view has no parent.
    pub fn get_parent(&self) -> WlcView {
        unsafe { WlcView(wlc_view_get_parent(self.0)) }
    }

    /// Set the parent of this view.
    ///
    /// Call with `WlcView::root()` to make its parent the root window.
    pub fn set_parent(&self, parent: &WlcView) {
        unsafe { wlc_view_set_parent(self.0, parent.0); }
    }

    /// Get the title of the view
    pub fn get_title(&self) -> String {
        let chars: *const i8;
        unsafe {
            chars = wlc_view_get_title(self.0);
            if chars == 0 as *const i8 {
                String::new()
            } else {
                    pointer_to_string(chars)
            }
        }
    }

    /// Get class (shell surface only).
    pub fn get_class(&self) -> String {
        let chars: *const i8;
        unsafe {
            chars = wlc_view_get_class(self.0);
            if chars == 0 as *const i8 {
                String::new()
            } else {
                pointer_to_string(chars)
            }
        }
    }

    /// Get app id (xdg-surface only).
    pub fn get_app_id(&self) -> String {
        let chars: *const i8;
        unsafe {
            chars = wlc_view_get_app_id(self.0);
            if chars == 0 as *const i8 {
                String::new()
            } else {
                pointer_to_string(chars)
            }
        }
    }
}
