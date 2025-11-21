#[macro_export]
macro_rules! array {
    // Multi-dimensional: array![[...], [...]]
    // Check if first element starts with a bracket
    ([$($inner:tt),+] $(, [$($rest:tt),+])+ $(,)?) => {{
        $crate::array_helper!(@build [$($inner),+] $([$($rest),+])+)
    }};

    // 1D array: array![1, 2, 3]
    ($($x:expr),+ $(,)?) => {{
        let data = vec![$($x),+];
        let shape = vec![data.len()];
        $crate::Array::new(data, shape)
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! array_helper {
    // Build from multiple rows
    (@build [$($first:tt),+] $([$($rest:tt),+])+) => {{
        let mut all_data = Vec::new();
        let mut first_shape = None;

        // Process first row
        let (data, shape) = $crate::array_helper!(@process [$($first),+]);
        first_shape = Some(shape.clone());
        all_data.extend(data);

        // Process remaining rows
        let num_rows = 1 $(+ $crate::array_helper!(@count_row [$($rest),+]))*;

        $(
            let (data, shape) = $crate::array_helper!(@process [$($rest),+]);
            assert_eq!(
                first_shape.as_ref().unwrap(), &shape,
                "Inconsistent dimensions: expected {:?}, got {:?}",
                first_shape.as_ref().unwrap(), shape
            );
            all_data.extend(data);
        )+

        let inner_shape = first_shape.unwrap();
        let mut final_shape = vec![num_rows];
        final_shape.extend(&inner_shape);

        $crate::Array::new(all_data, final_shape)
    }};

    // Process a nested array recursively
    (@process [[$($inner:tt),+] $(, [$($rest:tt),+])* $(,)?]) => {{
        let mut all_data = Vec::new();
        let mut first_shape = None;

        // Process first nested row
        let (data, shape) = $crate::array_helper!(@process [$($inner),+]);
        first_shape = Some(shape.clone());
        all_data.extend(data);

        let num_rows = 1 $(+ $crate::array_helper!(@count_row [$($rest),+]))*;

        // Process remaining nested rows
        $(
            let (data, shape) = $crate::array_helper!(@process [$($rest),+]);
            assert_eq!(
                first_shape.as_ref().unwrap(), &shape,
                "Inconsistent dimensions: expected {:?}, got {:?}",
                first_shape.as_ref().unwrap(), shape
            );
            all_data.extend(data);
        )*

        let inner_shape = first_shape.unwrap();
        let mut final_shape = vec![num_rows];
        final_shape.extend(&inner_shape);

        (all_data, final_shape)
    }};

    // Base case: flat array of scalars
    (@process [$($x:expr),+ $(,)?]) => {{
        let data = vec![$($x),+];
        let shape = vec![data.len()];
        (data, shape)
    }};

    // Helper to count rows
    (@count_row $x:tt) => { 1 };
}
