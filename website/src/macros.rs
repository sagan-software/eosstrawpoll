#[macro_export]
macro_rules! page_view {
    ($($t:ty)*) => ($(
        impl Renderable<$t> for $t {
            fn view(&self) -> Html<Self> {
                document().set_title(&self.title());
                html! {
                    <div class=format!("page {}", self.class()), >
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
