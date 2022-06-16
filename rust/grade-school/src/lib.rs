use std::collections::BTreeMap;

#[derive(Default)]
pub struct School {
    grades: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        Self::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grades
            .entry(grade)
            .and_modify(|e| {
                e.push(student.to_owned());
                e.sort_unstable();
            })
            .or_insert_with(|| vec![student.to_owned()]);
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().cloned().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        if self.grades.contains_key(&grade) {
            self.grades[&grade].clone()
        } else {
            Vec::new()
        }
    }
}
