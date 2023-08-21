use yew::prelude::*;

struct Calculator {
    value: String,
    memory: f64,
    operator: char,
}

enum Msg {
    AddDigit(char),
    SetOperator(char),
    Compute,
    Clear,
}

impl Component for Calculator {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: String::new(),
            memory: 0.0,
            operator: ' ',
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddDigit(digit) => {
                self.value.push(digit);
            }
            Msg::SetOperator(operator) => {
                self.memory = self.value.parse().unwrap();
                self.value.clear();
                self.operator = operator;
            }
            Msg::Compute => {
                let value: f64 = self.value.parse().unwrap();
                let result = match self.operator {
                    '+' => self.memory + value,
                    '-' => self.memory - value,
                    '*' => self.memory * value,
                    '/' => self.memory / value,
                    _ => 0.0,
                };
                self.value = result.to_string();
                self.memory = 0.0;
                self.operator = ' ';
            }
            Msg::Clear => {
                self.value.clear();
                self.memory = 0.0;
                self.operator = ' ';
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="calculator">
                <div class="display">{ &self.value }</div>
                <div class="row">
                    <button class="digit" onclick={link.callback(|_| Msg::AddDigit('1'))}>{ "1" }</button>
                    <button class="digit" onclick={link.callback(|_| Msg::AddDigit('2'))}>{ "2" }</button>
                    <button class="digit" onclick={link.callback(|_| Msg::AddDigit('3'))}>{ "3" }</button>
                    <button class="operator" onclick={link.callback(|_| Msg::SetOperator('+'))}>{ "+" }</button>
                </div>
                <div class="row">
                    <button class="digit" onclick={link.callback(|_| Msg::AddDigit('4'))}>{ "4" }</button>
                    <button class="digit" onclick={link.callback(|_| Msg::AddDigit('5'))}>{ "5" }</button>
                    <button class="digit" onclick={link.callback(|_| Msg::AddDigit('6'))}>{ "6" }</button>
                    <button class="operator" onclick={link.callback(|_| Msg::SetOperator('-'))}>{ "-" }</button>
                </div>
                <div class="row">
                    <button class="digit" onclick={link.callback(|_| Msg::AddDigit('7'))}>{ "7" }</button>
                    <button class="digit" onclick={link.callback(|_| Msg::AddDigit('8'))}>{ "8" }</button>
                    <button class="digit" onclick={link.callback(|_| Msg::AddDigit('9'))}>{ "9" }</button>
                    <button class="operator" onclick={link.callback(|_| Msg::SetOperator('*'))}>{ "*" }</button>
                </div>
                <div class="row">
                    <button class="operator" onclick={link.callback(|_| Msg::Clear)}>{ "C" }</button>
                    <button class="digit" onclick={link.callback(|_| Msg::AddDigit('0'))}>{ "0" }</button>
                    <button class="operator" onclick={link.callback(|_| Msg::Compute)}>{ "=" }</button>
                    <button class="operator" onclick={link.callback(|_| Msg::SetOperator('/'))}>{ "/" }</button>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Calculator>::new().render();
}
