use yew::prelude::*;


#[derive(Clone, PartialEq)]
enum Item {
    Seed,
    Plant,
    Manure
}

#[derive(Clone, PartialEq)]
struct Product {
    item: Item,
    name: String,
    rate: i32
}
#[derive(Properties, PartialEq)]
struct Products {
    products: Vec<Product>
}
impl Product {
    fn new(name: &str, rate: i32, item: Item) -> Self {
        let name = name.to_string();
        Self {
            item,
            name,
            rate
        }
    }
    fn show_type(&self) -> &str {
        match self.item {
            Item::Seed => "Seed",
            Item::Plant => "Plant",
            Item::Manure => "Manure"
        }
    }
}
#[derive(Clone, PartialEq)]
struct Slide {
    title: String,
    desc: String,
    img: String,
    rank: i32
}
#[derive(Properties, PartialEq)]
struct SlidesProps {
    slides: Vec<Slide>
}

impl Slide {

    fn new(title: &str, desc: &str, img: &str, rank: i32) -> Self {
        let title = title.to_string();
        let desc = desc.to_string();
        let img = img.to_string();

        Self {
            title,
            desc,
            rank,
            img
        }
    }
    fn convert(&self) -> Html {
        html! {
            <div class={format!("slide slide-{}", self.rank.clone().to_string())}>
                <div class="text-area my-macchiato-text">
                    <h3>{self.title.clone()}</h3>
                    <p>{self.desc.clone()}</p>
                </div>
                <img src={self.img.clone()} />
            </div>
        }
    }
}
#[function_component(SlidesComp)]
fn slides_comp(slides: &SlidesProps) -> Html {
    slides.slides.iter().map(| slide | html! {
        {slide.convert()}
    }).collect()
}
#[function_component(Header)]
fn header_comp() -> Html {
    html! {
        <header class="my-mocha-green">
            <h2>{"PlantGreen"}</h2>
            <nav>
                <a>{"Plants"}</a>
                <a>{"Seeds"}</a>
                <a>{"Contact"}</a>
            </nav>

            <button><i class="fa-solid fa-magnifying-glass"></i></button>
        </header>
    }
}

#[function_component(GetProducts)]
fn get_products_comp() -> Html {
    let products = vec![
        Product::new("Sunflower", 5, Item::Plant),
        Product::new("Roses", 5, Item::Plant),
        Product::new("Cow Manure", 3, Item::Manure)
    ];

    products.iter().map(| product | html! {
        <div class="product my-frappe-peach">
            <h3>{product.clone().name}</h3>
            <span class="type">{product.clone().show_type()}</span>
            <span class="rating">{product.clone().rate.to_string()}</span>
        </div>
    }).collect()
}
#[function_component(App)]
fn app() -> Html {
    let slides = vec![
        Slide::new("Plant a Tree!", "Tree is something essential for our life", "images/tree_woman.jpg", 1),
        Slide::new("Climate Change", "Everyday we have been tormented by these sudden changes done to our climate. Lets change that!", "images/climate_change.jpg", 2)
    ];
    html! {
        <>
            <Header />

            <main>
                <section class="carousel">
                    <SlidesComp slides={slides} />
                </section>

                <section class="items-rec">
                <h2>{"Buy our stuff!"}</h2>
                    <GetProducts />
                </section>
            </main>
        </>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}