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
//pub mod list;

/*pub mod Statics {*/
    /*use super::integer::Integer;*/
    /*pub static True:Integer = Integer::new(1);*/
    /*pub static False:Integer = Integer::new(0);*/
/*}*/

use object::Object;
#[inline]
fn cast_raw<T: Object>(_rhs: &dyn Object) -> *const T {
    _rhs as *const _ as *const T
}

#[inline]
fn cast_mut<T: Object>(_rhs: &mut dyn Object) -> *mut T {
    _rhs as *mut _ as *mut T
}

#[inline]
fn as_ref<'a, T: Object>(_rhs: &'a dyn Object) -> &'a T {
    let _rhs_p = cast_raw::<T>(_rhs);
    unsafe {
        &(*_rhs_p)
    }
}

#[inline]
fn as_mut<'a, T: Object>(_rhs: &'a mut dyn Object) -> &'a mut T {
    let _rhs_p = cast_mut::<T>(_rhs);
    unsafe {
        &mut(*_rhs_p)
    }
}

#[inline]
pub fn cast_box<T, U: ?Sized>(_rhs: Box<U>) -> Box<T> {
    unsafe {
        Box::from_raw(Box::into_raw(_rhs) as *mut _ as *mut T)
    }
}

#[inline]
pub fn as_box_ref<'a, T>(_rhs: &'a *mut T) -> &'a Box<T> {
    let p = _rhs.clone();
    unsafe {
        &Box::from_raw(p)
    }
}

#[inline]
pub fn as_box_clone<T>(_rhs: & *mut T) -> Box<T> {
    unsafe {
        Box::new((*_rhs).clone())
    }
}

#[inline]
pub fn box_clone_ptr<T>(_rhs: &Box<T>) -> *mut T {
    let temp = unsafe {
        let p = _rhs as *const _;
        (*p).clone()
    };
    Box::into_raw(temp)
}
