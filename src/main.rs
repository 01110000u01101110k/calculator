extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;

#[derive(Default, NwgUi)]
pub struct Calculator {
    #[nwg_control(size: (350, 500), position: (300, 300), title: "Калькулятор", flags: "WINDOW|VISIBLE")]

    #[nwg_events( OnWindowClose: [Calculator::close] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 2, min_size: [310, 470])]
    grid: nwg::GridLayout,

    #[nwg_control(text: "", align: nwg::HTextAlign::Right, readonly: true)]
    #[nwg_layout_item(layout: grid, col: 0, row: 0, col_span: 4)]
    input: nwg::TextInput,

    #[nwg_control(text: "1", focus: true)]
    #[nwg_layout_item(layout: grid, col: 0, row: 2)]
    #[nwg_events( OnButtonClick: [Calculator::change_input(SELF, CTRL)] )]
    btn_1: nwg::Button,

    #[nwg_control(text: "2", focus: true)]
    #[nwg_layout_item(layout: grid, col: 1, row: 2)]
    #[nwg_events( OnButtonClick: [Calculator::change_input(SELF, CTRL)] )]
    btn_2: nwg::Button,

    #[nwg_control(text: "3", focus: true)]
    #[nwg_layout_item(layout: grid, col: 2, row: 2)]
    #[nwg_events( OnButtonClick: [Calculator::change_input(SELF, CTRL)] )]
    btn_3: nwg::Button,

    #[nwg_control(text: "4", focus: true)]
    #[nwg_layout_item(layout: grid, col: 0, row: 3)]
    #[nwg_events( OnButtonClick: [Calculator::change_input(SELF, CTRL)] )]
    btn_4: nwg::Button,

    #[nwg_control(text: "5", focus: true)]
    #[nwg_layout_item(layout: grid, col: 1, row: 3)]
    #[nwg_events( OnButtonClick: [Calculator::change_input(SELF, CTRL)] )]
    btn_5: nwg::Button,

    #[nwg_control(text: "6", focus: true)]
    #[nwg_layout_item(layout: grid, col: 2, row: 3)]
    #[nwg_events( OnButtonClick: [Calculator::change_input(SELF, CTRL)] )]
    btn_6: nwg::Button,

    #[nwg_control(text: "7", focus: true)]
    #[nwg_layout_item(layout: grid, col: 0, row: 4)]
    #[nwg_events( OnButtonClick: [Calculator::change_input(SELF, CTRL)] )]
    btn_7: nwg::Button,

    #[nwg_control(text: "8", focus: true)]
    #[nwg_layout_item(layout: grid, col: 1, row: 4)]
    #[nwg_events( OnButtonClick: [Calculator::change_input(SELF, CTRL)] )]
    btn_8: nwg::Button,

    #[nwg_control(text: "9", focus: true)]
    #[nwg_layout_item(layout: grid, col: 2, row: 4)]
    #[nwg_events( OnButtonClick: [Calculator::change_input(SELF, CTRL)] )]
    btn_9: nwg::Button,

    #[nwg_control(text: "<--", focus: true)]
    #[nwg_layout_item(layout: grid, col: 0, row: 1)]
    #[nwg_events( OnButtonClick: [Calculator::delete] )]
    btn_delete: nwg::Button,

    #[nwg_control(text: "clear", focus: true)]
    #[nwg_layout_item(layout: grid, col: 1, row: 1)]
    #[nwg_events( OnButtonClick: [Calculator::clear] )]
    btn_clear: nwg::Button,

    #[nwg_control(text: "+", focus: true)]
    #[nwg_layout_item(layout: grid, col: 2, row: 1)]
    #[nwg_events( OnButtonClick: [Calculator::change_input(SELF, CTRL)] )]
    btn_plus: nwg::Button,

    #[nwg_control(text: "-", focus: true)]
    #[nwg_layout_item(layout: grid, col: 3, row: 1)]
    #[nwg_events( OnButtonClick: [Calculator::change_input(SELF, CTRL)] )]
    btn_minus: nwg::Button,

    #[nwg_control(text: "*", focus: true)]
    #[nwg_layout_item(layout: grid, col: 3, row: 2)]
    #[nwg_events( OnButtonClick: [Calculator::change_input(SELF, CTRL)] )]
    btn_mult: nwg::Button,

    #[nwg_control(text: "/", focus: true)]
    #[nwg_layout_item(layout: grid, col: 3, row: 3)]
    #[nwg_events( OnButtonClick: [Calculator::change_input(SELF, CTRL)] )]
    btn_divide: nwg::Button,

    #[nwg_control(text: "=", focus: true)]
    #[nwg_layout_item(layout: grid, col: 3, row: 4)]
    #[nwg_events( OnButtonClick: [Calculator::process] )]
    btn_process: nwg::Button,
}

impl Calculator {

    fn change_input(&self, value: &nwg::Button) {
        self.input.set_text(&format!("{}{}", self.input.text(), value.text()));
    }

    fn process(&self) {
        let input_values = self.input.text();

        if input_values.len() == 0 {
            return;
        }

        let mut chars: Vec<String> = vec![];

        let mut numbers: Vec<String> = vec![];

        let input_values_length = input_values.len();
        let mut index_last_char = 0;

        for (index, value) in input_values.char_indices() {
            if value == '+' || value == '-' || value == '*' || value == '/' {
                chars.push(value.to_string());

                numbers.push(input_values[index_last_char .. index].to_string());
                index_last_char = index + 1;
            }
        }
        numbers.push(input_values[index_last_char .. input_values_length].to_string());

        let mut result: i32 = numbers[0].parse::<i32>().unwrap();

        let mut index_first = 0;
        while index_first < chars.len() {
            if chars[index_first] == "*" {
                result *= numbers[index_first + 1].parse::<i32>().unwrap();
            } else if chars[index_first] == "/" {
                result /= numbers[index_first + 1].parse::<i32>().unwrap();
            }

            index_first += 1;
        }

        let mut index_second = 0;
        while index_second < chars.len() {
            if chars[index_second] == "+" {
                result += numbers[index_second + 1].parse::<i32>().unwrap();
            } else if chars[index_second] == "-" {
                result -= numbers[index_second + 1].parse::<i32>().unwrap();
            }

            index_second += 1;
        }

        self.input.set_text(&result.to_string());
    }

    fn delete(&self) {
        let value = self.input.text();
        if value.len() > 0 {
            self.input.set_text(&value[ .. value.len() - 1]);
        } else {
            self.input.set_text("");
        }
    }

    fn clear(&self) {
        self.input.set_text("");
    }

    fn close(&self) {
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = Calculator::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
