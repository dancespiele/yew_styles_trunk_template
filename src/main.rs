#![recursion_limit = "1024"]

extern crate web_sys;
extern crate yew;
extern crate yew_router;
extern crate yew_styles;

mod app;
mod pages;
use app::App;

pub fn main() {
    yew::start_app::<App>();
}
