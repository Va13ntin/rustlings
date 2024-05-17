// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

pub trait Grade {
    fn letter_grade(&self) -> String;
}

impl Grade for f32 {
    fn letter_grade(&self) -> String {
        if *self >= 4.5 {
            String::from("A+")
        } else if *self >= 4.0 {
            String::from("A")
        } else if *self >= 3.5 {
            String::from("B+")
        } else if *self >= 3.0 {
            String::from("B")
        } else if *self >= 2.5 {
            String::from("C+")
        } else if *self >= 2.0 {
            String::from("C")
        } else if *self >= 1.5 {
            String::from("D+")
        } else if *self >= 1.0 {
            String::from("D")
        } else {
            String::from("F")
        }
    }
}

pub struct ReportCard<T: Grade> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: Grade> ReportCard<T> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, self.grade.letter_grade())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of C"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: 4.8,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
