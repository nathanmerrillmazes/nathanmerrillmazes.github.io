extern crate wee_alloc;

mod maze;
mod start;
mod tilings;
mod generators;


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

