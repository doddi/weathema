use serde::Deserialize;

pub struct WeatherAPI;

impl WeatherAPI {
    pub(crate) fn new() -> Self {
        Self
    }

    pub async fn get_weather(
        &self,
        location: &str,
    ) -> Result<WeatherInformation, Box<dyn std::error::Error>> {
        let id = self.get_location_id(location).await?;
        let weather = self.get_weather_data(id).await?;
        Ok(weather)
    }

    async fn get_location_id(&self, location: &str) -> Result<usize, Box<dyn std::error::Error>> {
        const ENDPOINT: &str = "https://open.live.bbc.co.uk/locator/locations?filter=international&place-types=settlement,airport,district&s={}&format=json&order=importance&a=true";

        let response: WeatherLocationResponse = reqwest::get(ENDPOINT.replace("{}", location))
            .await?
            .json()
            .await?;
        if response.response.results.total_results == 0 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No results found",
            )));
        }

        let id = response.response.results.results[0].id.parse().unwrap();
        Ok(id)
    }

    async fn get_weather_data(
        &self,
        id: usize,
    ) -> Result<WeatherInformation, Box<dyn std::error::Error>> {
        const ENDPOINT: &str =
            "https://weather-broker-cdn.api.bbci.co.uk/en/forecast/aggregated/{}";
        let response: WeatherForecastResponse =
            reqwest::get(ENDPOINT.replace("{}", id.to_string().as_str()))
                .await?
                .json()
                .await?;
        Ok(response.into())
    }
}

#[derive(Debug)]
pub(crate) struct WeatherInformation {
    pub wind_speed: f64,
    pub wind_direction: String,

    pub min_temperature: f64,
    pub max_temperature: f64,

    pub weather: String,
    pub weather_type: u8,

    pub sunrise: String,
    pub sunset: String,

    pub precipitation_chance: f64,
}

impl Into<WeatherInformation> for WeatherForecastResponse {
    fn into(self) -> WeatherInformation {
        let forecast = &self.forecasts[0];
        let summary = &forecast.summary.report;

        WeatherInformation {
            wind_speed: summary.wind_speed_kph,
            wind_direction: summary.wind_direction.clone(),
            weather: summary.weather_type_text.clone().into(),
            weather_type: summary.weather_type,
            min_temperature: summary.min_temp_c,
            max_temperature: summary.max_temp_c,
            sunrise: summary.sunrise.clone(),
            sunset: summary.sunset.clone(),
            precipitation_chance: summary.precipitation_probability_in_percent,
        }
    }
}

////////////// Weather API //////////////
#[derive(Debug, Deserialize)]
struct WeatherForecastResponse {
    forecasts: Vec<WeatherForecast>,
}

#[derive(Debug, Deserialize)]
struct WeatherForecast {
    detailed: WeatherDetailedForecast,
    summary: WeatherSummaryForecast,
}

#[derive(Debug, Deserialize)]
struct WeatherDetailedForecast {
    #[serde(rename = "issueDate")]
    issue_date: String,
    #[serde(rename = "lastUpdated")]
    last_updated: String,
    reports: Vec<WeatherDetailedReport>,
}

#[derive(Debug, Deserialize)]
struct WeatherDetailedReport {
    #[serde(rename = "weatherType")]
    weather_type: u8,
    #[serde(rename = "weatherTypeText")]
    weather_type_text: String,
}

#[derive(Debug, Deserialize)]
struct WeatherSummaryForecast {
    #[serde(rename = "issueDate")]
    issue_date: String,
    #[serde(rename = "lastUpdated")]
    last_updated: String,
    report: WeatherSummaryReport,
}

#[derive(Debug, Deserialize)]
struct WeatherSummaryReport {
    sunrise: String,
    sunset: String,
    #[serde(rename = "maxTempC")]
    max_temp_c: f64,
    #[serde(rename = "minTempC")]
    min_temp_c: f64,
    #[serde(rename = "windSpeedKph")]
    wind_speed_kph: f64,
    #[serde(rename = "windDirection")]
    wind_direction: String,
    #[serde(rename = "weatherType")]
    weather_type: u8,
    #[serde(rename = "weatherTypeText")]
    weather_type_text: String,

    #[serde(rename = "precipitationProbabilityInPercent")]
    precipitation_probability_in_percent: f64,
}

////////////// Location API //////////////
#[derive(Debug, Deserialize)]
struct WeatherLocationResponse {
    response: WeatherLocationWrappedResults,
}

#[derive(Debug, Deserialize)]
struct WeatherLocationWrappedResults {
    results: WeatherLocationResults,
}

#[derive(Debug, Deserialize)]
struct WeatherLocationResults {
    results: Vec<WeatherLocationResult>,
    #[serde(rename = "totalResults")]
    total_results: u32,
}

#[derive(Debug, Deserialize)]
struct WeatherLocationResult {
    id: String,
    name: String,
    container: String,
    country: String,
    latitude: f64,
    longitude: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test() {
        let data = r#"
        {
  "response": {
    "results": {
      "results": [
        {
          "id": "2650584",
          "name": "Dyserth",
          "container": "Denbighshire",
          "containerId": 2651385,
          "language": "en",
          "timezone": "Europe/London",
          "country": "GB",
          "latitude": 53.30032,
          "longitude": -3.41262,
          "placeType": "settlement",
          "topicId": "c8zwn5l8l6rt"
        }
      ],
      "totalResults": 1
    }
  }
}
        "#;

        let result: WeatherLocationResponse = serde_json::from_str(&data).unwrap();

        assert_eq!(result.response.results.total_results, 1);
        assert_eq!(result.response.results.results[0].id, "2650584");
        assert_eq!(result.response.results.results[0].name, "Dyserth");
        assert_eq!(result.response.results.results[0].container, "Denbighshire");
        assert_eq!(result.response.results.results[0].country, "GB");
        assert_eq!(result.response.results.results[0].latitude, 53.30032);
        assert_eq!(result.response.results.results[0].longitude, -3.41262);
    }

    #[test]
    fn deserialize_weather_location_response() {
        let data = json!({
            "response": {
                "results": {
                    "totalResults": 1,
                    "results": [
                        {
                            "id": "12345",
                            "name": "London",
                            "container": "England",
                            "country": "UK",
                            "latitude": 51.5074,
                            "longitude": -0.1278
                        }
                    ]
                }
            }
        });

        let json_str = data.to_string();
        let result: WeatherLocationResponse = serde_json::from_str(&json_str).unwrap();

        assert_eq!(result.response.results.total_results, 1);
        assert_eq!(result.response.results.results[0].id, "12345");
        assert_eq!(result.response.results.results[0].name, "London");
        assert_eq!(result.response.results.results[0].container, "England");
        assert_eq!(result.response.results.results[0].country, "UK");
        assert_eq!(result.response.results.results[0].latitude, 51.5074);
        assert_eq!(result.response.results.results[0].longitude, -0.1278);
    }

    #[test]
    fn decode_weather_forecast() {
        let file = std::fs::File::open("src/test_data/weather_forecast.json").unwrap();
        let result: WeatherForecastResponse = serde_json::from_reader(file).unwrap();

        assert_eq!(result.forecasts.len(), 14);
        assert_eq!(result.forecasts[0].summary.report.weather_type, 3);
    }
}
