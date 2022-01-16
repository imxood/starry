use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use bevy::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

pub struct WebPlugin;

impl Plugin for WebPlugin {
    fn build(&self, app: &mut App) {
        // window resize event flag
        let window_resize_evt = WindowResizeEvent::default();

        let mut window_resize_evt0 = window_resize_evt.clone();

        // 用于处理 Window 的 resize event
        let window_resize_handle = WindowResizeHandle {
            listener: Closure::wrap(Box::new(move || {
                window_resize_evt0.set();
                log::info!("resize!");
            }) as Box<dyn FnMut()>),
        };

        app.insert_resource(window_resize_evt);
        app.insert_resource(window_resize_handle);
        app.add_startup_system(window_maxium);
        app.add_startup_system(window_resize_register);
        app.add_system(window_resize_nofity);
    }
}

// 初始化时, 设置canvas最大化
fn window_maxium(mut windows: ResMut<Windows>) {
    let window = web_sys::window().expect("no global `window` exists");
    let width: f32 = window.inner_width().unwrap().as_f64().unwrap() as f32;
    let height: f32 = window.inner_height().unwrap().as_f64().unwrap() as f32;
    if let Some(window) = windows.get_primary_mut() {
        if window.width() != width || window.height() != height {
            window.set_resolution(width, height);
            log::info!("set_resolution({width}, {height})");
        }
    }
}

/// 注册 Window 的 resize事件
fn window_resize_register(window_resize_handle: Res<WindowResizeHandle<dyn FnMut()>>) {
    let window = web_sys::window().expect("Failed to obtain window");
    window.set_onresize(Some(window_resize_handle.listener.as_ref().unchecked_ref()));
}

fn window_resize_nofity(
    mut window_resize_evt: ResMut<WindowResizeEvent>,
    windows: ResMut<Windows>,
) {
    if window_resize_evt.get() {
        window_maxium(windows);
        window_resize_evt.clear();
    }
}

pub struct WindowResizeHandle<T: ?Sized> {
    pub listener: Closure<T>,
}

unsafe impl<T: ?Sized> Send for WindowResizeHandle<T> {}
unsafe impl<T: ?Sized> Sync for WindowResizeHandle<T> {}

#[derive(Default, Clone)]
pub struct WindowResizeEvent {
    happened: Arc<AtomicBool>,
}

impl WindowResizeEvent {
    pub fn set(&mut self) {
        self.happened.store(true, Ordering::SeqCst);
    }
    pub fn clear(&mut self) {
        self.happened.store(false, Ordering::SeqCst);
    }
    pub fn get(&mut self) -> bool {
        self.happened.load(Ordering::SeqCst)
    }
}
