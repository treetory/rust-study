import * as wasm from "../pkg/exporting_structs_enums";

let treetory = wasm.TreetoryStruct.new(40, '전인환', 'treetory@letsee.io', 0);

let btn_struct_get_age = document.getElementById('btn_struct_get_age');
btn_struct_get_age.addEventListener('click', (e) => {
    alert(treetory.get_age());
});

let btn_struct_get_name = document.getElementById('btn_struct_get_name');
btn_struct_get_name.addEventListener('click', (e) => {
    alert(treetory.get_name);
});

let btn_struct_get_email = document.getElementById('btn_struct_get_email');
btn_struct_get_email.addEventListener('click', (e) => {
    alert(treetory.get_email);
});