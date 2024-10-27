use tui_input::Input;

pub const CATEGORIES: [&str; 18] = [
    "Arrays & Hasing",
    "Two Pointers",
    "Sliding Window",
    "Stack",
    "Binary Search",
    "Linked List",
    "Trees",
    "Heap / Priority Queue",
    "Backtracking",
    "Tries",
    "Graphs",
    "Advanced Graphs",
    "1-D Dynamic Programming",
    "2-D Dynamic Programming",
    "Greedy",
    "Intervals",
    "Math & Geometry",
    "Bit Manipulation",
];

pub fn number_validator(num: &Input) -> bool {
    num.value().trim().parse::<u32>().is_ok()
}

pub fn type_validator(entered_type: &str) -> bool {
    CATEGORIES.contains(&entered_type)
}
