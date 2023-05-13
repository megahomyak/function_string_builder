trait InnerCollector {
    fn collect(&mut self, string: &str);
}

pub struct Collector<'a> {
    inner: &'a mut dyn InnerCollector,
}

impl<'a> Collector<'a> {
    pub fn collect(&mut self, string: &str) {
        self.inner.collect(string);
    }
}

struct StringCollector<'a> {
    master_string: &'a mut String,
}

struct LengthCollector {
    length: usize,
}

impl<'a> InnerCollector for StringCollector<'a> {
    fn collect(&mut self, string: &str) {
        self.master_string.push_str(string);
    }
}

impl InnerCollector for LengthCollector {
    fn collect(&mut self, string: &str) {
        self.length += string.len();
    }
}

pub fn build(collector: impl Fn(Collector)) -> String {
    let mut master_string = String::new();
    let mut length_collector = LengthCollector { length: 0 };
    collector(Collector {
        inner: &mut length_collector,
    });
    master_string.reserve_exact(length_collector.length);
    let mut string_collector = StringCollector {
        master_string: &mut master_string,
    };
    collector(Collector {
        inner: &mut string_collector,
    });
    master_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_building() {
        assert_eq!(
            build(|mut collector| {
                collector.collect(&"a");
                collector.collect(&"bcd");
                collector.collect(&"ef");
            }),
            "abcdef"
        );
    }
}
