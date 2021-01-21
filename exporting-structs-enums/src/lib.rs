mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// When it comes to Enums:
// - They  are C styled.
// - JS represents them through an object with a number for each variant.
// - enum 타입의 반환값을 JS 로 전달할 수 있기 위해선 Copy, Clone trait 을 derive 해줘야만 한다. 이건 ownership 때문이다.
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum AnswerEnum {
    Yes,
    No,
}

#[wasm_bindgen]
pub fn verify_enum_choice(choice: AnswerEnum) -> bool {
    match choice {
        AnswerEnum::Yes => true,
        AnswerEnum::No => false,
    }
}

// When it comes to Structs:
// - Cannot contain lifetimes or type parameters.
// - Each field value must impl the Copy trait.
// - String, enum 타입을 struct 의 변수로 선언할 때, private 으로 선언해야만 했다. 이를 getter, setter 를 이용해서 외부에서 접근하도록 한다.
#[wasm_bindgen]
pub struct TreetoryStruct {
    pub age: i32,
    name: String,
    email: String,
    answer: AnswerEnum,
}

// For struct impl, we have the option for struct methods and type-level functions.
// JS handles structs by creating a JS object with a pointer (i.o.w. we can use references!).
#[wasm_bindgen]
impl TreetoryStruct {
    pub fn new(age: i32, name: String, email: String, answer: AnswerEnum) -> TreetoryStruct {
        TreetoryStruct { age, name, email, answer }
    }

    pub fn get_age(&self) -> i32 {
        self.age
    }

    pub fn set_age(&mut self, value: i32) {
        self.age = value;
    }
    // String 타입의 getter 는 이렇게 바인딩 해줘야만 했다.
    #[wasm_bindgen(getter)]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    // String 타입의 setter 도 이렇게 바인딩 해줘야만 했다.
    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, value: String) {
        self.name = value;
    }

    #[wasm_bindgen(getter)]
    pub fn get_email(&self) -> String {
        self.email.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_email(&mut self, value: String) {
        self.email = value;
    }
    // enum 타입의 getter 도 이렇게 바인딩 해줘야만 했다.
    #[wasm_bindgen(getter)]
    pub fn get_answer(&self) -> AnswerEnum {
        self.answer
    }
    // enum 타입의 setter 도 이렇게 바인딩 해줘야만 했다.
    #[wasm_bindgen(setter)]
    pub fn set_answer(&mut self, value: AnswerEnum) {
        self.answer = value;
    }

    pub fn transfer_ownership(self) -> TreetoryStruct {
        self
    }
}

// 지금 이코드를 빌드할 때는 필수적으로 wasm-pack build --dev 명령을 사용해야만 했다.
// --dev 옵션을 붙이지 않으면 validator 가 문제를 일으켰다.
// 관련된 내용은 여기서 확인할 수 있었다.
// https://github.com/rustwasm/wasm-pack/issues/886
