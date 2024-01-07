extern crate core;

use crate::benchmark::web::benchmark_web;
use crate::utils::docker_stats;

mod benchmark {
    pub mod computation;
    pub mod web;
    pub mod benchmark;
}

mod utils {
    pub mod docker_runner;
    pub mod docker_stats;
    pub mod http_load_tester;
    pub mod meta_data_parser;
    pub mod result_writer;
    pub mod serialization;
    pub mod version;
}

const CONTAINER_NAME: &str = "benchmark";

fn main() {
    let mut reader = docker_stats::DockerStatsReader::new(CONTAINER_NAME);
    reader.run();

    benchmark_web(
        "benchmark/web/java/springboot-3.2-mvc-temurin-21",
        &mut reader,
    );

    reader.stop();
    reader.dispose();
}
