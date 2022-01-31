/**********************************************
  > File Name		: objects/map.rs
  > Author		    : lunar
  > Email			: lunar_ubuntu@qq.com
  > Created Time	: Sun Jan 30 22:48:44 2022
  > Location        : Shanghai
  > Copyright@ https://github.com/xiaoqixian
 **********************************************/

use std::fmt;

use super::{Object, object::Object as ObjectTrait};

#[derive(Debug, Clone)]
pub struct MapEntry {
    k: Object,
    v: Object
}

impl MapEntry {
    fn new(k: Object, v: Object) -> Self {
        Self {k,v}
    }
}

#[derive(Debug, Clone)]
pub struct Dict {
    entries: Vec<Option<MapEntry>>
}

impl ObjectTrait for Dict {
    fn as_any(&self) -> &dyn std::any::Any {self}

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {self}

    fn klass(&self) -> super::klass::Klass {
        super::klass::Klass::DictKlass
    }
}

impl fmt::Display for Dict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::from("{");
        let len = self.entries.len();
        let mut flag = false;

        for i in 0..len {
            if self.entries[i].is_none() {
                continue;
            }
            let entry = self.entries[i].as_ref().unwrap();
            if flag {
                s.push_str(format!(", {}: {}", entry.k, entry.v).as_str());
            } else {
                s.push_str(format!("{}: {}", entry.k, entry.v).as_str());
                flag = true;
            }
        }
        s.push_str("}");
        write!(f, "{}", s)
    }
}

impl Dict {
    pub fn new() -> Object {
        std::rc::Rc::new(Self {
            entries: Vec::new()
        })
    }

    fn get_entry<'a>(&'a self, k: &Object) -> Option<&'a MapEntry> {
        for entry in self.entries.iter() {
            if let Some(e) = entry {
                if k.equal(&e.k) {
                    return Some(e);
                }
            }
        }
        None
    }

    /**
     * Find a mutable entry by a key.
     * If not found, return the first empty slot.
     */
    fn get_mut_entry<'a>(&'a mut self, k: &Object) -> (Option<&'a mut MapEntry>, Option<usize>) {
        let mut slot: Option<usize> = None;
        let mut index: Option<usize> = None;

        for i in 0..self.entries.len() {
            match &self.entries[i] {
                None => {
                    if slot.is_none() {
                        slot = Some(i);
                    }
                },
                Some(ref e) => {
                    if k.equal(&e.k) {
                        index = Some(i);
                        break;
                    }
                }
            }
        }

        match index {
            Some(i) => (self.entries[i].as_mut(), None),
            None => (None, slot)
        }
    }

    pub fn get(&self, k: &Object) -> Option<Object> {
        match self.get_entry(k) {
            None => None,
            Some(e) => Some(e.v.clone())
        }
    }

    pub fn put(&mut self, k: Object, v: Object) {
        match self.get_mut_entry(&k) {
            (None, slot) => {
                match slot {
                    Some(i) => self.entries[i] = Some(MapEntry::new(k, v)),
                    None => self.entries.push(Some(MapEntry::new(k, v)))
                }
            },
            (Some(e), _) => e.v = v
        }
    }

    /**
     * remove an entry from the map and return it as wrapped in an Option so 
     * user knows if the remove operation successes.
     */
    pub fn remove(&mut self, k: Object) -> Option<MapEntry> {
        let mut index: Option<usize> = None;
        for i in 0..self.entries.len() {
            if let Some(ref e) = self.entries[i] {
                if k.equal(&e.k) {
                    index = Some(i);
                    break;
                }
            }
        }

        match index {
            None => None,
            Some(i) => {
                Some(std::mem::replace(&mut self.entries[i], None).unwrap())
            }
        }
    }
}
