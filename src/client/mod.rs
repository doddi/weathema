use serde::Deserialize;

pub struct WeatherAPI;

impl WeatherAPI {
    pub(crate) fn new() -> Self {
        Self
    }

    pub async fn get_weather(
        &self,
        location: &str,
    ) -> Result<WeatherForecastResponse, Box<dyn std::error::Error>> {
        let id = self.get_location_id(location).await?;
        let response = self.get_weather_data(id).await?;
        Ok(response)
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
    ) -> Result<WeatherForecastResponse, Box<dyn std::error::Error>> {
        const ENDPOINT: &str =
            "https://weather-broker-cdn.api.bbci.co.uk/en/forecast/aggregated/{}";
        let response: WeatherForecastResponse =
            reqwest::get(ENDPOINT.replace("{}", id.to_string().as_str()))
                .await?
                .json()
                .await?;
        Ok(response)
    }
}

pub(crate) enum WeathemaComponentMessaging {
    ForecastWaiting,
    ForecastReceived(WeatherForecastResponse),
    ForecastError(String),
}

////////////// Weather API //////////////
#[derive(Debug, Deserialize)]
pub(crate) struct WeatherForecastResponse {
    pub forecasts: Vec<WeatherForecast>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(crate) struct WeatherForecast {
    pub detailed: WeatherDetailedForecast,
    pub summary: WeatherSummaryForecast,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(crate) struct WeatherDetailedForecast {
    #[serde(rename = "issueDate")]
    pub issue_date: String,
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
    pub reports: Vec<WeatherDetailedReport>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(crate) struct WeatherDetailedReport {
    #[serde(rename = "weatherType")]
    pub weather_type: u8,
    #[serde(rename = "weatherTypeText")]
    pub weather_type_text: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(crate) struct WeatherSummaryForecast {
    #[serde(rename = "issueDate")]
    pub issue_date: String,
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
    pub report: WeatherSummaryReport,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(crate) struct WeatherSummaryReport {
    pub sunrise: String,
    pub sunset: String,
    #[serde(rename = "maxTempC")]
    pub max_temp_c: f64,
    #[serde(rename = "minTempC")]
    pub min_temp_c: f64,
    #[serde(rename = "windSpeedKph")]
    pub wind_speed_kph: f64,
    #[serde(rename = "windDirection")]
    pub wind_direction: String,
    #[serde(rename = "weatherType")]
    pub weather_type: u8,
    #[serde(rename = "weatherTypeText")]
    pub weather_type_text: String,

    #[serde(rename = "precipitationProbabilityInPercent")]
    pub precipitation_probability_in_percent: f64,
}

////////////// Location API //////////////
#[derive(Debug, Deserialize)]
pub(crate) struct WeatherLocationResponse {
    pub response: WeatherLocationWrappedResults,
}

#[derive(Debug, Deserialize)]
pub(crate) struct WeatherLocationWrappedResults {
    pub results: WeatherLocationResults,
}

#[derive(Debug, Deserialize)]
pub(crate) struct WeatherLocationResults {
    pub results: Vec<WeatherLocationResult>,
    #[serde(rename = "totalResults")]
    pub total_results: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(crate) struct WeatherLocationResult {
    pub id: String,
    pub name: String,
    pub container: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
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
