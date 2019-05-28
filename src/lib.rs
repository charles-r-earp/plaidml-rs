use std::marker::PhantomData;
use cpp::*;

cpp!{{
  #include <plaidml/plaidml++.h>
  using namespace vertexai::plaidml;
}}

//cpp_class!(pub unsafe struct Shape as "shape");

cpp_class!(pub unsafe struct BaseTensor as "std::unique_ptr<base_tensor>");

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


