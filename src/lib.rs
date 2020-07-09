pub mod archive;
pub mod github;
pub mod measurement;
pub mod openssl;
pub mod process;
pub mod web;

pub mod prelude {
    pub use crate::archive::decompress;
    pub use crate::github::{GitHubApi, GitHubApiError, Release};
    pub use crate::measurement::{
        benchmark_consumption, benchmark_efficiency, benchmark_endurance, benchmark_speed,
        ConsumptionBenchmarkError, ConsumptionBenchmarkRun, EfficiencyBenchmarkDef,
        EfficiencyBenchmarkFinish, EfficiencyBenchmarkRun, Endurance, EnduranceBenchmarkDef,
        EnduranceBenchmarkFinish, EnduranceBenchmarkRun, NamedProcess, ResourcesUsage, Speed,
        SpeedBenchmarkDef, SpeedBenchmarkFinish, SpeedBenchmarkRun, Thresholds, Timestamp,
    };
    pub use crate::openssl::Openssl;
    pub use crate::web::download_file;
}