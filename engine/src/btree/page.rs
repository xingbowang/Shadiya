/**
 * This file include definitions of both internal and leaf pages.
 * Internal pages stores links to leaf pages. Internal page access is lockless, it is copy on write.
 * Leaf pages contains user data directly. Leaf page access is locked. It is modified in place.
 */

pub enum PageType {
    INTERNAL,
    LEAF
}

pub struct internal_page {
    children_page_type : PageType,
    children_pages : Vec<i32>
}

pub struct leaf_page {
    data : Vec<i32>
}
