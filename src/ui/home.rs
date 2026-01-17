use dioxus::prelude::*;
use crate::models::Slider;

#[derive(Clone)]
pub struct HomeProps {
    pub sliders: Vec<Slider>,
}

#[component]
#[component]
pub fn Home(props: HomeProps) -> Element {
    rsx! {
        div {
            class: "slider-container",
            "Benvenuto nella home",

            for slider in props.sliders.iter() {
                div {
                    class: "slider-item",
                    img {
                        src: "{slider.img}",
                        alt: "{slider.titolo}"
                    }
                    h3 { "{slider.titolo}" }
                }
            }
        }
    }
}

