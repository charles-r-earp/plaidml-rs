use cpp::*;

cpp!{{
  #include <plaidml/plaidml++.h>
  using namespace vertexai::plaidml;
}}

cpp_class!(pub unsafe struct Device as "device");
