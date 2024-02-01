use simple_weather_utility_rs;

#[test]
fn test_day_1_leapyear(){
    assert_eq!(1, simple_weather_utility_rs::calculate_day_of_year(true, 01, 01))
}

#[test]
fn test_day_366_leapyear(){
    assert_eq!(366, simple_weather_utility_rs::calculate_day_of_year(true, 12, 31))
}

#[test]
fn test_day_1_nonleapyear(){
    assert_eq!(1, simple_weather_utility_rs::calculate_day_of_year(false, 1, 1))
}

#[test]
fn test_day_365_nonleapyear(){
    assert_eq!(365, simple_weather_utility_rs::calculate_day_of_year(false, 12, 31))
}