use simple_weather_utility_rs;

#[test]
fn test_leap_year() {
    assert_eq!(true, simple_weather_utility_rs::check_if_leap_year(2020))
}

#[test]
fn test_non_leap_year() {
    assert_eq!(false, simple_weather_utility_rs::check_if_leap_year(2019))
}

#[test]
fn test_100_non_leapyear(){
    assert_eq!(false, simple_weather_utility_rs::check_if_leap_year(1700))
}

#[test]
fn test_400_leapyear(){
    assert_eq!(true, simple_weather_utility_rs::check_if_leap_year(1600))
}