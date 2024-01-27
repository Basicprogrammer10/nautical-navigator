use std::f32;

pub trait Average<T> {
    fn average(self) -> T;
}

macro_rules! impl_average {
    ($type:ty, $zero:expr) => {
        impl<K: Iterator<Item = $type>> Average<$type> for K {
            fn average(self) -> $type {
                let mut sum: $type = $zero;
                let mut count = 0;

                for i in self {
                    count += 1;
                    sum += i;
                }

                sum / count as $type
            }
        }
    };
}

impl_average!(f32, 0.0);
impl_average!(f64, 0.0);
