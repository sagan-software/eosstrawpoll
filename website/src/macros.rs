#[macro_export]
macro_rules! page_view {
    ($($t:ty)*) => ($(
        impl Renderable<$t> for $t {
            fn view(&self) -> Html<Self> {
                let title = format!("{} - EOS Straw Poll", self.title());
                document().set_title(&title);
                let state_class = match self.get_state() {
                    PageState::Loading => "page_loading",
                    PageState::Loaded => "page_loaded",
                    PageState::Error => "page_error",
                };
                let class = format!("page {} {}", state_class, self.class());
                html! {
                    <div class=class, >
                        <header class="page_header", >
                            <div class="app_container", >
                                <h1 class="page_title", >
                                    { self.title() }
                                </h1>
                            </div>
                        </header>
                        <main class="page_content", >
                            <div class="app_container", >
                                { self.content() }
                            </div>
                        </main>
                    </div>
                }
            }
        }
    )*)
}
