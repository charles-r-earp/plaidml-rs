use std::marker::PhantomData;
use cpp::*;

cpp!{{
  #include <plaidml/plaidml++.h>
  using vertexai::ctx;
  using namespace vertexai::plaidml;
  
  template<typename T>
  struct slice {
    T* const ptr;
    std::size_t const len;
    T* begin() const { return ptr; }
    T* end() const { return ptr + len; } 
  };
  
  /*template<typename T>
  struct mut_slice {
    const T* ptr,
    const std::size_t len
  };*/
}}

pub unsafe trait Data {
  // plaidml_datatype
  fn data() -> usize;
}

macro_rules! impl_data {
  ($t:ty => $dt:expr) => {
    unsafe impl Data for $t {
      #[inline]
      fn data() -> usize {
        $dt
      }
    }
  };
  ($t:ty => $dt:expr $(, $t2:ty => $dt2:expr)*) => {
    impl_data!($t => $dt);
    impl_data!($($t2 => $dt2), *);
  }
}

impl_data!(
// invalid => 0,
  bool => 0x02,
  i8 => 0x10,
  i16 => 0x11,
  i32 => 0x12,
  i64 => 0x13,
// i128 => 0x14,
  u8 => 0x20,
  u16 => 0x21,
  u32 => 0x22,
  u64 => 0x23,
//  f16 => 0x31,
  f32 => 0x32,
  f64 => 0x33
);

 

cpp_class!(pub unsafe struct Ctx as "std::shared_ptr<ctx>");

impl Ctx {
  // note: Ctx::default() does not use std::make_shared
  pub fn new() -> Self {
    cpp!(unsafe [] -> Ctx as "std::shared_ptr<ctx>" {
      return std::make_shared<ctx>();
    })
  }
}

cpp_class!(unsafe struct BaseShape as "base_shape");

impl BaseShape {
  #[inline]
  fn new<'c>(ctx: &'c Ctx, dt: usize) -> Self {
    cpp!(unsafe [ctx as "std::shared_ptr<ctx>*", dt as "std::size_t"] -> BaseShape as "base_shape" {
      return base_shape(*ctx, static_cast<plaidml_datatype>(dt));
    })
  }
  #[inline]
  fn add_dimensions<'d>(&mut self, dims: &'d [usize]) {
    cpp!(unsafe [self as "base_shape*", dims as "slice<std::size_t>"] {
      self->add_dimensions(dims);
    });
  }
}

pub struct Shape<T> {
  base_shape: BaseShape,
  _marker: PhantomData<T>
}

impl<T> Shape<T> {
  #[inline]
  pub fn new<'c>(ctx: &'c Ctx) -> Self
    where T: Data {
    Self{base_shape: BaseShape::new(ctx, T::data()),
         _marker: PhantomData::default()}
  }
  #[inline]
  pub fn add_dims<D>(&mut self, dims: D) -> &mut Self
    where D: AsRef<[usize]> {
    self.base_shape.add_dimensions(dims.as_ref());
    self
  } 
  #[inline]
  pub fn dims<'c, D>(self, ctx: &'c Ctx, dims: D) -> Self
    where T: Data, 
          D: AsRef<[usize]> {
    let mut s = Self::new(ctx);
    s.add_dims(dims);
    s
  }
}

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


