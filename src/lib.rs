use std::marker::PhantomData;
use cpp::*;

cpp!{{
  #include <plaidml/plaidml++.h>
  using vertexai::ctx;
  using namespace vertexai::plaidml;
}}

cpp_class!(unsafe struct Ctx as "std::shared_ptr<ctx>");

impl  Ctx {
  // note: Ctx::default() does not use std::make_shared
  fn new() -> Self {
    cpp!(unsafe [] -> Ctx as "std::shared_ptr<ctx>" {
      return std::make_shared<ctx>();
    })
  }
}

cpp_class!(unsafe struct BaseShape as "base_shape");

pub struct Shape<T> {
  base_shape: BaseShape,
  _marker: PhantomData<T>
}

/*impl<T> Shape<T> {
  fn new<D>(dims: D) -> Self
    where D: AsRef<[usize]> {
    cpp!(unsafe [*/

cpp_class!(unsafe struct BaseTensor as "std::unique_ptr<base_tensor>");

pub struct Tensor<T> {
  base_tensor: BaseTensor,
  _marker: PhantomData<T>
}

cpp_class!(pub unsafe struct Placeholder as "placeholder");

cpp_class!(pub unsafe struct Variable as "variable");

cpp_class!(pub unsafe struct Application as "application");

cpp_class!(pub unsafe struct Function as "function");

cpp_class!(pub unsafe struct Gradient as "gradient");

cpp_class!(pub unsafe struct Device as "device");


