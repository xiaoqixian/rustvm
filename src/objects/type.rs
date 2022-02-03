/**********************************************
  > File Name		: type.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Tue 01 Feb 2022 04:44:50 PM CST
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::rc::Rc;
use std::any::Any;

use super::{Object, object::Object as ObjectTrait, klass::KlassContainer, string::Str, object::NewObject};

/**
 * A TypeObject instance represents a class.
 * Builtin types like str and int are stored in a static variable.
 * User-defined classes are stored in `names`.
 */
#[derive(Clone)]
pub struct TypeObject {
    kc: Rc<KlassContainer>
}

impl TypeObject {
    pub fn new(attrs: &Object, name: &Object) -> Rc<Self> {
        Rc::new(Self {
            kc: KlassContainer::new(attrs, name)
        })
    }

    pub fn allocate_instance(&self) -> Object {
        NewObject::new(self.kc.clone())
    }
}

impl ObjectTrait for TypeObject {
    fn as_any(&self) -> &dyn Any {self}

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn klass(&self) -> super::klass::Klass {
        super::klass::Klass::TypeKlass
    }
}

impl std::fmt::Display for TypeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<type {}>", &self.kc.name)
    }
}

impl std::fmt::Debug for TypeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<type {}>", &self.kc.name)
    }
}
