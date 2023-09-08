use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0.0",
    author = "Retrokiller543",
    name = "FerricSort",
    about = "A fast sorting algorithm for large files"
)]
struct Opts {
    file: Option<String>,
    #[clap(short, long)]
    new_name: Option<String>,
}

mod file {
    use std::{io::Write, path::Path};

    pub struct File {
        pub name: String,
        pub path: String,
        pub content: Vec<i64>,
    }

    // Extension trait for Vec<i64>
    pub trait SortVecI64 {
        fn sorted(&self) -> Vec<i64>;
        fn custom_sort(&mut self);
    }

    impl SortVecI64 for Vec<i64> {
        // Sort the vector and return a new sorted vector
        fn sorted(&self) -> Vec<i64> {
            let mut sorted_vec = self.clone();
            sorted_vec.custom_sort();
            sorted_vec
        }

        fn custom_sort(&mut self) {
            const THRESHOLD: usize = 15; // Switch to insertion sort when the array size is <= THRESHOLD

            // Function to select pivot for quicksort based on the strategy median of three
            fn choose_pivot_value(list: &[i64]) -> i64 {
                let first = list[0];
                let middle = list[list.len() / 2];
                let last = list[list.len() - 1];

                // Find the median of the first, middle, and last elements
                let mut med = first;
                if middle < first && middle > last || middle > first && middle < last {
                    med = middle;
                } else if last < first && last > middle || last > first && last < middle {
                    med = last;
                }

                med
            }

            // Function to partition the list
            fn partition(list: &mut [i64], left: usize, right: usize, pivot_val: i64) -> usize {
                let mut i = left;
                let mut j = right;
                while i <= j {
                    while list[i] < pivot_val {
                        i += 1;
                    }
                    while list[j] > pivot_val {
                        if j == 0 {
                            break;
                        } // Ensure we don't underflow
                        j -= 1;
                    }
                    if i <= j {
                        list.swap(i, j);
                        i += 1;
                        if j == 0 {
                            break;
                        } // Ensure we don't underflow
                        j -= 1;
                    }
                }
                i
            }

            // Function to sort the list
            fn quick_sort(list: &mut [i64], left: usize, right: usize) {
                if right - left + 1 <= THRESHOLD {
                    insertion_sort(&mut list[left..=right]);
                } else {
                    let pivot_val = choose_pivot_value(&list[left..=right]);
                    let partition_idx = partition(list, left, right, pivot_val);
                    if partition_idx > 0 {
                        quick_sort(list, left, partition_idx - 1);
                    }
                    quick_sort(list, partition_idx, right);
                }
            }

            // Function to sort the list using insertion sort
            fn insertion_sort(list: &mut [i64]) {
                for i in 1..list.len() {
                    let mut j = i;
                    while j > 0 && list[j] < list[j - 1] {
                        list.swap(j, j - 1);
                        j -= 1;
                    }
                }
            }

            // Initiate the quicksort
            let len = self.len();
            if !self.is_empty() {
                quick_sort(self, 0, len - 1);
            }
        }
    }

    impl File {
        pub fn read_file(path: &Path) -> File {
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            let content_str = std::fs::read_to_string(path).unwrap();
            let content: Vec<i64> = content_str
                .lines()
                .filter_map(|line| line.parse().ok())
                .collect();
            File {
                name,
                path: path.to_str().unwrap().to_string(),
                content,
            }
        }

        pub fn write_file(&self, new_name: Option<String>) {
            match new_name {
                Some(name) => {
                    let path = Path::new(&name);
                    let mut file = std::fs::File::create(path).unwrap();
                    for num in &self.content {
                        let num_str = num.to_string();
                        file.write_all(num_str.as_bytes()).unwrap();
                        file.write_all(b"\n").unwrap();
                    }
                }
                None => {
                    let path = Path::new(&self.path);
                    let mut file = std::fs::File::create(path).unwrap();
                    for num in &self.content {
                        let num_str = num.to_string();
                        file.write_all(num_str.as_bytes()).unwrap();
                        file.write_all(b"\n").unwrap();
                    }
                }
            }
        }

        pub fn sort(&self) -> File {
            let sorted_content = self.content.sorted();

            File {
                name: self.name.clone(),
                path: self.path.clone(),
                content: sorted_content,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use rand::Rng;

        fn generate_test_data(size: usize) -> Vec<i64> {
            let mut vec = Vec::new();

            for _ in 0..size {
                let mut rng = rand::thread_rng();
                let num = rng.gen_range(0..10000);
                vec.push(num);
            }

            vec
        }

        fn generate_test_file(vec: Vec<i64>) -> File {
            let sorted = vec.sorted();
            let file = File {
                name: "tmp-test.txt".to_string(),
                path: "tmp-test.txt".to_string(),
                content: sorted,
            };
            file
        }

        #[test]
        fn test_read_file() {
            let path = Path::new("Tests/test.txt");
            let file = File::read_file(path);
            assert_eq!(file.name, "test.txt");
            assert_eq!(file.content, vec![2, 1, 4, 3, 5]);
        }

        #[test]
        fn test_sort_simple() {
            let path = Path::new("Tests/test.txt");
            let file = File::read_file(path);
            let file = file.sort();
            assert_eq!(file.name, "test.txt");
            assert_eq!(file.content, vec![1, 2, 3, 4, 5]);
        }

        #[test]
        fn test_sort_random_100() {
            let vec = generate_test_data(100);
            let mut control = vec.clone();
            let file = generate_test_file(vec);
            control.sort();
            assert_eq!(file.content, control);
        }

        #[test]
        fn test_sort_random_1000() {
            let vec = generate_test_data(1000);
            let mut control = vec.clone();
            let file = generate_test_file(vec);
            control.sort();
            assert_eq!(file.content, control);
        }

        #[test]
        fn test_sort_random_10000() {
            let vec = generate_test_data(10000);
            let mut control = vec.clone();
            let file = generate_test_file(vec);
            control.sort();
            assert_eq!(file.content, control);
        }

        #[test]
        fn test_sort_random_100000() {
            let vec = generate_test_data(100000);
            let mut control = vec.clone();
            let file = generate_test_file(vec);
            control.sort();
            assert_eq!(file.content, control);
        }

        #[test]
        fn test_sort_random_1000000() {
            let vec = generate_test_data(1000000);
            let mut control = vec.clone();
            let file = generate_test_file(vec);
            control.sort();
            assert_eq!(file.content, control);
        }
    }
}

fn main() {
    let opts: Opts = Opts::parse();
    match opts.file {
        Some(file) => {
            let path = std::path::Path::new(&file);
            let file = file::File::read_file(path);
            let file = file.sort();

            match opts.new_name {
                Some(name) => {
                    file.write_file(Some(name));
                }
                None => {
                    file.write_file(None);
                }
            }
        }
        None => {
            println!("No file specified");
        }
    }
}
