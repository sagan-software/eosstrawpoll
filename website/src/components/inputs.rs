use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum Size {
    Small,
    Medium,
    Large,
}

impl Size {
    pub fn as_str(&self) -> &str {
        match self {
            Size::Small => "sm",
            Size::Medium => "md",
            Size::Large => "lg",
        }
    }
}

impl ToString for Size {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl Default for Size {
    fn default() -> Self {
        Size::Medium
    }
}

#[derive(PartialEq, Clone)]
pub enum ButtonStyle {
    Primary,
    Link,
    Danger,
}

impl ButtonStyle {
    pub fn as_str(&self) -> &str {
        match self {
            ButtonStyle::Primary => "primary",
            ButtonStyle::Link => "link",
            ButtonStyle::Danger => "danger",
        }
    }
}

impl ToString for ButtonStyle {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl Default for ButtonStyle {
    fn default() -> Self {
        ButtonStyle::Primary
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct Button {
    pub size: Size,
    pub style: ButtonStyle,
    pub text: String,
    pub type_: String,
    pub disabled: bool,
    pub class: String,
}

impl Component for Button {
    type Message = ();
    type Properties = Self;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        props
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.size = props.size;
        self.style = props.style;
        self.text = props.text;
        self.type_ = props.type_;
        self.disabled = props.disabled;
        self.class = props.class;
        true
    }
}

impl Renderable<Button> for Button {
    fn view(&self) -> Html<Self> {
        let class = format!(
            "btn btn-{} btn-{} {}",
            self.size.as_str(),
            self.style.as_str(),
            self.class,
        );
        html! {
            <button
                class=class,
                type=&self.type_,
                disabled=self.disabled,
            >
                { &self.text }
            </button>
        }
    }
}

pub struct TextInput {}

pub struct CheckboxInput {}

pub struct RadioInput {}
