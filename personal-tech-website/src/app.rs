use leptos::*;
use leptos_meta::*;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use leptos_router::*;
stylance::import_crate_style!(my_style, "style/main.scss");

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/personal-tech-website.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/template" view=Template/>
                    <Route path="/dynamic" view=DynamicTemplate/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Theme {
    Light,
    Dark
}

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = localStorage)]
//     fn getItem(key: &str) -> Option<String>;
    
//     #[wasm_bindgen(js_namespace = localStorage)]
//     fn setItem(key: &str, value: &str);
    
//     #[wasm_bindgen(js_namespace = document)]
//     fn querySelector(selector: &str) -> JsValue;
    
//     #[wasm_bindgen(js_name = "document.documentElement.classList.add")]
//     fn add_class_to_html(class: &str);
    
//     #[wasm_bindgen(js_name = "document.documentElement.classList.remove")]
//     fn remove_class_from_html(class: &str);
// }

// #[component]
// pub fn ThemeToggle() -> impl IntoView {
//     // Initialize theme from localStorage or default to light
//     let initial_theme = match getItem("theme").as_deref() {
//         Some("dark") => Theme::Dark,
//         _ => Theme::Light,
//     };
    
//     // Create a signal to track theme state
//     let (theme, set_theme) = create_signal(initial_theme);
    
//     // Apply initial theme to document
//     if theme() == Theme::Dark {
//         add_class_to_html("night-mode");
//     } else {
//         remove_class_from_html("night-mode");
//     }
    
//     // Toggle the theme
//     let toggle_theme = move |_| {
//         set_theme.update(|current_theme| {
//             match current_theme {
//                 Theme::Light => {
//                     setItem("theme", "dark");
//                     add_class_to_html("night-mode");
//                     *current_theme = Theme::Dark;
//                 },
//                 Theme::Dark => {
//                     setItem("theme", "light");
//                     remove_class_from_html("night-mode");
//                     *current_theme = Theme::Light;
//                 }
//             }
//         });
//     };
    
//     view! {
//         <button
//             class="theme-toggle"
//             aria-label="Toggle dark/light mode"
//             on:click=toggle_theme
//         >
//             <div class=move || {
//                 let base = "toggle-track";
//                 if theme() == Theme::Dark {
//                     format!("{} dark-active", base)
//                 } else {
//                     base.to_string()
//                 }
//             }>
//                 <span class="sun-icon">"☀️"</span>
//                 <span class="moon-icon">"🌙"</span>
//                 <div class="toggle-thumb"></div>
//             </div>
//         </button>
//     }
// }

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    // let (night_mode, set_night_mode) = create_signal(false);

    view! {
        
        // <nav>
        // <div>
        // <ThemeToggle/>
        // </div>
        // </nav>

        <header class="bg-navy">
        <div class="container" style="padding: 4rem 1rem;">
            <h1 style="color: var(--cream); border-color: var(--green);">"Emmanuel Okedele"</h1>
            <p style="color: var(--cream); font-size: 1.2rem; max-width: 600px; margin-bottom: 2rem;">
                "Discover our collection of sustainable products designed with the planet in mind."
            </p>
            <A href="#articles" class="btn">"Browse Collection"</A  > //use Navigate to move to another page on click
            <A href="#about" class="btn btn-secondary">"Reach Out"</A>
        </div>
    </header>



    <section style="padding: 3rem 0;">
    <div class="container">
        <h2>"Random Jokes"</h2>
    </div>
        </section>

    <section style="padding: 3rem 0;">
        <div class="container">
            <h2>"About Me"</h2>
        </div>
    </section>

    <section style="padding: 3rem 0;">
    <div class="container">
        <h2>"Articles"</h2>
    </div>
    </section>

    <div class="footer-grid">
    <A href="mailto:okedeleayodeji60@gmail.com">"Email"</A  >
    <A href="github.com/ok3ks">"Github"</A  >
    </div>

    //Use form as contact 

    }
}
#[component]
fn DynamicTemplate() -> impl IntoView {
    view! {

<div class="container" style="padding-top: 1.5rem;">
        <nav style="border-radius: var(--border-radius-lg); overflow: hidden;">
            <div class="container">
                <ul>
                    <li><a href="#home">"Home"</a></li>
                    <li><a href="#about">"About"</a></li>
                    <li><a href="#products">"Products"</a></li>
                    <li><a href="#sustainability">"Sustainability"</a></li>
                    <li><a href="#contact">"Contact"</a></li>
                </ul>
            </div>
        </nav>
    </div>

    // <!-- Dynamic Hero Section -->
    <div class="container">
        <div class="hero-container">
            <div class="hero-content">
                <div>
                    <span class="pill pill-green">"Eco-Friendly"</span>
                    <span class="pill pill-taupe">"Sustainable"</span>
                    <span class="pill pill-green">"Plastic-Free"</span>
                </div>
                <h1>"Reimagine Your Sustainable Lifestyle"</h1>
                <p>"Curated products for mindful living, designed with purpose and care for our planet."</p>
                <div style="margin-top: 2rem;">
                    <a href="#products" class="btn">"Shop Collection"</a>
                    <a href="#about" class="btn btn-secondary" style="margin-left: 1rem;">"Our Story"</a>
                </div>
            </div>
            <div class="hero-image">
                // <img src="hero-image.jpg" alt="Sustainable lifestyle products">
            </div>
        </div>
    </div>

    // <!-- Overlapping Grid Layout -->
    <div class="container">
        <h2 style="text-align: center;">"Featured Collections"</h2>
        <div class="overlap-grid">
            <div class="overlap-item">
                <div class="overlap-image">
                    // <img src="kitchen.jpg" alt="Sustainable Kitchen">
                </div>
                <div class="overlap-content">
                    <h3>"Plastic-Free Kitchen"</h3>
                    <p>"Transform your kitchen with sustainable alternatives to single-use plastics."</p>
                    <a href="#kitchen" class="btn btn-secondary">"Explore"</a>
                </div>
            </div>
            
            <div class="overlap-item">
                <div class="overlap-image">
                    // <img src="bath.jpg" alt="Eco Bath Products">
                </div>
                <div class="overlap-content">
                    <h3>"Eco Bath & Body"</h3>
                    <p>"Personal care products that nurture you and the environment."</p>
                    <a href="#bath" class="btn btn-secondary">"Explore"</a>
                </div>
            </div>
            
            <div class="overlap-item">
                <div class="overlap-image">
                    // <img src="home.jpg" alt="Sustainable Home">
                </div>
                <div class="overlap-content">
                    <h3>"Conscious Home"</h3>
                    <p>"Ethically made decor and household essentials."</p>
                    <a href="#home-goods" class="btn btn-secondary">"Explore"</a>
                </div>
            </div>
            
            <div class="overlap-item">
                <div class="overlap-image">
                    // <img src="outdoor.jpg" alt="Outdoor Essentials">
                </div>
                <div class="overlap-content">
                    <h3>"Outdoor Essentials"</h3>
                    <p>"Gear for adventures with minimal environmental impact."</p>
                    <a href="#outdoor" class="btn btn-secondary">"Explore"</a>
                </div>
            </div>
        </div>
    </div>

    // <!-- Skewed Background Features Section -->
    <section class="feature-section">
        <div class="container">
            <h2 style="text-align: center; color: var(--cream);">"Why Choose Earth Tones"</h2>
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 2rem; margin-top: 3rem;">
                <div class="card-curved">
                    <div style="padding: 2rem; text-align: center;">
                        <div style="width: 80px; height: 80px; background-color: var(--taupe); border-radius: 50%; display: flex; align-items: center; justify-content: center; margin: 0 auto 1.5rem auto;">
                            <span style="font-size: 2rem; color: var(--navy);">"♻️"</span>
                        </div>
                        <h3>"Sustainable Materials"</h3>
                        <p>"Every product is carefully crafted from renewable or biodegradable materials."</p>
                    </div>
                </div>
                
                <div class="card-curved">
                    <div style="padding: 2rem; text-align: center;">
                        <div style="width: 80px; height: 80px; background-color: var(--taupe); border-radius: 50%; display: flex; align-items: center; justify-content: center; margin: 0 auto 1.5rem auto;">
                            <span style="font-size: 2rem; color: var(--navy);">"🌱"</span>
                        </div>
                        <h3>"Plastic-Free Packaging"</h3>
                        <p>"All items ship in compostable or recyclable packaging with zero plastic."</p>
                    </div>
                </div>
                
                <div class="card-curved">
                    <div style="padding: 2rem; text-align: center;">
                        <div style="width: 80px; height: 80px; background-color: var(--taupe); border-radius: 50%; display: flex; align-items: center; justify-content: center; margin: 0 auto 1.5rem auto;">
                            <span style="font-size: 2rem; color: var(--navy);">"🤝"</span>
                        </div>
                        <h3>"Ethical Production"</h3>
                        <p>"We partner with artisans and manufacturers who provide fair wages and safe conditions."</p>
                    </div>
                </div>
            </div>
        </div>
    </section>

    // <!-- Testimonials with Quote Mark -->
    <div class="container" style="padding: 5rem 0;">
        <h2 style="text-align: center;">"Community Voices"</h2>
        <div class="testimonial-grid">
            <div class="testimonial-card">
                <p>"I've tried many eco-friendly products, but Earth Tones stands out for quality and attention to detail. Their beeswax wraps have completely replaced plastic wrap in my kitchen."</p>
                <div style="margin-top: 2rem; display: flex; align-items: center;">
                    <div style="width: 50px; height: 50px; border-radius: 50%; background-color: var(--green); overflow: hidden; margin-right: 1rem;">
                        // <img src="person1.jpg" alt="Customer">
                    </div>
                    </div>
                    </div>
                    </div>

                    </div>


}}

#[component]
fn Template() -> impl IntoView {
    // let (nightMode , setNightMode) = create_signal(false);
    view! {
    <nav>
        <div class="container">
            <ul>
                <li><A href="#home">"Home"</A ></li>
                <li><A href="#about">"About"</A></li>
                <li><A href="#products">"Products"</A></li>
                <li><A href="#sustainability">"Sustainability"</A></li>
                <li><A href="#contact">"Contact"</A></li>
            </ul>
        </div>
    </nav>

    // <!-- Hero Section -->
    <header class="bg-navy">
        <div class="container" style="padding: 4rem 1rem;">
            <h1 style="color: var(--cream); border-color: var(--green);">"Natural Living, Simplified"</h1>
            <p style="color: var(--cream); font-size: 1.2rem; max-width: 600px; margin-bottom: 2rem;">
                "Discover our collection of sustainable products designed with the planet in mind."
            </p>
            <A href="#products" class="btn">"Browse Collection"</A  >
            <A href="#about" class="btn btn-secondary">"Learn More"</A>
        </div>
    </header>

    // <!-- Featured Products -->
    <section id="products" style="padding: 3rem 0;">
        <div class="container">
            // <h2>Featured Products</h2>
            <p>"Handcrafted with sustainable materials and minimal environmental impact."</p>
            
            <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 2rem; margin-top: 2rem;">
                // <!-- Product Card 1 -->
                <div class="card">
                    // <img src="product1.jpg" alt="Eco-friendly Water Bottle" style="width: 100%; height: 200px; object-fit: cover; border-radius: 4px;">
                    <h3 style="margin-top: 1rem; color: var(--brown);">"Bamboo Water Bottle"</h3>
                    <p>"Made from sustainable bamboo with zero plastic components."</p>
                    <p class="text-accent" style="font-weight: 600; font-size: 1.2rem;">"$24.99"</p>
                    <button class="btn" style="width: 100%; margin-top: 1rem;">"Add to Cart"</button>
                </div>
                
                // <!-- Product Card 2 -->
                <div class="card">
                    // <img src="product2.jpg" alt="Hemp Tote Bag" style="width: 100%; height: 200px; object-fit: cover; border-radius: 4px;">
                    <h3 style="margin-top: 1rem; color: var(--brown);">"Hemp Tote Bag"</h3>
                    <p>"Durable hemp material designed to replace hundreds of plastic bags."</p>
                    <p class="text-accent" style="font-weight: 600; font-size: 1.2rem;">"$18.50"</p>
                    <button class="btn" style="width: 100%; margin-top: 1rem;">"Add to Cart"</button>
                </div>
                
                // <!-- Product Card 3 -->
                <div class="card card-accent">
                    // <img src="product3.jpg" alt="Beeswax Wraps" style="width: 100%; height: 200px; object-fit: cover; border-radius: 4px;">
                    <h3 style="margin-top: 1rem; color: var(--brown);">Beeswax Food Wraps</h3>
                    <p>Reusable alternative to plastic wrap, handmade with organic cotton.</p>
                    <p class="text-accent" style="font-weight: 600; font-size: 1.2rem;">$15.99</p>
                    <button class="btn" style="width: 100%; margin-top: 1rem;">Add to Cart</button>
                </div>
            </div>
            
            <div style="text-align: center; margin-top: 3rem;">
                <A href="#all-products" class="btn btn-dark">View All Products</A  >
            </div>
        </div>
    </section>

    // <!-- About Section -->
    <section id="about" class="bg-taupe" style="padding: 3rem 0;">
        <div class="container">
            <h2 style="color: var(--navy); border-color: var(--navy);">Our Mission</h2>
            <div style="display: flex; flex-wrap: wrap; gap: 2rem; align-items: center;">
                <div style="flex: 1; min-width: 300px;">
                    <p style="font-size: 1.1rem; color: var(--navy);">
                        "Founded in 2020, Earth Tones is committed to providing sustainable alternatives to everyday products. 
                        We believe small changes can make A big impact on our planet."
                    </p>
                    <blockquote>
                        "We don't need A handful of people doing zero waste perfectly. 
                        We need millions doing it imperfectly."
                    </blockquote>
                    <p>
                        "Our products are sourced from ethical suppliers and crafted with minimal environmental impact. 
                        We're proud to be carbon-neutral since 2022."
                    </p>
                </div>
                <div style="flex: 1; min-width: 300px;">
                    // <img src="about-image.jpg" alt="Our workshop" style="width: 100%; border-radius: 8px; box-shadow: 0 4px 8px rgba(0,0,0,0.1);">
                </div>
            </div>
        </div>
    </section>

    // <!-- Testimonials -->
    <section style="padding: 3rem 0;">
        <div class="container">
            <h2>"What Our Customers Say"</h2>
            <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 2rem; margin-top: 2rem;">
                // <!-- Testimonial 1 -->
                <div class="card">
                    <p style="font-style: italic;">
                        "The quality of these products is exceptional. I've been using the beeswax wraps for months and they're still good as new."
                    </p>
                    <p style="font-weight: 600; color: var(--brown);">- "Sarah J."</p>
                </div>
                
                // <!-- Testimonial 2 -->
                <div class="card">
                    <p style="font-style: italic;">
                        "I love how everything is packaged without plastic. Even the shipping materials are compostable!"
                    </p>
                    <p style="font-weight: 600; color: var(--brown);">- "Michael T."</p>
                </div>
                
                // <!-- Testimonial 3 -->
                <div class="card">
                    <p style="font-style: italic;">
                        "The bamboo utensil set is perfect for my lunch bag. No more plastic cutlery for me."
                    </p>
                    <p style="font-weight: 600; color: var(--brown);">- "EmmA L".</p>
                </div>
            </div>
        </div>
    </section>

    // <!-- Newsletter -->
    <section class="bg-green" style="padding: 3rem 0;">
        <div class="container" style="text-align: center;">
            <h2 style="color: var(--cream); border-color: var(--cream);">"Join Our Newsletter"</h2>
            <p style="color: var(--cream); max-width: 600px; margin: 1rem auto;">
                "Subscribe to get sustainable living tips and be the first to know about new products and special offers."
            </p>
            // <form style="max-width: 500px; margin: 2rem auto;">
                <div style="display: flex; gap: 0.5rem;">
                    // <input type="email" placeholder="Your email address" style="margin-bottom: 0;">
                    <button type="submit" class="btn-dark">"Subscribe"</button>
                </div>
            // </form>
        </div>
    </section>

    // <!-- Footer -->
    <footer>
        <div class="container">
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 2rem;">
                <div>
                    <h3 style="color: var(--cream); border: none;">"Earth Tones"</h3>
                    <p>"Sustainable products for everyday living".</p>
                    <p> "2025 Earth Tones. All rights reserved."</p>
                </div>
                <div>
                    <h4 style="color: var(--cream); margin-top: 0;">"Quick Links"</h4>
                    <ul style="list-style: none; padding: 0;">
                        <li><A href="home">Home</A></li>
                        <li><A href="#about">About Us</A></li>
                        <li><A href="#products">Products</A></li>
                        <li><A href="#blog">Blog</A></li>
                    </ul>
                </div>
                <div>
                    <h4 style="color: var(--cream); margin-top: 0;">"Customer Service"</h4>
                    <ul style="list-style: none; padding: 0;">
                        <li><A href="#contact">"Contact Us"</A></li>
                        <li><A href="#shipping">"Shipping Policy"</A></li>
                        <li><A href="#returns">"Returns & Refunds"</A></li>
                        <li><A href="#faq">"FAQ"</A   ></li>
                    </ul>
                </div>
                <div>
                    <h4 style="color: var(--cream); margin-top: 0;">"Connect With Us"</h4>
                    <ul style="list-style: none; padding: 0;">
                        <li><A href="#instagram">"Instagram"</A   ></li>
                        <li><A href="#pinterest">"Pinterest"</A  ></li>
                        <li><A href="#facebook">"Facebook"</A  ></li>
                        <li><A href="#twitter">"Twitter"</A  ></li>
                    </ul>
                </div>
            </div>
        </div>
    </footer>
// </body>
    }
}
/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
