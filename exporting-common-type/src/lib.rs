mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn char_example(_c: char)  -> char {
    // 캐릭터 값을 받아서 새로운 캐릭터로 반환
    '🚀'
}

#[wasm_bindgen]
pub fn string_examples(s: String) -> String {
    // 스트링 값을 받아서 앞에 Hello 를 붙여서 반환
    format!("Hello {}", s)
}

#[wasm_bindgen]
pub fn number_example(n: i32) -> i32 {
    // number 를 받아서 100을 더해서 반환
    n+100
}

#[wasm_bindgen]
pub fn bool_example(_b: bool) -> bool {

    let mut result: bool = false;

    if _b == true {
        result = false;
    } else {
        result = true;
    }

    result
}

// `Box<[JsValue]>` are the representation for a JS array object.
// When it comes to Js Arrays:
// - They are iterable.
// - Can contain multiple types by being of type JsValue (strictly typed arrays exist for numbers).
// - Don't really support N-dimensional arrays and are expensive to work with.
#[wasm_bindgen]
pub fn mixed_array_example(array: Box<[JsValue]>) -> Box<[JsValue]>{

    // 0. JsValue 는 웹의 Object 타입에 대응되는 것으로 보임.
    // 1. mutable vector 를 선언 (Generics 는 JsValue)
    let mut values = vec![
        "Hello".into(),
        512.into(),
        JsValue::NULL,
        JsValue::UNDEFINED,
        61.20.into()
    ];

    // 2. 인자로 받은 array (JsValue의 집합 모음인 array 를 Box monad 에 담아서 인자로 받은 것) 을 순회하면서,
    for value in array.iter() {
        values.push(value.clone());
    }

    // 3. 선언한 values [rust 안에서 만든 값들로 이루어진 객체] 에 JS 에서 넘겨준 array 를 붙인 것을 Box 에 담아서 반환
    values.into_boxed_slice()
}

// Typed arrays are only available for number types.
// For example, the function below will return a JS Int32Array type.
#[wasm_bindgen]
pub fn typed_array_example(_array: Box<[i32]>) -> Box<[i32]> {

    // 1. mutable vector 를 선언
    let mut values = vec![1, 2, 3, 4, 5, 6, 7];

    // 2. 인자로 받은 array of numbers 에서 선언한 vector 에 없는 number 가 있으면 추가
    for n in _array.iter() {
        if !values.contains(n) {
            values.push(n.clone());
        }
    }

    // 3. 2번째 스텝의 연산이 완료된 결과를 반환
    values.into_boxed_slice()
}

// When it comes to Option:
// - Some returns the value inside.
// - None returns a JS undefined.
#[wasm_bindgen(catch)]
pub fn option_example(_b: bool) -> Option<i32> {
    // 인자값 (true/false) 에 따라 Option 을 달리 반환
    if _b == true {
        None
    } else {
        Some(100)
    }
}

// When it comes to Result
// - Result<T, JsValue> is the only supported signature. T must be convertible to a JsValue.
// - #[wasm_bindgen(catch)] must be used when returning a result.
// - Err will be equivalent to a JS thrown error.
// - Ok will return the value inside.
#[wasm_bindgen]
pub fn result_example(_b: bool) -> Result<String, JsValue> {

    let result = match _b {
        true => {Ok(format!("Parameter is {}", _b).into())}
        false  => {Err("Look Pa, I'm throwing a JS error!".into())}
    };

    result
}