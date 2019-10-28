/*
 * MIT License (MIT)
 * Copyright (c) 2019 Activeledger
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! input_str {
    ($e:expr) => {
        $crate::PacketValue::String($e.to_string())
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! input_array {
    ($e:expr) => {
        $crate::PacketValue::Array($e)
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! input_map {
    ($e:expr) => {
        $crate::PacketValue::Object($e)
    };
}

/// # Signees Macro
///
/// This macro is used to quickly create a new Signees struct.
///
/// ## Examples
///
/// ### Keys and stream IDs
///
/// #### One key
/// ```
/// # use active_tx::{signees, Key};
/// # use activeledger::key::EllipticCurve;
/// #
/// let key = Key::Ec(EllipticCurve::new("key").unwrap());
/// let signees = signees![{"streamid" => key}];
/// ```
///
/// #### Multiple keys
/// ```
/// # use active_tx::{signees, Key};
/// # use activeledger::key::EllipticCurve;
/// #
/// let key = Key::Ec(EllipticCurve::new("key").unwrap());
/// let key2 = Key::Ec(EllipticCurve::new("key2").unwrap());
/// let key3 = Key::Ec(EllipticCurve::new("key3").unwrap());
///
/// let signees = signees![
///     {"streamid" => key},
///     {"streamid2" => key2},
///     {"streamid3" => key3}
/// ];
/// ```
///
/// #### Key for selfsign
/// This uses the keys name as the stream id.
/// ```
/// # use active_tx::{signees, Key};
/// # use activeledger::key::EllipticCurve;
/// #
/// let key = Key::Ec(EllipticCurve::new("key").unwrap());
/// let signees = signees!(key);
/// ```
///
/// #### Empty
/// Use this to quickly create an empty Signees if you wish to manually enter keys and streamids
/// via the .add() method.
/// ```
/// # use active_tx::signees;
/// #
/// let signees = signees!();
/// ```
#[macro_export]
macro_rules! signees {
    [$({$streamid: expr => $key:expr}),+] => {{
        let mut signees = $crate::Signees::new();

        $( signees.add($key, $streamid); )*

        signees
    }};

    ($key:expr) => {{
       let mut s = $crate::Signees::new();
       s.add_selfsign($key);
       s.clone()
    }};

    () => {{
        let s = $crate::Signees::new();
        s.clone()
    }};
}

/// # Packet data macro
///
/// This macro can be used to generate [`PacketValue`]s which are consumed by the [`PacketBuilder`] to generate
/// sections of the transaction packet.
/// These are $i (input), $o (output), and $r (readonly).
///
/// This macro is designed to be more restrictive than standard JSON. This means that it will only
/// accept objects, string, and arrays. The `json!` macro in the [serde_json](https://crates.io/crates/serde_json) crate
/// can be used for more flexibility and passed to the `add_json` method provided in [`PacketBuilder`].
///
/// ## Examples
/// ```
/// # use active_tx::{PacketBuilder, packet_data};
/// let value = packet_data!({"basic": "object"});
/// // This value can now be passed to the builder for consumption
/// let builder = PacketBuilder::new(value);
///
///
/// // Objects can be nested
/// let value = packet_data!({"parent": {"nested": "object"}});
///
/// // And arrays can be used
/// let value = packet_data!({"array": ["1", "2", "3"]});
/// ```
///
#[macro_export(local_inner_macros)]
macro_rules! packet_data {
    ($($data:tt)+) => {
        packet_data_internal!($($data)+)
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! packet_data_internal {
    // ---------------
    // Object handling
    // ---------------

    // Reached the end
    (@object $object:ident () () ()) => {};

    // Entry with trailing comma
    (@object $object:ident [$($key:tt)+] ($value:expr), $($tail:tt)*) => {
        let _ = $object.insert(($($key)+).into(), $value);
        packet_data_internal!(@object $object () ($($tail)*) ($($tail)*));
    };

    // Last entry, no trailing comma
    (@object $object:ident [$($key:tt)+] ($value:expr)) => {
        let _ = $object.insert(($($key)+).into(), $value);
    };

    // Next value is object
    (@object $object:ident ($($key:tt)+) (: {$($map:tt)*} $($tail:tt)*) $copy:tt) => {
        packet_data_internal!(@object $object [$($key)+] (packet_data_internal!({$($map)*})) $($tail)*);
    };

    // Next value is array
    (@object $object:ident ($($key:tt)+) (: [$($array:tt)*] $($tail:tt)*) $copy:tt) => {
        packet_data_internal!(@object $object [$($key)+] (packet_data_internal!([$($array)*])) $($tail)*);
    };

    // Next value, expression, tail comma
    (@object $object:ident ($($key:tt)+) (: $value:expr, $($tail:tt)*) $copy:tt) => {
        packet_data_internal!(@object $object [$($key)+] (packet_data_internal!($value)) , $($tail)*);
    };

    // Last value, expression, no tail comma
    (@object $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        packet_data_internal!(@object $object [$($key)+] (packet_data_internal!($value)));
    };

    // Fully parenthesized
    (@object $object:ident () (($key:expr) : $($tail:tt)*) $copy:tt) => {
        packet_data_internal!(@object $object ($key) (: $($tail)*) (: $($tail)*));
    };

    // Munch token
    (@object $object:ident ($($key:tt)*) ($data:tt $($tail:tt)*) $copy:tt) => {
        packet_data_internal!(@object $object ($($key)* $data) ($($tail)*) ($($tail)*));
    };

    (@object $object:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($tail:tt)*) => {
        input_unexpected!($unexpected);
    };

    // --------------
    // Array Handling
    // --------------

    // Done with trailing commas
    (@array [$($elems:expr,)*]) => {
        packet_data_internal_vec![$($elems,)*]
    };

    // Done without trailing commas
    (@array [$($elems:expr),*]) => {
        packet_data_internal_vec![$($elems),*]
    };

    // Next: expression followed by comma
    (@array [$($elems:expr,)*] $next:expr, $($tail:tt)*) => {

        packet_data_internal!(@array [$($elems,)* packet_data_internal!($next),] $($tail)*)
    };

    // Last: expression no tail comma
    (@array [$($elems:expr,)*] $last:expr) => {
        packet_data_internal!(@array [$($elems,)* packet_data_internal!($last)])
    };

    // Comma after most recent
    (@array [$($elems:expr,)*] , $($tail:tt)*) => {
        packet_data_internal!(@array [$($elems,)*] $($tail:tt)*)
    };

    // Unexpected element
    (@array [$($elems:expr),*] $unexpected:tt $($tail:tt)*) => {
        input_unexpected!($unexpected)
    };

    // -------------------
    // Main implementation
    // -------------------

    // Empty object
    ({}) => {{
        $crate::PacketValue::Object(std::collections::HashMap::new())
    }};

    // Object
    ({ $($data:tt)+ }) => {
        $crate::PacketValue::Object({
            let mut object = std::collections::HashMap::new();
            packet_data_internal!(@object object () ($($data)+) ($($data)+));
            object
        })
    };

    // Empty array
    ([]) => {
        $crate::PacketValue::Array(packet_data_internal_vec![])
    };

    // Array
    ([ $($data:tt)+ ]) => {
        $crate::PacketValue::Array(packet_data_internal!(@array [] $($data)+))
    };

    // String
    ($other:expr) => {
        input_str!(&$other)
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! packet_data_internal_vec {
    ($($data:tt)*) => {
        vec![$($data)*]
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! input_unexpected {
    () => {};
}
