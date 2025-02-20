// Copyright 2022 The Engula Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod cluster;
mod health;
mod job;
mod metadata;
mod metrics;
mod service;

pub use self::service::AdminService;
use self::service::Router;
use crate::Server;

pub fn make_admin_service(server: Server) -> AdminService {
    let router = Router::empty()
        .route(
            "/metrics",
            self::metrics::MetricsHandle::new(server.to_owned()),
        )
        .route("/job", self::job::JobHandle::new(server.to_owned()))
        .route(
            "/metadata",
            self::metadata::MetadataHandle::new(server.to_owned()),
        )
        .route("/health", self::health::HealthHandle)
        .route(
            "/cordon",
            self::cluster::CordonHandle::new(server.to_owned()),
        )
        .route(
            "/uncordon",
            self::cluster::UncordonHandle::new(server.to_owned()),
        )
        .route("/drain", self::cluster::DrainHandle::new(server.to_owned()))
        .route("/node_status", self::cluster::StatusHandle::new(server));
    let api = Router::nest("/admin", router);
    AdminService::new(api)
}
