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
                        <img src="rb.png" width="28" height="28"/>
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
            <section class={classes!("is-fullheight")}>
                <div class="container content is-fullheight">
                    <h1 class={classes!("title")}>{ "Welcome to RoBaertschi's Website" }</h1>
                    <p>{"Here you'll find some of my Projects. Here a list of my Projects that i am currently working on."}</p>
                    <ul>
                        <li>
                            <a href={"book.html"}>
                                {"Minecraft Getting Started with Modding Book"}
                            </a>
                        </li>
                        <li>
                            <a href={"https://github.com/RoBaertschi/swisscheeseclient"}>
                                {"Swisscheeseclient"}
                            </a>
                        </li>
                        <li>
                            <a href={"https://github.com/RoBaertschi/stackable"}>
                                {"Stackable"}
                            </a>
                        </li>
                        <li>
                            {"More soon"}
                        </li>
                        
                    </ul>
                </div>
            </section>

            <footer class={classes!("footer")}>
                <div class={classes!("content", "has-text-centered")}> 
                    {"Big Thanks to:"}
                    <br/>
                    {"Icon from "} <a href="https://www.flaticon.com/free-icons/rb" title="rb icons">{"Rb icons created by Muhammad_Usman - Flaticon"}</a>
                </div>
            </footer>

        </>    
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}