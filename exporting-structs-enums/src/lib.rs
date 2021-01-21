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

    #[wasm_bindgen(getter)]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

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

    #[wasm_bindgen(getter)]
    pub fn get_answer(&self) -> AnswerEnum {
        self.answer
    }

    #[wasm_bindgen(setter)]
    pub fn set_answer(&mut self, value: AnswerEnum) {
        self.answer = value;
    }

    pub fn transfer_ownership(self) -> TreetoryStruct {
        self
    }
}