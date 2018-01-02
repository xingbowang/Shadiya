use std::sync::RwLock;
use super::page::leaf_page;

struct rw_lock {
    lock : RwLock<leaf_page>
}

impl rw_lock {

}
