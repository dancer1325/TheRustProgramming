mod ownership;
mod borrowing;
mod lifetime;
mod lifetimeinstructimpl;
mod referencecounted;
mod atomicreferencecounted;
mod mutex;

fn main() {
    // ownership::demo();
    // borrowing::demo();
    // lifetime::demo();
    // lifetimeinstructimpl::demo();
    // referencecounted::demo();
    // atomicreferencecounted::demo();
    mutex::demo();
}
