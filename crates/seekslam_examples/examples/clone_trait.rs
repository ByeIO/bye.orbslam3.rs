#![allow(unused)]

//! 为Trait实现Clone方法

fn main()
{
  use clone_dyn_types::CloneDyn;

  /// 封装具有特定特征的迭代器的 trait，根据需求定制
  pub trait IterTrait< 'a, T >
  where
    T : 'a,
    Self : Iterator< Item = T > + ExactSizeIterator< Item = T > + DoubleEndedIterator,
    Self : CloneDyn,
  {
  }

  impl< 'a, T, I > IterTrait< 'a, T > for I
  where
    T : 'a,
    Self : Iterator< Item = T > + ExactSizeIterator< Item = T > + DoubleEndedIterator,
    Self : CloneDyn,
  {
  }

  // 为盒装的 `IterTrait` trait 对象实现 `Clone`
  impl< 'c, T > Clone for Box< dyn IterTrait< 'c, T > + 'c >
  {
    #[ inline ]
    fn clone( &self ) -> Self
    {
      clone_dyn_types::clone_into_box( &**self )
    }
  }

  ///
  /// 获取整数向量迭代器的函数
  ///
  /// 该函数返回一个实现了 `IterTrait` trait 的盒装迭代器。
  /// 如果输入是 `Some`，则返回向量的迭代器；
  /// 如果输入是 `None`，则返回空迭代器。
  ///
  /// Rust 的类型系统由于对象安全限制，不允许 trait 对象直接实现 `Clone` trait。
  /// 具体来说，`Clone` trait 需要在编译时知道具体类型，而 trait 对象无法提供这一信息。
  ///
  /// 在本例中，我们需要返回一个可克隆的迭代器。由于返回的是 trait 对象（`Box< dyn IterTrait >`），
  /// 无法直接为该 trait 对象实现 `Clone`。这时 `clone_dyn_types` 包中的 `CloneDyn` trait 就派上用场了。
  ///
  /// `CloneDyn` trait 通过允许克隆 trait 对象来绕过这一限制。
  /// 它使用过程宏生成克隆 trait 对象所需的代码，使得克隆 trait 对象集合成为可能。
  ///
  /// 此处无法使用 `impl Iterator` 因为代码返回了两种不同类型的迭代器：
  /// - 当输入为 `Some` 时返回 `std::slice::Iter`
  /// - 当输入为 `None` 时返回 `std::iter::Empty`
  ///
  /// 为了处理这种情况，函数返回一个 trait 对象（`Box<dyn IterTrait>`）。
  /// 但由于 Rust 的对象安全限制，`Clone` trait 无法为 trait 对象实现。
  /// `CloneDyn` trait 通过启用 trait 对象的克隆功能解决了这个问题。
  ///

  pub fn get_iter< 'a >( src : Option< &'a Vec< i32 > > ) -> Box< dyn IterTrait< 'a, &'a i32 > + 'a >
  {
    match &src
    {
      Some( src ) => Box::new( src.iter() ),
      _ => Box::new( core::iter::empty() ),
    }
  }

  /// 使用迭代器并打印其元素的函数
  ///
  /// 该函数通过克隆迭代器来演示 `CloneDyn` trait 的使用。
  /// 然后遍历克隆的迭代器并打印每个元素。
  pub fn use_iter< 'a >( iter : Box< dyn IterTrait< 'a, &'a i32 > + 'a > )
  {
    // 如果没有为迭代器实现 CloneDyn，Clone 将不可用。
    // 作为一个对象安全的 trait，它本身不能实现 Clone。
    // 但得益于 CloneDyn，该对象变得可克隆。
    //
    // 这行代码演示了克隆迭代器并遍历克隆后的迭代器。
    // 如果没有 `CloneDyn`，就需要将迭代器收集到容器中，从而在堆上分配内存。
    iter.clone().for_each( | e | println!( "{e}" ) );

    // 遍历原始迭代器并打印每个元素
    iter.for_each( | e | println!( "{e}" ) );
  }

  // 创建一个整数向量
  let data = vec![ 1, 2, 3 ];
  // 获取向量的迭代器
  let iter = get_iter( Some( &data ) );
  // 使用迭代器打印其元素
  use_iter( iter );

}