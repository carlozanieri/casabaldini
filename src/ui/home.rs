use dioxus::prelude::*;
use crate::models::Slider;

#[derive(Clone)]
pub struct HomeProps {
    pub sliders: Vec<Slider>,
}

#[component]
pub fn Home(props: HomeProps) -> Element {
    let sliders = &props.sliders;

    rsx! {
        div {
            "Hello Slider",
            // Qui puoi fare il mapping dei tuoi slider reali:
            // sliders.iter().map(|s| rsx! { ... })
        }
    }
}
