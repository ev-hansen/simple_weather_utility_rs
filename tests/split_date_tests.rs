use simple_weather_utility_rs;

#[test]
fn test_year(){
    assert_eq!(2024, simple_weather_utility_rs::split_date("2024-01-01").year)
}

#[test]
fn test_month(){
    assert_eq!(01, simple_weather_utility_rs::split_date("2024-01-01").month)
}

#[test]
fn test_day(){
    assert_eq!(01, simple_weather_utility_rs::split_date("2024-01-01").day)
}