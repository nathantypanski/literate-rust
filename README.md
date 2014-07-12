literate-rust [![build-status](https://travis-ci.org/nathantypanski/literate-rust.svg?branch=master)](https://travis-ci.org/nathantypanski/rtop/builds)
=============

Support for literate programming in Rust

Excerpt from [post to rust-dev](https://mail.mozilla.org/pipermail/rust-dev/2014-June/010628.html):

> ## Who would benefit from this
> 
> ### Schools teaching Rust
> 
> Universities teaching Rust would be one of my first bets as a user for
> Literate Rust. While literate programming is sometimes too far a
> stretch from reality for production code (which is both a good and bad
> thing), pedagogical gains for literate programming are huge - you can
> run the same code that's teaching you about the language or the
> concept. Essays and writings that feature the code are
> indistinguishable from valid Rust code.
> 
> ### Documentation/books/etc.
> 
> Another use is general documentation, books about Rust, etc. where
> there are real benefits to having valid code all the way through them.
> When walking through a sample project, the sample project is also the
> code that produced it. This makes documentation easier, since you're
> more able to ensure that your examples compile properly and actually
> run when the compiler updates, etc.
> 
> There's also a mental shift when doing literate programming, versus
> just ///'ing within code and whatnot: your primary thought process
> becomes thinking and communicating with the reader, rather than
> working on the code. You focus more on the text and ensuring that it
> makes sense.
> 
> ## How this could work with Rustdoc for APIs
> 
> API documentation in a literate program would be included as before,
> in rustdoc comments in the output. The internal comments should be the
> default for text, not external API.
> 
> As an example, here's some code taken from 
> src/libstd/collections/hashmap.rs, and interpreted in a literate style:
> 
>     > /// Does not initialize the buckets. The caller should ensure they,
>     > /// at the very least, set every hash to EMPTY_BUCKET.
>     > unsafe fn new_uninitialized(capacity: uint) -> RawTable<K, V> {
>     >     let hashes_size = capacity.checked_mul(&size_of::<u64>())
>     >                               .expect("capacity overflow");
>     >     let keys_size = capacity.checked_mul(&size_of::< K >())
>     >                             .expect("capacity overflow");
>     >     let vals_size = capacity.checked_mul(&size_of::< V >())
>     >                             .expect("capacity overflow");
>     >
>     
>     Allocating hashmaps is a little tricky. We need to allocate three
>     arrays, but since we know their sizes and alignments up front,
>     we just allocate a single array, and then have the subarrays
>     point into it.
>     
>     This is great in theory, but in practice getting the alignment
>     right is a little subtle. Therefore, calculating offsets has been
>     factored out into a different function.
>     
>     >     let (malloc_alignment, hash_offset, keys_offset, vals_offset, size) =
>     >         calculate_offsets(
>     >             hashes_size, min_align_of::<u64>(),
>     >             keys_size,   min_align_of::< K >(),
>     >             vals_size,   min_align_of::< V >());
> 
> The non-bird text (without the >) is what would normally go in the
> comments for the code. In this case, since it's an API, the thing you
> actually want in your regular documentation is still the rustdoc api
> comments (///), but with the option of generating a literate
> presentation of the code (perhaps by autoconversion to pandoc?) where
> you read the internal comments instead of the API docs as the main
> source of writing.
> 
> In other words, literate files can serve as a guide to *developers* of
> the code, not necessarily just to users. Though for non-API material,
> it's useful for both.
> 
> ## Editor updates
> 
> - Emacs and Vim plugins would have to be updated to understand the new
>   extension, and actually behave properly when editing one of these
>   files. Probably we should have conditional syntax highlighting
>   depending on the block type that we're working on.
> 
> ## Rustc
> 
> Rustc would need to be able to interpret and understand literate Rust
> code just as well as it can regular code, so that they can link to one
> another without an additional step - otherwise this isn't useful. I
> should be able to run Literate Rust just like it's any other Rust
> code.
> 
> ### Conversion
> 
> There are relatively easy ways to do "literate -> code" conversion
> without actually interfering with anything. Haskell has a sed
> implementation [2] on their wiki. Re-implementing this in Rust would
> not be a significant effort.
> 
> I think a reasonable way to manage the conversion would be to have a
> hidden step that moves the converted code to /tmp or something
> similar, then compiles that code and removes the converted bit.
> 
> There would be some additional voodoo required to make this behave
> perfectly:
> 
> - Pathnames should still show the original file.
> - Line numbers can't be wonky.
> - Linker would have to understand this, not sure how that would be
>   affected.
> 
> The gist of this is that the conversion step should be as transparent
> as possible - ideally, an absolute minimum of compiler code would need
> to be modified, since we would have some way to transparently resolve
> the literate file to its converted version.
> 
> [2]: <http://www.haskell.org/haskellwiki/Literate_programming/Bird_conversion_via_sed>
> 
> Any feedback is appreciated.
