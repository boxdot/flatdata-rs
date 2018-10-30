use archive::{Factory, IndexFactory, StructMut, VariadicStruct, VariadicStructFactory};
use memory;
use storage::ResourceHandle;
use vector::ExternalVector;

use std::borrow::BorrowMut;
use std::fmt;
use std::io;
use std::marker;

/// A container for writing an indexed sequence of heterogeneous data items.
///
/// The concept of a multivector is used for storing and reading heterogeneous
/// flatdata structs in/from the same container. The data is indexed by
/// integers. Each index refers to a bucket which may contain a variable number
/// of items of different types unified in the same variant enum `Ts`.
/// Such bucket may also be empty, which allows to represent sparse data in a
/// multivector. For those who are familiar with C++'s `std::multimap` data
/// structure, a multivector can be considered as a `std::multimap` mapping
/// integers to sequences of variable length.
///
/// A `MultiVector` corresponds rather to [`ExternalVector`] than to
/// [`Vector`], in the sense that the items are flushed to storage whenever the
/// internal buffer is full. In particular, it is only possible to modify the
/// last bucket. There is no access to the buckets previously stored.
///
/// For accessing and reading the data stored by in multivector, cf.
/// [`MultiArrayView`].
///
/// A multivector *must* be closed, after the last element was written to it.
/// After closing, it can not be used anymore. Not closing the multivector will
/// result in panic on drop (in debug mode).
///
/// Internally data is stored like this:
///
/// * `Index`: `Vector<Idx>` - encodes start/end byte in `Data` array for each
/// element `i`. * `Data`: `Vec<u8>` - sequence of serialized (`Tag`,
/// `ItemData`) tuples, where `Tag` encodes the the variant type, and
/// `ItemData` contains the underlying variant data. `Tag` has size of 1 byte,
/// `ItemData` is of size `Ts::Type::SIZE_IN_BYTES`.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate flatdata;
/// # fn main() {
/// use flatdata::{
///     create_multi_vector, ArrayView, MultiArrayView, MultiVector,
///     Struct, MemoryResourceStorage, ResourceStorage,
/// };
///
/// // define structs usually generated by flatdata's generator
///
/// define_index!(
///     IdxFactory, Idx, IdxMut, "some_idx_schema", 4, 32
/// );
///
/// define_struct!(AFactory, A, AMut, "some_A_schema", 4,
///     (x, set_x, u32, 0, 16),
///     (y, set_y, u32, 16, 16)
/// );
///
/// define_struct!(BFactory, B, BMut, "some_B_schema", 2,
///     (id, set_id, u32, 0, 16)
/// );
///
/// define_variadic_struct!(ABFactory, AB, ABItemBuilder, Idx,
///     0 => (A, add_a),
///     1 => (B, add_b)
/// );
///
/// // create multivector and serialize some data
///
/// let mut storage = MemoryResourceStorage::new("/root/multivec".into());
/// {
///     let mut mv = create_multi_vector::<IdxFactory, ABFactory>(
///             &mut storage, "multivector", "some schema")
///         .expect("failed to create MultiVector");
///     {
///         let mut item = mv.grow().expect("grow failed");
///         {
///             let mut a = item.add_a();
///             a.set_x(1);
///             a.set_y(2);
///         }
///         {
///             let mut b = item.add_b();
///             b.set_id(42);
///         }
///     }
///     mv.close().expect("close failed");
/// }
///
/// // open index and data, and read the data
///
/// let index_resource = storage
///     .read_and_check_schema("multivector_index", "index(some schema)")
///     .expect("read_and_check_schema failed");
/// let index: ArrayView<IdxFactory> = ArrayView::new(&index_resource);
/// let resource = storage
///     .read_and_check_schema("multivector", "some schema")
///     .expect("read_and_check_schema failed");
/// let mv: MultiArrayView<IdxFactory, ABFactory> = MultiArrayView::new(index, &resource);
///
/// assert_eq!(mv.len(), 1);
/// let mut item = mv.at(0);
/// let a = item.next().unwrap();
/// match a {
///     AB::A(ref a) => {
///         assert_eq!(a.x(), 1);
///         assert_eq!(a.y(), 2);
///     },
///     _ => assert!(false),
/// }
/// let b = item.next().unwrap();
/// match b {
///     AB::B(ref b) => {
///         assert_eq!(b.id(), 42);
///     },
///     _ => assert!(false),
/// }
///
/// # }
/// ```
///
/// [`ExternalVector`]: struct.ExternalVector.html
/// [`Vector`]: struct.Vector.html
/// [`MultiArrayView`]: struct.MultiArrayView.html
pub struct MultiVector<Idx, Ts> {
    index: ExternalVector<Idx>,
    data: Vec<u8>,
    data_handle: ResourceHandle,
    size_flushed: usize,
    _phantom: marker::PhantomData<Ts>,
}

impl<Idx, Ts> MultiVector<Idx, Ts>
where
    Idx: for<'b> IndexFactory<'b>,
    Ts: for<'b> VariadicStructFactory<'b>,
{
    /// Creates an empty multivector.
    pub fn new(index: ExternalVector<Idx>, data_handle: ResourceHandle) -> Self {
        Self {
            index,
            data: vec![0; memory::PADDING_SIZE],
            data_handle,
            size_flushed: 0,
            _phantom: marker::PhantomData,
        }
    }

    /// Appends a new item to the end of this multivector and returns a builder
    /// for it.
    ///
    /// The builder is used for storing different variants of `Ts` in the newly
    /// created item.
    ///
    /// Calling this method may flush data to storage (cf. [`flush`]), which
    /// may fail due to different IO reasons.
    ///
    /// [`flush`]: #method.flush
    pub fn grow(&mut self) -> io::Result<<Ts as VariadicStructFactory>::ItemMut> {
        if self.data.len() > 1024 * 1024 * 32 {
            self.flush()?;
        }
        self.add_to_index()?;
        Ok(<Ts as VariadicStructFactory>::create_mut(&mut self.data))
    }

    /// Flushes the not yet flushed content in this multivector to storage.
    ///
    /// Only data is flushed.
    fn flush(&mut self) -> io::Result<()> {
        self.data_handle
            .borrow_mut()
            .write(&self.data[..self.data.len() - memory::PADDING_SIZE])?;
        self.size_flushed += self.data.len() - memory::PADDING_SIZE;
        self.data.clear();
        self.data.resize(memory::PADDING_SIZE, 0);
        Ok(())
    }

    fn add_to_index(&mut self) -> io::Result<()> {
        let idx_mut = <Idx as Factory>::ItemMut::from(self.index.grow()?.as_mut_ptr());
        <Idx as IndexFactory>::set_index(
            idx_mut,
            self.size_flushed + self.data.len() - memory::PADDING_SIZE,
        );
        Ok(())
    }

    /// Flushes the remaining not yet flushed elements in this multivector and
    /// finalizes the data inside the storage.
    ///
    /// After this method is called, more data cannot be written into this
    /// multivector. A multivector *must* be closed, otherwise it will
    /// panic on drop (in debug mode).
    pub fn close(&mut self) -> io::Result<()> {
        self.add_to_index()?; // sentinel for last item
        self.index.close()?;
        self.flush()?;
        self.data_handle.borrow_mut().close()
    }
}

impl<Idx, Ts> Drop for MultiVector<Idx, Ts> {
    fn drop(&mut self) {
        debug_assert!(!self.data_handle.is_open(), "MultiVector not closed")
    }
}

impl<Idx, Ts: VariadicStruct> fmt::Debug for MultiVector<Idx, Ts>
where
    Idx: for<'b> IndexFactory<'b>,
    Ts: for<'b> VariadicStructFactory<'b>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MultiVector {{ len: {} }}", self.index.len())
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use archive::Struct;
    use arrayview::ArrayView;
    use memstorage::MemoryResourceStorage;
    use multiarrayview::MultiArrayView;
    use storage::create_multi_vector;
    use storage::ResourceStorage;

    define_index!(IdxFactory, Idx, IdxMut, "some_idx_schema", 4, 32);

    define_struct!(
        AFactory,
        A,
        AMut,
        "no_schema",
        4,
        (x, set_x, u32, 0, 16),
        (y, set_y, u32, 16, 16)
    );

    define_variadic_struct!(VariantFactory, Variant, VariantItemBuilder, Idx, 0 => (A, add_a) );

    #[test]
    fn test_multi_vector() {
        let mut storage = MemoryResourceStorage::new("/root/resources".into());
        {
            let mut mv = create_multi_vector::<IdxFactory, VariantFactory>(
                &mut storage,
                "multivector",
                "Some schema",
            )
            .expect("failed to create MultiVector");
            {
                let mut item = mv.grow().expect("grow failed");
                {
                    let mut a = item.add_a();
                    a.set_x(1);
                    a.set_y(2);
                    assert_eq!(a.x(), 1);
                    assert_eq!(a.y(), 2);
                }
                {
                    let mut b = item.add_a();
                    b.set_x(3);
                    b.set_y(4);
                    assert_eq!(b.x(), 3);
                    assert_eq!(b.y(), 4);
                }
            }
            mv.close().expect("close failed");
        }

        let index_resource = storage
            .read_and_check_schema("multivector_index", "index(Some schema)")
            .expect("read_and_check_schema failed");
        let index: ArrayView<IdxFactory> = ArrayView::new(&index_resource);
        let resource = storage
            .read_and_check_schema("multivector", "Some schema")
            .expect("read_and_check_schema failed");
        let mv: MultiArrayView<IdxFactory, VariantFactory> = MultiArrayView::new(index, &resource);

        assert_eq!(mv.len(), 1);
        let mut item = mv.at(0);
        let a = item.next().unwrap();
        match a {
            Variant::A(ref a) => {
                assert_eq!(a.x(), 1);
                assert_eq!(a.y(), 2);
            }
        }
        let b = item.next().unwrap();
        match b {
            Variant::A(ref a) => {
                assert_eq!(a.x(), 3);
                assert_eq!(a.y(), 4);
            }
        }
    }
}
