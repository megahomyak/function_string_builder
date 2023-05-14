# A new approach to building strings

Instead of maintaining a buffer, this thing minimizes the amount of allocations by calling the user-provided function two times: the first time it sums the lengths of the strings returned using a callback by the function, the second time it pushes said strings into a `String` that was extended using the lengths sum.

Such approach is aimed at minimizing the amount and size of allocations as much as possible.

# An example

    assert_eq!(
        build(|mut collector| {
            collector.collect("a");
            collector.collect("bcd");
            collector.collect("ef");
        }),
        "abcdef"
    );
