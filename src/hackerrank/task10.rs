#[allow(dead_code)]
pub fn day_of_the_programmer(year: i32) -> String {
    if year == 1918 {
        return format!("26.09.{}", year);
    }

    let leap_year;

    if year < 1918 {
        leap_year = year % 4 == 0;
    } else {
        leap_year = (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0);
    }

    let day = if leap_year { 12 } else { 13 };

    format!("{:02}.09.{}", day, year)
}

#[cfg(test)]
mod tests {
    use super::day_of_the_programmer;

    #[test]
    fn julian_leap_year() {
        assert_eq!(day_of_the_programmer(1800), "12.09.1800");
    }

    #[test]
    fn gregorian_leap_year() {
        assert_eq!(day_of_the_programmer(2000), "12.09.2000");
    }

    #[test]
    fn special_transition_year() {
        assert_eq!(day_of_the_programmer(1918), "26.09.1918");
    }

    #[test]
    fn normal_year() {
        assert_eq!(day_of_the_programmer(2017), "13.09.2017");
    }
}