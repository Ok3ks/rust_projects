use leptos::*;
use leptos_meta::*;
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

                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <div class = "background">
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        </div>
    }
}

#[component]
fn Template() -> impl IntoView {
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
