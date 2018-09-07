#[macro_export]
macro_rules! page_view {
    ($($t:ty)*) => ($(
        impl Renderable<$t> for $t {
            fn view(&self) -> Html<Self> {
                ::seo::set_title(&self.get_title());
                ::seo::set_description(&self.get_description());
                ::seo::set_url(&self.get_route().to_absolute());
                let state_class = match self.get_state() {
                    PageState::Loading => "page_loading",
                    PageState::Loaded => "page_loaded",
                    PageState::Error => "page_error",
                };
                let class = format!("page {} {}", state_class, self.get_class());
                html! {
                    <div class=class, >
                        <header class="page_header", >
                            <div class="app_container", >
                                <h1 class="page_title", >
                                    { self.get_title() }
                                </h1>
                            </div>
                        </header>
                        <main class="page_content", >
                            <div class="app_container", >
                                { self.get_content() }
                            </div>
                        </main>
                    </div>
                }
            }
        }
    )*)
}
