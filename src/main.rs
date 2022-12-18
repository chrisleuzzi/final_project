    /// Initialize vector of tuples, where each tuple represents a course and the grade received
    /// Initialize variables to hold the total number of credits and the total grade points
    /// For loop Iterates through the courses and calculate the total credits and grade points
    /// get_credits function returns the number of credits for a given course
    /// Calculate the GPA by dividing the total grade points by the total credits
    /// The result is then printed to the terminal

fn main() {

    let courses = vec![("Web Development", 4.0), ("Information Security", 3.3), ("Advanced Programming Workshop", 3.7), ("Ethics", 3.3)];
    let mut total_credits = 0;
    let mut total_grade_points = 0.0;

    for (course, grade) in &courses {
        let credits = get_credits(course);
        let grade_points = credits as f64 * grade;
        total_credits += credits;
        total_grade_points += grade_points;
    }

    let gpa = total_grade_points / total_credits as f64;

    println!("GPA: {:.2}", gpa);
}

fn get_credits(course: &str) -> u32 {
    match course {
        "Web Development" => 4,
        "Information Security" => 3,
        "Advanced Programming Workshop" => 2,
        "Ethics" => 3,
        _ => panic!("Invalid course"),
    }
}
