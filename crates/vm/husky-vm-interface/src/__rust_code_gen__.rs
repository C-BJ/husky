// this is generated by husky_vm_interface_code_gen::rust_code::write_rust_code
// do not modify by hand

use crate::*;

type void = ();
type b32 = u32;
type b64 = u64;

// void
#[no_mangle]
pub unsafe extern "C" fn __void_primitive_value_to_bool(data: __RegisterData) -> bool {
    let data = data.as_void;
    false
}

#[no_mangle]
pub unsafe extern "C" fn __void_primitive_value_to_box(data: __RegisterData) -> *mut () {
    let data = data.as_void;
    let ptr: *mut void = Box::<void>::into_raw(Box::new(data));
    ptr as *mut ()
}

#[no_mangle]
pub unsafe extern "C" fn __void_clone(data: *mut ()) -> *mut () {
    Box::<void>::into_raw(Box::new((*(data as *mut void)).clone())) as *mut ()
}

#[no_mangle]
pub unsafe extern "C" fn __void_drop(data: *mut ()) {
    Box::from_raw(data as *mut void);
}

#[no_mangle]
pub unsafe extern "C" fn __void_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const void) == *(other as *const () as *const void)
}

#[no_mangle]
pub unsafe extern "C" fn __void_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<void>(&__VOID_VTABLE) = registers[1].downcast_void()
}

#[no_mangle]
pub static __VOID_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: Some(__void_primitive_value_to_bool),
    primitive_value_to_box: Some(__void_primitive_value_to_box),
    clone: __void_clone,
    drop: __void_drop,
    eq: __void_eq,
    assign: __void_assign,
    typename_str: "void",
    typename_str_hash_u64: 8073556201194512886,
};

impl<'eval> __Register<'eval> {
    pub fn downcast_void(&self) -> void {
        unsafe {
            if self.vtable.typename_str_hash_u64 != 8073556201194512886 {
                panic!("expect `void` but get {} instead", self.vtable.typename_str)
            }
            match self.data_kind {
                __RegisterDataKind::PrimitiveValue => self.data.as_void,
                __RegisterDataKind::EvalRef
                | __RegisterDataKind::TempRef
                | __RegisterDataKind::TempMut
                | __RegisterDataKind::Box => *(self.data.as_ptr as *const void),
                _ => panic!(),
            }
        }
    }

    pub fn downcast_opt_void(&self) -> Option<void> {
        unsafe {
            if self.vtable.typename_str_hash_u64 != 8073556201194512886 {
                panic!("expect `void` but get `{}` instead", self.vtable.typename_str)
            }
            match self.data_kind {
                __RegisterDataKind::PrimitiveValue => Some(self.data.as_void),
                __RegisterDataKind::EvalRef
                | __RegisterDataKind::TempRef
                | __RegisterDataKind::TempMut
                | __RegisterDataKind::Box => Some(*(self.data.as_ptr as *const void)),
                __RegisterDataKind::Undefined => None,
                _ => panic!(),
            }
        }
    }
}

// bool
#[no_mangle]
pub unsafe extern "C" fn __bool_primitive_value_to_bool(data: __RegisterData) -> bool {
    let data = data.as_bool;
    data
}

#[no_mangle]
pub unsafe extern "C" fn __bool_primitive_value_to_box(data: __RegisterData) -> *mut () {
    let data = data.as_bool;
    let ptr: *mut bool = Box::<bool>::into_raw(Box::new(data));
    ptr as *mut ()
}

#[no_mangle]
pub unsafe extern "C" fn __bool_clone(data: *mut ()) -> *mut () {
    Box::<bool>::into_raw(Box::new((*(data as *mut bool)).clone())) as *mut ()
}

#[no_mangle]
pub unsafe extern "C" fn __bool_drop(data: *mut ()) {
    Box::from_raw(data as *mut bool);
}

#[no_mangle]
pub unsafe extern "C" fn __bool_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const bool) == *(other as *const () as *const bool)
}

#[no_mangle]
pub unsafe extern "C" fn __bool_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<bool>(&__BOOL_VTABLE) = registers[1].downcast_bool()
}

#[no_mangle]
pub static __BOOL_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: Some(__bool_primitive_value_to_bool),
    primitive_value_to_box: Some(__bool_primitive_value_to_box),
    clone: __bool_clone,
    drop: __bool_drop,
    eq: __bool_eq,
    assign: __bool_assign,
    typename_str: "bool",
    typename_str_hash_u64: 729807561129781588,
};

impl<'eval> __Register<'eval> {
    pub fn downcast_bool(&self) -> bool {
        unsafe {
            if self.vtable.typename_str_hash_u64 != 729807561129781588 {
                panic!("expect `bool` but get {} instead", self.vtable.typename_str)
            }
            match self.data_kind {
                __RegisterDataKind::PrimitiveValue => self.data.as_bool,
                __RegisterDataKind::EvalRef
                | __RegisterDataKind::TempRef
                | __RegisterDataKind::TempMut
                | __RegisterDataKind::Box => *(self.data.as_ptr as *const bool),
                _ => panic!(),
            }
        }
    }

    pub fn downcast_opt_bool(&self) -> Option<bool> {
        unsafe {
            if self.vtable.typename_str_hash_u64 != 729807561129781588 {
                panic!("expect `bool` but get `{}` instead", self.vtable.typename_str)
            }
            match self.data_kind {
                __RegisterDataKind::PrimitiveValue => Some(self.data.as_bool),
                __RegisterDataKind::EvalRef
                | __RegisterDataKind::TempRef
                | __RegisterDataKind::TempMut
                | __RegisterDataKind::Box => Some(*(self.data.as_ptr as *const bool)),
                __RegisterDataKind::Undefined => None,
                _ => panic!(),
            }
        }
    }
}

// i32
#[no_mangle]
pub unsafe extern "C" fn __i32_primitive_value_to_bool(data: __RegisterData) -> bool {
    let data = data.as_i32;
    data != 0
}

#[no_mangle]
pub unsafe extern "C" fn __i32_primitive_value_to_box(data: __RegisterData) -> *mut () {
    let data = data.as_i32;
    let ptr: *mut i32 = Box::<i32>::into_raw(Box::new(data));
    ptr as *mut ()
}

#[no_mangle]
pub unsafe extern "C" fn __i32_clone(data: *mut ()) -> *mut () {
    Box::<i32>::into_raw(Box::new((*(data as *mut i32)).clone())) as *mut ()
}

#[no_mangle]
pub unsafe extern "C" fn __i32_drop(data: *mut ()) {
    Box::from_raw(data as *mut i32);
}

#[no_mangle]
pub unsafe extern "C" fn __i32_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const i32) == *(other as *const () as *const i32)
}

#[no_mangle]
pub unsafe extern "C" fn __i32_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<i32>(&__I32_VTABLE) = registers[1].downcast_i32()
}

#[no_mangle]
pub static __I32_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: Some(__i32_primitive_value_to_bool),
    primitive_value_to_box: Some(__i32_primitive_value_to_box),
    clone: __i32_clone,
    drop: __i32_drop,
    eq: __i32_eq,
    assign: __i32_assign,
    typename_str: "i32",
    typename_str_hash_u64: 6639413044669031007,
};

impl<'eval> __Register<'eval> {
    pub fn downcast_i32(&self) -> i32 {
        unsafe {
            if self.vtable.typename_str_hash_u64 != 6639413044669031007 {
                panic!("expect `i32` but get {} instead", self.vtable.typename_str)
            }
            match self.data_kind {
                __RegisterDataKind::PrimitiveValue => self.data.as_i32,
                __RegisterDataKind::EvalRef
                | __RegisterDataKind::TempRef
                | __RegisterDataKind::TempMut
                | __RegisterDataKind::Box => *(self.data.as_ptr as *const i32),
                _ => panic!(),
            }
        }
    }

    pub fn downcast_opt_i32(&self) -> Option<i32> {
        unsafe {
            if self.vtable.typename_str_hash_u64 != 6639413044669031007 {
                panic!("expect `i32` but get `{}` instead", self.vtable.typename_str)
            }
            match self.data_kind {
                __RegisterDataKind::PrimitiveValue => Some(self.data.as_i32),
                __RegisterDataKind::EvalRef
                | __RegisterDataKind::TempRef
                | __RegisterDataKind::TempMut
                | __RegisterDataKind::Box => Some(*(self.data.as_ptr as *const i32)),
                __RegisterDataKind::Undefined => None,
                _ => panic!(),
            }
        }
    }
}

// i64
#[no_mangle]
pub unsafe extern "C" fn __i64_primitive_value_to_bool(data: __RegisterData) -> bool {
    let data = data.as_i64;
    data != 0
}

#[no_mangle]
pub unsafe extern "C" fn __i64_primitive_value_to_box(data: __RegisterData) -> *mut () {
    let data = data.as_i64;
    let ptr: *mut i64 = Box::<i64>::into_raw(Box::new(data));
    ptr as *mut ()
}

#[no_mangle]
pub unsafe extern "C" fn __i64_clone(data: *mut ()) -> *mut () {
    Box::<i64>::into_raw(Box::new((*(data as *mut i64)).clone())) as *mut ()
}

#[no_mangle]
pub unsafe extern "C" fn __i64_drop(data: *mut ()) {
    Box::from_raw(data as *mut i64);
}

#[no_mangle]
pub unsafe extern "C" fn __i64_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const i64) == *(other as *const () as *const i64)
}

#[no_mangle]
pub unsafe extern "C" fn __i64_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<i64>(&__I64_VTABLE) = registers[1].downcast_i64()
}

#[no_mangle]
pub static __I64_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: Some(__i64_primitive_value_to_bool),
    primitive_value_to_box: Some(__i64_primitive_value_to_box),
    clone: __i64_clone,
    drop: __i64_drop,
    eq: __i64_eq,
    assign: __i64_assign,
    typename_str: "i64",
    typename_str_hash_u64: 9204872793588273300,
};

impl<'eval> __Register<'eval> {
    pub fn downcast_i64(&self) -> i64 {
        unsafe {
            if self.vtable.typename_str_hash_u64 != 9204872793588273300 {
                panic!("expect `i64` but get {} instead", self.vtable.typename_str)
            }
            match self.data_kind {
                __RegisterDataKind::PrimitiveValue => self.data.as_i64,
                __RegisterDataKind::EvalRef
                | __RegisterDataKind::TempRef
                | __RegisterDataKind::TempMut
                | __RegisterDataKind::Box => *(self.data.as_ptr as *const i64),
                _ => panic!(),
            }
        }
    }

    pub fn downcast_opt_i64(&self) -> Option<i64> {
        unsafe {
            if self.vtable.typename_str_hash_u64 != 9204872793588273300 {
                panic!("expect `i64` but get `{}` instead", self.vtable.typename_str)
            }
            match self.data_kind {
                __RegisterDataKind::PrimitiveValue => Some(self.data.as_i64),
                __RegisterDataKind::EvalRef
                | __RegisterDataKind::TempRef
                | __RegisterDataKind::TempMut
                | __RegisterDataKind::Box => Some(*(self.data.as_ptr as *const i64)),
                __RegisterDataKind::Undefined => None,
                _ => panic!(),
            }
        }
    }
}

// b32
#[no_mangle]
pub unsafe extern "C" fn __b32_primitive_value_to_bool(data: __RegisterData) -> bool {
    let data = data.as_b32;
    data != 0
}

#[no_mangle]
pub unsafe extern "C" fn __b32_primitive_value_to_box(data: __RegisterData) -> *mut () {
    let data = data.as_b32;
    let ptr: *mut b32 = Box::<b32>::into_raw(Box::new(data));
    ptr as *mut ()
}

#[no_mangle]
pub unsafe extern "C" fn __b32_clone(data: *mut ()) -> *mut () {
    Box::<b32>::into_raw(Box::new((*(data as *mut b32)).clone())) as *mut ()
}

#[no_mangle]
pub unsafe extern "C" fn __b32_drop(data: *mut ()) {
    Box::from_raw(data as *mut b32);
}

#[no_mangle]
pub unsafe extern "C" fn __b32_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const b32) == *(other as *const () as *const b32)
}

#[no_mangle]
pub unsafe extern "C" fn __b32_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<b32>(&__B32_VTABLE) = registers[1].downcast_b32()
}

#[no_mangle]
pub static __B32_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: Some(__b32_primitive_value_to_bool),
    primitive_value_to_box: Some(__b32_primitive_value_to_box),
    clone: __b32_clone,
    drop: __b32_drop,
    eq: __b32_eq,
    assign: __b32_assign,
    typename_str: "b32",
    typename_str_hash_u64: 9758498138566595375,
};

impl<'eval> __Register<'eval> {
    pub fn downcast_b32(&self) -> b32 {
        unsafe {
            if self.vtable.typename_str_hash_u64 != 9758498138566595375 {
                panic!("expect `b32` but get {} instead", self.vtable.typename_str)
            }
            match self.data_kind {
                __RegisterDataKind::PrimitiveValue => self.data.as_b32,
                __RegisterDataKind::EvalRef
                | __RegisterDataKind::TempRef
                | __RegisterDataKind::TempMut
                | __RegisterDataKind::Box => *(self.data.as_ptr as *const b32),
                _ => panic!(),
            }
        }
    }

    pub fn downcast_opt_b32(&self) -> Option<b32> {
        unsafe {
            if self.vtable.typename_str_hash_u64 != 9758498138566595375 {
                panic!("expect `b32` but get `{}` instead", self.vtable.typename_str)
            }
            match self.data_kind {
                __RegisterDataKind::PrimitiveValue => Some(self.data.as_b32),
                __RegisterDataKind::EvalRef
                | __RegisterDataKind::TempRef
                | __RegisterDataKind::TempMut
                | __RegisterDataKind::Box => Some(*(self.data.as_ptr as *const b32)),
                __RegisterDataKind::Undefined => None,
                _ => panic!(),
            }
        }
    }
}

// b64
#[no_mangle]
pub unsafe extern "C" fn __b64_primitive_value_to_bool(data: __RegisterData) -> bool {
    let data = data.as_b64;
    data != 0
}

#[no_mangle]
pub unsafe extern "C" fn __b64_primitive_value_to_box(data: __RegisterData) -> *mut () {
    let data = data.as_b64;
    let ptr: *mut b64 = Box::<b64>::into_raw(Box::new(data));
    ptr as *mut ()
}

#[no_mangle]
pub unsafe extern "C" fn __b64_clone(data: *mut ()) -> *mut () {
    Box::<b64>::into_raw(Box::new((*(data as *mut b64)).clone())) as *mut ()
}

#[no_mangle]
pub unsafe extern "C" fn __b64_drop(data: *mut ()) {
    Box::from_raw(data as *mut b64);
}

#[no_mangle]
pub unsafe extern "C" fn __b64_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const b64) == *(other as *const () as *const b64)
}

#[no_mangle]
pub unsafe extern "C" fn __b64_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<b64>(&__B64_VTABLE) = registers[1].downcast_b64()
}

#[no_mangle]
pub static __B64_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: Some(__b64_primitive_value_to_bool),
    primitive_value_to_box: Some(__b64_primitive_value_to_box),
    clone: __b64_clone,
    drop: __b64_drop,
    eq: __b64_eq,
    assign: __b64_assign,
    typename_str: "b64",
    typename_str_hash_u64: 11108470303398574121,
};

impl<'eval> __Register<'eval> {
    pub fn downcast_b64(&self) -> b64 {
        unsafe {
            if self.vtable.typename_str_hash_u64 != 11108470303398574121 {
                panic!("expect `b64` but get {} instead", self.vtable.typename_str)
            }
            match self.data_kind {
                __RegisterDataKind::PrimitiveValue => self.data.as_b64,
                __RegisterDataKind::EvalRef
                | __RegisterDataKind::TempRef
                | __RegisterDataKind::TempMut
                | __RegisterDataKind::Box => *(self.data.as_ptr as *const b64),
                _ => panic!(),
            }
        }
    }

    pub fn downcast_opt_b64(&self) -> Option<b64> {
        unsafe {
            if self.vtable.typename_str_hash_u64 != 11108470303398574121 {
                panic!("expect `b64` but get `{}` instead", self.vtable.typename_str)
            }
            match self.data_kind {
                __RegisterDataKind::PrimitiveValue => Some(self.data.as_b64),
                __RegisterDataKind::EvalRef
                | __RegisterDataKind::TempRef
                | __RegisterDataKind::TempMut
                | __RegisterDataKind::Box => Some(*(self.data.as_ptr as *const b64)),
                __RegisterDataKind::Undefined => None,
                _ => panic!(),
            }
        }
    }
}

// f32
#[no_mangle]
pub unsafe extern "C" fn __f32_primitive_value_to_bool(data: __RegisterData) -> bool {
    let data = data.as_f32;
    data != 0.0
}

#[no_mangle]
pub unsafe extern "C" fn __f32_primitive_value_to_box(data: __RegisterData) -> *mut () {
    let data = data.as_f32;
    let ptr: *mut f32 = Box::<f32>::into_raw(Box::new(data));
    ptr as *mut ()
}

#[no_mangle]
pub unsafe extern "C" fn __f32_clone(data: *mut ()) -> *mut () {
    Box::<f32>::into_raw(Box::new((*(data as *mut f32)).clone())) as *mut ()
}

#[no_mangle]
pub unsafe extern "C" fn __f32_drop(data: *mut ()) {
    Box::from_raw(data as *mut f32);
}

#[no_mangle]
pub unsafe extern "C" fn __f32_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const f32) == *(other as *const () as *const f32)
}

#[no_mangle]
pub unsafe extern "C" fn __f32_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<f32>(&__F32_VTABLE) = registers[1].downcast_f32()
}

#[no_mangle]
pub static __F32_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: Some(__f32_primitive_value_to_bool),
    primitive_value_to_box: Some(__f32_primitive_value_to_box),
    clone: __f32_clone,
    drop: __f32_drop,
    eq: __f32_eq,
    assign: __f32_assign,
    typename_str: "f32",
    typename_str_hash_u64: 211483071870485656,
};

impl<'eval> __Register<'eval> {
    pub fn downcast_f32(&self) -> f32 {
        unsafe {
            if self.vtable.typename_str_hash_u64 != 211483071870485656 {
                panic!("expect `f32` but get {} instead", self.vtable.typename_str)
            }
            match self.data_kind {
                __RegisterDataKind::PrimitiveValue => self.data.as_f32,
                __RegisterDataKind::EvalRef
                | __RegisterDataKind::TempRef
                | __RegisterDataKind::TempMut
                | __RegisterDataKind::Box => *(self.data.as_ptr as *const f32),
                _ => panic!(),
            }
        }
    }

    pub fn downcast_opt_f32(&self) -> Option<f32> {
        unsafe {
            if self.vtable.typename_str_hash_u64 != 211483071870485656 {
                panic!("expect `f32` but get `{}` instead", self.vtable.typename_str)
            }
            match self.data_kind {
                __RegisterDataKind::PrimitiveValue => Some(self.data.as_f32),
                __RegisterDataKind::EvalRef
                | __RegisterDataKind::TempRef
                | __RegisterDataKind::TempMut
                | __RegisterDataKind::Box => Some(*(self.data.as_ptr as *const f32)),
                __RegisterDataKind::Undefined => None,
                _ => panic!(),
            }
        }
    }
}

// f64
#[no_mangle]
pub unsafe extern "C" fn __f64_primitive_value_to_bool(data: __RegisterData) -> bool {
    let data = data.as_f64;
    data != 0.0
}

#[no_mangle]
pub unsafe extern "C" fn __f64_primitive_value_to_box(data: __RegisterData) -> *mut () {
    let data = data.as_f64;
    let ptr: *mut f64 = Box::<f64>::into_raw(Box::new(data));
    ptr as *mut ()
}

#[no_mangle]
pub unsafe extern "C" fn __f64_clone(data: *mut ()) -> *mut () {
    Box::<f64>::into_raw(Box::new((*(data as *mut f64)).clone())) as *mut ()
}

#[no_mangle]
pub unsafe extern "C" fn __f64_drop(data: *mut ()) {
    Box::from_raw(data as *mut f64);
}

#[no_mangle]
pub unsafe extern "C" fn __f64_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const f64) == *(other as *const () as *const f64)
}

#[no_mangle]
pub unsafe extern "C" fn __f64_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<f64>(&__F64_VTABLE) = registers[1].downcast_f64()
}

#[no_mangle]
pub static __F64_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: Some(__f64_primitive_value_to_bool),
    primitive_value_to_box: Some(__f64_primitive_value_to_box),
    clone: __f64_clone,
    drop: __f64_drop,
    eq: __f64_eq,
    assign: __f64_assign,
    typename_str: "f64",
    typename_str_hash_u64: 14456281901843390161,
};

impl<'eval> __Register<'eval> {
    pub fn downcast_f64(&self) -> f64 {
        unsafe {
            if self.vtable.typename_str_hash_u64 != 14456281901843390161 {
                panic!("expect `f64` but get {} instead", self.vtable.typename_str)
            }
            match self.data_kind {
                __RegisterDataKind::PrimitiveValue => self.data.as_f64,
                __RegisterDataKind::EvalRef
                | __RegisterDataKind::TempRef
                | __RegisterDataKind::TempMut
                | __RegisterDataKind::Box => *(self.data.as_ptr as *const f64),
                _ => panic!(),
            }
        }
    }

    pub fn downcast_opt_f64(&self) -> Option<f64> {
        unsafe {
            if self.vtable.typename_str_hash_u64 != 14456281901843390161 {
                panic!("expect `f64` but get `{}` instead", self.vtable.typename_str)
            }
            match self.data_kind {
                __RegisterDataKind::PrimitiveValue => Some(self.data.as_f64),
                __RegisterDataKind::EvalRef
                | __RegisterDataKind::TempRef
                | __RegisterDataKind::TempMut
                | __RegisterDataKind::Box => Some(*(self.data.as_ptr as *const f64)),
                __RegisterDataKind::Undefined => None,
                _ => panic!(),
            }
        }
    }
}

// __VirtualFunction
#[no_mangle]
pub unsafe extern "C" fn __virtual_function_clone(data: *mut ()) -> *mut () {
    Box::<__VirtualFunction>::into_raw(Box::new((*(data as *mut __VirtualFunction)).clone())) as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __virtual_function_drop(data: *mut ()) {
    Box::from_raw(data as *mut __VirtualFunction);
}
#[no_mangle]
pub unsafe extern "C" fn __virtual_function_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const __VirtualFunction) == *(other as *const () as *const __VirtualFunction)
}
#[no_mangle]
pub unsafe extern "C" fn __virtual_function_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<__VirtualFunction>(&__VIRTUAL_FUNCTION_VTABLE) = registers[1].downcast_move(&__VIRTUAL_FUNCTION_VTABLE)
}
#[no_mangle]
pub static __VIRTUAL_FUNCTION_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __virtual_function_clone,
    drop: __virtual_function_drop,
    eq: __virtual_function_eq,
    assign: __virtual_function_assign,
    typename_str_hash_u64: 14269288641304277215,
    typename_str: "__VirtualFunction",
};

// __VirtualEnum
#[no_mangle]
pub unsafe extern "C" fn __virtual_enum_clone(data: *mut ()) -> *mut () {
    Box::<__VirtualEnum>::into_raw(Box::new((*(data as *mut __VirtualEnum)).clone())) as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __virtual_enum_drop(data: *mut ()) {
    Box::from_raw(data as *mut __VirtualEnum);
}
#[no_mangle]
pub unsafe extern "C" fn __virtual_enum_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const __VirtualEnum) == *(other as *const () as *const __VirtualEnum)
}
#[no_mangle]
pub unsafe extern "C" fn __virtual_enum_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<__VirtualEnum>(&__VIRTUAL_ENUM_VTABLE) = registers[1].downcast_move(&__VIRTUAL_ENUM_VTABLE)
}
#[no_mangle]
pub static __VIRTUAL_ENUM_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __virtual_enum_clone,
    drop: __virtual_enum_drop,
    eq: __virtual_enum_eq,
    assign: __virtual_enum_assign,
    typename_str_hash_u64: 10189591299398487822,
    typename_str: "__VirtualEnum",
};
