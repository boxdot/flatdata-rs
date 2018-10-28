use archive::{Index, VariadicStruct};
use arrayview::ArrayView;
use handle::Handle;

use std::fmt;
use std::iter;
use std::marker;

/// A read-only view on a multivector.
///
/// For the detailed description of multivector and examples, cf.
/// [`MultiVector`].
///
/// [`MultiVector`]: struct.MultiVector.html
#[derive(Clone)]
pub struct MultiArrayView<'a, Idx: 'a, Ts: 'a> {
    index: ArrayView<'a, Idx>,
    data: &'a [u8],
    _phantom: marker::PhantomData<&'a Ts>,
}

impl<'a, Idx, Ts> MultiArrayView<'a, Idx, Ts>
where
    Idx: Index,
    Ts: VariadicStruct,
{
    /// Creates a new `MultiArrayView` to the data at the given address.
    ///
    /// The returned array view does not own the data.
    pub fn new(index: ArrayView<'a, Idx>, data_mem_descr: &'a [u8]) -> Self {
        Self {
            index,
            data: &data_mem_descr,
            _phantom: marker::PhantomData,
        }
    }

    /// Number of indexed items in the array.
    ///
    /// Note that this is not the *total* number of overall elements stored in
    /// the array. An item may be also empty.
    pub fn len(&self) -> usize {
        // last index element is a sentinel
        self.index.len() - 1
    }

    /// Returns `true` if no item is stored in the array.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a read-only iterator to the elements of the item at position
    /// `index`.
    ///
    /// # Panics
    ///
    /// Panics if index is greater than or equal to `MultiArrayView::len()`.
    pub fn at(&self, index: usize) -> MultiArrayViewItemIter<'a, Ts> {
        let start = self.index.at(index).value();
        let end = self.index.at(index + 1).value();
        MultiArrayViewItemIter {
            data: &self.data[start..end],
            _phantom: marker::PhantomData,
        }
    }

    /// Returns an iterator through the indexed items of the array.
    pub fn iter(&'a self) -> MultiArrayViewIter<Idx, Ts> {
        MultiArrayViewIter {
            view: self,
            next_pos: 0,
        }
    }
}

/// Iterator through elements of an array item.
///
/// An item may be empty.
#[derive(Debug, Clone)]
pub struct MultiArrayViewItemIter<'a, Ts: 'a> {
    data: &'a [u8],
    _phantom: marker::PhantomData<&'a Ts>,
}

impl<'a, Ts: 'a + VariadicStruct> iter::Iterator for MultiArrayViewItemIter<'a, Ts> {
    type Item = Handle<'a, Ts>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() > 0 {
            let type_index = self.data[0];
            self.data = &self.data[1..];
            let res = Ts::from((type_index, self.data.as_ptr()));
            self.data = &self.data[res.size_in_bytes()..];
            Some(Handle::new(res))
        } else {
            None
        }
    }
}

impl<'a, Idx, Ts> fmt::Debug for MultiArrayView<'a, Idx, Ts>
where
    Idx: Index,
    Ts: VariadicStruct,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let preview: Vec<(usize, Vec<_>)> = self
            .iter()
            .take(super::DEBUG_PREVIEW_LEN)
            .enumerate()
            .map(|(index, item)| (index, item.collect()))
            .collect();
        write!(
            f,
            "MultiArrayView {{ len: {}, data: {:?}{} }}",
            self.len(),
            preview,
            if self.len() <= super::DEBUG_PREVIEW_LEN {
                ""
            } else {
                "..."
            }
        )
    }
}

/// Iterator through items of an multivector.
#[derive(Debug, Clone)]
pub struct MultiArrayViewIter<'a, Idx: 'a + Index, Ts: 'a + VariadicStruct> {
    view: &'a MultiArrayView<'a, Idx, Ts>,
    next_pos: usize,
}

impl<'a, Idx: Index, Ts: 'a + VariadicStruct> iter::Iterator for MultiArrayViewIter<'a, Idx, Ts> {
    type Item = MultiArrayViewItemIter<'a, Ts>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next_pos < self.view.len() {
            let element = self.view.at(self.next_pos);
            self.next_pos += 1;
            Some(element)
        } else {
            None
        }
    }
}

impl<'a, Idx: Index, Ts: VariadicStruct> iter::ExactSizeIterator
    for MultiArrayViewIter<'a, Idx, Ts>
{
    fn len(&self) -> usize {
        self.view.len()
    }
}
