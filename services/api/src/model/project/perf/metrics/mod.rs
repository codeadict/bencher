use bencher_json::project::report::new::{JsonBenchmarks, JsonMetrics};
use diesel::{RunQueryDsl, SqliteConnection};
use dropshot::HttpError;

use crate::{
    error::api_error,
    model::project::perf::{InsertPerf, QueryPerf},
    schema,
};

pub mod detectors;

use self::detectors::Detectors;

use super::metric::InsertMetric;

pub struct Metrics {
    pub report_id: i32,
    pub detectors: Detectors,
}

impl Metrics {
    pub fn new(
        conn: &mut SqliteConnection,
        project_id: i32,
        branch_id: i32,
        testbed_id: i32,
        report_id: i32,
        benchmarks: JsonBenchmarks,
    ) -> Result<Self, HttpError> {
        Ok(Self {
            report_id,
            detectors: Detectors::new(conn, project_id, branch_id, testbed_id, benchmarks)?,
        })
    }

    pub fn insert(
        &mut self,
        conn: &mut SqliteConnection,
        iteration: usize,
        benchmark_name: String,
        json_metrics: JsonMetrics,
    ) -> Result<(), HttpError> {
        // All benchmarks should already exist
        // Get the benchmark ID from the detectors cache
        let benchmark_id = self.detectors.benchmark_id(benchmark_name)?;

        let insert_perf = InsertPerf::from_json(self.report_id, iteration, benchmark_id);
        diesel::insert_into(schema::perf::table)
            .values(&insert_perf)
            .execute(conn)
            .map_err(api_error!())?;
        let perf_id = QueryPerf::get_id(conn, &insert_perf.uuid)?;

        for (metric_kind_key, metric) in json_metrics.inner {
            // All metric kinds should already exist
            // Get the metric kind ID from the detectors cache
            let metric_kind_id = self.detectors.metric_kind_id(metric_kind_key)?;

            let insert_metric = InsertMetric::from_json(perf_id, metric_kind_id, metric);
            diesel::insert_into(schema::metric::table)
                .values(&insert_metric)
                .execute(conn)
                .map_err(api_error!())?;

            self.detectors
                .detect(conn, perf_id, benchmark_id, metric_kind_id, metric)?;
        }

        Ok(())
    }
}