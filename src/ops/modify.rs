use deno_core::op;
use deno_core::serde_v8;
use deno_core::v8;
use serde_v8::from_v8;

#[op(v8)]
fn op_encode<'a>(scope: &mut v8::HandleScope, text: serde_v8::Value) -> serde_v8::Value<'a> {
    let text = v8::Local::<v8::String>::try_from(text.v8_value).unwrap();
    let text_str = serde_v8::to_utf8(text, scope);
    let bytes = text_str.into_bytes();
    let len = bytes.len();
    let backing_store = v8::ArrayBuffer::new_backing_store_from_vec(bytes).make_shared();
    let buffer = v8::ArrayBuffer::with_backing_store(scope, &backing_store);
    let u8array = v8::Uint8Array::new(scope, buffer, 0, len).unwrap();
    from_v8(scope, u8array.into()).unwrap()
}

#[op(v8)]
fn op_encode_fast<'a>(
    scope: &mut v8::HandleScope,
    text: serde_v8::Value<'a>,
) -> serde_v8::Value<'a> {
    let s = v8::Local::<v8::String>::try_from(text.v8_value).unwrap();
    let len = s.length();
    let capacity = (len as f64 * 1.2) as usize;
    let mut buf = Vec::with_capacity(capacity);
    let mut nchars = 0;
    let data = buf.as_mut_ptr();
    let length = s.write_utf8(
        scope,
        unsafe { std::slice::from_raw_parts_mut(data, len) },
        Some(&mut nchars),
        v8::WriteOptions::NO_NULL_TERMINATION | v8::WriteOptions::REPLACE_INVALID_UTF8,
    );
    unsafe { buf.set_len(length) };
    let backing_store = v8::ArrayBuffer::new_backing_store_from_vec(buf).make_shared();
    let buffer = v8::ArrayBuffer::with_backing_store(scope, &backing_store);
    from_v8(scope, buffer.into()).unwrap()
}
