use plaidml::*;

fn main() {
  let ref ctx = Ctx::new();
  let ref s = Shape::<f32>::dims(ctx, [1, 2]);
}
