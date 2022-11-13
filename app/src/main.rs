use engine;
use pollster::FutureExt;

fn main() {
    engine::run().block_on();
}
