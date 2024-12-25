use std::fs;
use std::io;

fn parse_file() -> Result<Vec<Vec<i32>>, io::Error> {
    let content = match fs::read_to_string("./d2.txt") {
        Ok(content) => content,
        Err(e) => return Err(e),
    };

    let reports = content
        .split("\n")
        .into_iter()
        .map(|row| {
            row.split(" ")
                .into_iter()
                .map(|v| v.trim())
                .filter(|v| !v.is_empty())
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect();

    return Ok(reports);
}

fn check_safety(report: &Vec<i32>) -> bool {
    let is_asc = report.windows(2).all(|w| w[0] > w[1]);
    let is_desc = report.windows(2).all(|w| w[0] < w[1]);
    let is_diff_correct = report.windows(2).all(|w| {
        let diff = i32::abs(w[0] - w[1]);
        diff >= 1 && diff <= 3
    });

    return (is_asc || is_desc) && is_diff_correct;
}

fn check_safety2(report: &Vec<i32>) -> bool {
    if check_safety(report) {
        return true;
    }

    let any_possibility = report.iter().enumerate().any(|(i, _)| {
        let mut mod_report = report.clone();
        mod_report.remove(i);
        return check_safety(&mod_report);
    });

    return any_possibility;
}

pub fn main() {
    let input = parse_file()
        .unwrap()
        .into_iter()
        .filter(|report| report.len() > 0);

    let number_of_safe = input.clone().filter(check_safety).count();
    let damper_number_of_safe = input.clone().filter(check_safety2).count();

    println!("Safe reports (no damper): {number_of_safe}");
    println!("Safe reports (damper): {damper_number_of_safe}");
}

//fn check_safety_dampener(report: &Vec<i32>) -> bool {
//    let asc_check = report.windows(2).map(|w| w[0] > w[1]);
//    let desc_check = report.windows(2).map(|w| w[0] < w[1]);
//    let is_diff_correct = report.windows(2).map(|w| {
//        let diff = i32::abs(w[0] - w[1]);
//        diff >= 1 && diff <= 3
//    });
//
//    // [1,3,4,6,8] -> (1,3) (3,4) (4,6) (6,8) = len - 1
//
//    // [1,3,2,4,5] -> (1,3) (3,2) (2,4) (4,5)
//    //            ->  true  false true  true
//    //            - 3 or 2
//
//    // [1,3,2,6,7] -> (1,3) (3,2) (2,6) (6,7)
//    //             -> true  false false true
//    //             - 2
//
//    // [1,3,4,5,6,10] -> (1,3) (3,4) (4,5) (5,6) (6,10)
//    //               ->  true  true  true  true  false
//    //               - 10
//
//    // [10,2,3,4,5] -> (10,2) (2,3) (3,4) (4,5)
//    //              -> false  true  true  true
//    //              - 10
//
//    return (is_asc || is_desc) && is_diff_correct;
//}
