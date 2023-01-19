use gloo::utils::document;
use web_sys::Element;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let onclick = 
        move |_event: MouseEvent| {
            let burger: Element = document().get_element_by_id("burger").unwrap();

            
            burger.class_list().toggle("is-active").unwrap();
            

            let menu: Element = document().get_element_by_id("main-navbar").unwrap();
            menu.class_list().toggle("is-active").unwrap();
        };

    

    

    html!{
        <>
            <nav class={classes!("navbar")} role="navigation"> 
                <div class="navbar-brand">
                    <a class="navbar-item" href="index.html">
                        <img src="https://bulma.io/images/bulma-logo.png" width="112" height="28"/>
                    </a>
                
                    <a {onclick} role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample" id="burger">
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </a>
                </div>

                <div id="main-navbar" class={classes!("navbar-menu")}>
                    <div class="navbar-start">
                        <a class={classes!("navbar-item")} href="index.html">
                        {"Home"}
                        </a>
                    </div>
                </div>

            </nav>
            <section>
                <div class="container content">
                    <h1 class={classes!("title")}>{ "Welcome to RoBaertschi's Website" }</h1>
                    
                </div>
            </section>

        </>    
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}