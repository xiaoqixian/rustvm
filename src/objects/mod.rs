/**********************************************
  > File Name		: object/mod.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue Nov  2 15:48:20 2021
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

pub mod object;
pub mod integer;
pub mod string;
pub mod frame;

pub mod statics {
    use super::object::{*};

    const PY_NONE_INSTANCE: PyNone = PyNone {};
    pub const PY_NONE: *mut dyn Object = {
        &PY_NONE_INSTANCE as *const PyNone as *mut PyNone as *mut dyn Object 
    };
    
    const TRUE_INSTANCE: PyTrue = PyTrue {val: 1};
    pub const TRUE:*mut dyn Object = {
        &TRUE_INSTANCE as *const PyTrue as *mut PyTrue as *mut dyn Object
    };

    const FALSE_INSTANCE: PyFalse = PyFalse {val:0};
    pub const FALSE:*mut dyn Object = {
        &FALSE_INSTANCE as *const PyFalse as *mut PyFalse as *mut dyn Object
    };
}

/*use object::Object;*/
/*#[inline]*/
/*fn cast_raw<T: Object>(_rhs: &dyn Object) -> *const T {*/
    /*_rhs as *const _ as *const T*/
/*}*/

/*#[inline]*/
/*fn cast_mut<T: Object>(_rhs: &mut dyn Object) -> *mut T {*/
    /*_rhs as *mut _ as *mut T*/
/*}*/

/*#[inline]*/
/*fn as_ref<T: Object+Sized>(_rhs: *const dyn Object) -> &'static T {*/
    /*unsafe {*/
        /*&(*(_rhs as *const T))*/
    /*}*/
/*}*/

/*#[inline]*/
/*fn as_mut<'a, T: Object>(_rhs: &'a mut dyn Object) -> &'a mut T {*/
    /*let _rhs_p = cast_mut::<T>(_rhs);*/
    /*unsafe {*/
        /*&mut(*_rhs_p)*/
    /*}*/
/*}*/

/*#[inline]*/
/*pub fn cast_box<T, U: ?Sized>(_rhs: Box<U>) -> Box<T> {*/
    /*unsafe {*/
        /*Box::from_raw(Box::into_raw(_rhs) as *mut _ as *mut T)*/
    /*}*/
/*}*/

/*#[inline]*/
/*pub fn as_box_ref<'a, T: ?Sized>(_rhs: &'a *mut T) -> &'a Box<T> {*/
    /*let p = _rhs.clone();*/
    /*unsafe {*/
        /*&Box::from_raw(p)*/
    /*}*/
/*}*/

/*[>#[inline]<]*/
/*[>pub fn as_box_clone<T>(_rhs: & *mut T) -> Box<T> {<]*/
    /*[>unsafe {<]*/
        /*[>Box::new((*_rhs).clone())<]*/
    /*[>}<]*/
/*[>}<]*/

/*#[inline]*/
/*pub fn box_clone_ptr<T>(_rhs: &Box<T>) -> *mut T {*/
    /*let temp: Box<T> = _rhs.clone();*/
    /*Box::into_raw(temp)*/
/*}*/
