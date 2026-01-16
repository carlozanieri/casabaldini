use dioxus::prelude::*;
use crate::models::Slider;

#[component]
pub fn Home(sliders: Vec<Slider>) -> Element {
    rsx! {
        head {
            meta { charset: "utf-8" }
            meta {
                name: "viewport",
                content: "width=device-width, user-scalable=no, initial-scale=1.0"
            }
            meta {
                http_equiv: "X-Frame-Options",
                content: "SAMEORIGIN"
            }

            link {
                rel: "stylesheet",
                href: "/static/home/dist/css/slider-pro.min.css"
            }
            link {
                rel: "stylesheet",
                href: "/static/home/dist/css/examples.css"
            }
            link {
                rel: "stylesheet",
                href: "/staticmenu_6/css/default.css"
            }
            link {
                rel: "stylesheet",
                href: "/static/beb/menu_6/css/component.css"
            }

            script {
                src: "https://code.jquery.com/jquery-3.6.2.min.js"
            }
            script {
                src: "/static/home/dist/js/jquery.sliderPro.min.js"
            }

            script {
                dangerous_inner_html: r#"
                $(document).ready(function($) {
                    $('#example1').sliderPro({
                        width: 960,
                        height: 500,
                        arrows: true,
                        buttons: false,
                        waitForLayers: true,
                        thumbnailWidth: 200,
                        thumbnailHeight: 100,
                        thumbnailPointer: true,
                        autoplay: true,
                        autoScaleLayers: false,
                        breakpoints: {
                            500: {
                                thumbnailWidth: 120,
                                thumbnailHeight: 50
                            }
                        }
                    });
                });
                "#
            }
        }

        body {
            span {
                style: "position: inherit; margin-top: 65%; margin-left: 28%;",

                div { id: "example1", class: "slider-pro",
                    div { class: "sp-slides",
                        for s in sliders {
                            div { class: "sp-slide",
                                img {
                                    src: "/static/img/index/{s.img}",
                                    alt: "{s.titolo}",
                                    style: "max-width: 110%; height: 110%;"
                                }

                                h3 {
                                    class: "sp-layer sp-black sp-padding",
                                    "data-horizontal": "40",
                                    "data-vertical": "10%",
                                    "{s.titolo}"
                                }

                                p {
                                    class: "sp-layer sp-white sp-padding hide-medium-screen",
                                    "data-horizontal": "40",
                                    "data-vertical": "22%",
                                    "{s.caption}"
                                }

                                p {
                                    class: "sp-layer sp-black sp-padding hide-small-screen",
                                    style: "background-color:#330101;",
                                    "data-horizontal": "40",
                                    "data-vertical": "34%",
                                    "data-width": "650",
                                    "{s.testo}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
