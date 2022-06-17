use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use codebake::lisp;

struct App {
    env: lisp::Environment<'static>,
    text_input: NodeRef,
    output: String,
}

enum Msg {
    Run,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            env: lisp::default_env(),
            text_input: NodeRef::default(),
            output: String::new(),
        }
    }
    
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Run => {
                let input: String = self.text_input.cast::<HtmlTextAreaElement>().unwrap().value();
                let split = input.split('\n');
                log::debug!("running script {}", input);
                
                for expr in split {
                    if expr == "" {
                        continue;
                    }

                    match lisp::parse_eval(expr.to_string(), &mut self.env) {
                        Ok(expr) => self.output = format!("{}", expr),
                        Err(e) => self.output = format!("{}", e),
                    }
                }
                log::debug!("output from script: {}", self.output);

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_: MouseEvent| { Msg::Run });

        html! {
            <div id="app">
                <div class="half">
                    <label for="input">{ "script" }</label>
                    <textarea ref={self.text_input.clone()} class="textbox" id="input"></textarea>
                    <button {onclick} type="button">{ "run" }</button>
                </div>

                <div class="half">
                    <label for="output">{ "output from script" }</label>
                    <textarea disabled=true class="textbox" id="output" value={ self.output.clone() } />
                </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
