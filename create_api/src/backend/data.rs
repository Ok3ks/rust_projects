use chrono:: {DateTime, Utc};
use rand:: {Rng, thread_rng};
use serde::Serialize;
use anyhow::Result;



#[derive(Debug, Serialize, Clone)]
pub struct Trip {
    tpep_pickup_datetime: DateTime<Utc>,
    tpep_dropoff_datetime: DateTime<Utc>,
    trip_distance: f64,
    fare_amount: f64,
}

pub async fn get_fake_trips(_from_ms: i64, n_results: i64) -> Result<Vec<Trip>> {

    let mut rng = thread_rng();
    let trips = (0..n_results).map(|_| {
        let random_seconds = rng.gen_range(0..60);
        let pickup_time = DateTime::<Utc>::from_timestamp(
            _from_ms/1000 + random_seconds, 0
        ).unwrap();
        let dropoff_time = DateTime::<Utc>::from_timestamp(
            _from_ms / 1000 + random_seconds + rng.gen_range(300..3600), 0
        ).unwrap();

        Trip {
            tpep_pickup_datetime: pickup_time,
            tpep_dropoff_datetime: dropoff_time,
            trip_distance: rng.gen_range(0.5..20.0),
            fare_amount: rng.gen_range(2.5..100.0),
        }
    }).collect();
    
    Ok(trips)
}

pub async fn get_trips(_from_ms: i64, n_results: i64) -> Result<Vec<Trip>> {
    let (year, month) = get_year_and_month(from_ms);
    info!("Extracted year: {}, month: {}", year, month);

    info!("Downloading parquet file for year: {}, month: {}");
    let file_path = download_parquet_file(year, month).await?;

    //Get the trips from the file
    let trips = get_trips_from_file(&file_path, _from_ms, n_results)?;

    info!("Returning {} trips", trips.len());
    Ok(trips)
}

pub fn get_year_and_month(_from_ms:i64) -> (i32, i32) {
    let datetime = DateTime::<Utc>::from_timestamp(_from_ms/ 1000, 0).unwrap();
    (datetime.year(), datetime.month() as i32)
}

pub async fn download_parquet_file(year: i32, month:i32) -> Result<String> {
    
    let url = format!(
        "https://d37ci6vzurychx.cloudfront.net/trip-data/yellow_tripdata_{}-{:02}.parquet",
        year,
        month
    );

    let file_path = format!("yellow_tripdata_{}-{:02}.parquet", year, month);
    
    //Check if the file exists before trying to download
    if tokio::fs::try_exists(&file_path).await? {
        info!("File {} already exists", &file_path);
        return Ok(file_path);
    }

    info!("Downloading file from {}", &url);
    let response = reqwest::get(&url).await?;
    if response.status().is_success() {
        let byte = response.bytes().await?;

        tokio::fs::write(&file_path, bytes).await?
        info!("File {} downloaded successfully", &file_path);
    } else {
        error!("Failed to download file");
    }
    Ok(file_path)
}
