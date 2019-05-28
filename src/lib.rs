use cpp::*;

cpp!{{
  #include <memory>
  #include <plaidml/plaidml++.h>
  using namespace vertexai::plaidml;
  using std::unique_ptr;
}}

cpp_class!(pub unsafe struct Tensor as "unique_ptr<base_tensor>");

cpp_class!(pub unsafe struct Placeholder as "placeholder");

cpp_class!(pub unsafe struct Variable as "variable");

cpp_class!(pub unsafe struct Application as "application");

cpp_class!(pub unsafe struct Function as "function");

cpp_class!(pub unsafe struct Gradient as "gradient");

cpp_class!(pub unsafe struct Device as "device");


