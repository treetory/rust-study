import * as wasm from "../pkg/exporting_common_type";

let flag = false;
const changeFlag = () => {
    flag = (flag === false ? true : false);
    return flag;
}

let btn_char_example = document.getElementById('btn_char_example');
btn_char_example.addEventListener('click', (e) => {
    console.warn(wasm.char_example('a'));
});

let btn_string_examples = document.getElementById('btn_string_examples');
btn_string_examples.addEventListener('click', (e) => {
    console.warn(wasm.string_examples('treetory'));
});

let btn_number_example = document.getElementById('btn_number_example');
btn_number_example.addEventListener('click', (e) => {
    console.warn(wasm.number_example(100));
});

let btn_bool_example = document.getElementById('btn_bool_example');
btn_bool_example.addEventListener('click', (e) => {
    console.warn(wasm.bool_example(changeFlag()));
});

let btn_mixed_array_example = document.getElementById('btn_mixed_array_example');
btn_mixed_array_example.addEventListener('click', (e) => {
    console.warn(wasm.mixed_array_example([31,32,33,34]));
});

let btn_typed_array_example = document.getElementById('btn_typed_array_example');
btn_typed_array_example.addEventListener('click', (e) => {
    console.warn(wasm.typed_array_example([45,76,23,88]));
});

let btn_option_example = document.getElementById('btn_option_example');
btn_option_example.addEventListener('click', (e) => {
    console.warn(wasm.option_example(changeFlag()));
});

let btn_result_example = document.getElementById('btn_result_example');
btn_result_example.addEventListener('click', (e) => {
    console.warn(wasm.result_example(changeFlag()));
});