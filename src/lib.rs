pub struct Date{
    pub year: i32,
    pub month: i16,
    pub day: i16,
    pub is_leapyear: bool,
    pub day_of_year: i16
}

pub fn check_if_leap_year(year: i32) -> bool{
    (((year % 100) != 0) && ((year % 4) == 0)) || ((year % 400) == 0)
}

pub fn calculate_day_of_year(is_leapyear: bool, month: i16, day:i16) -> i16{
    let total_before: Vec<i16> = if is_leapyear == false {
        vec![0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334]
    }
    else {
        vec![0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335]
    };
    total_before[(month-1) as usize] + day
}

pub fn split_date(date: &str) -> Date{
    
    let given_data: Vec<&str> = date.split("-").collect();
    let this_year: i32 = given_data[0].parse::<i32>().unwrap();
    let this_month: i16 =  given_data[1].parse::<i16>().unwrap();
    let this_day: i16 =  given_data[2].parse::<i16>().unwrap();
    let this_leapyear_status: bool = check_if_leap_year(this_year);
    let this_day_of_year: i16 = calculate_day_of_year(this_leapyear_status, this_month, this_day);
    
    let date_data: Date = Date{
        year: this_year,
        month: this_month,
        day: this_day,
        is_leapyear: this_leapyear_status,
        day_of_year: this_day_of_year
    };
    date_data
}

