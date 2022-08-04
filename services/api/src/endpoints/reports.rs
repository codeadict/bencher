use std::sync::Arc;

use bencher_json::{
    JsonNewReport,
    JsonReport,
    ResourceId,
};
use diesel::{
    expression_methods::BoolExpressionMethods,
    JoinOnDsl,
    QueryDsl,
    RunQueryDsl,
};
use dropshot::{
    endpoint,
    HttpError,
    HttpResponseAccepted,
    HttpResponseHeaders,
    HttpResponseOk,
    Path,
    RequestContext,
    TypedBody,
};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    db::{
        model::{
            project::QueryProject,
            report::{
                InsertReport,
                QueryReport,
            },
        },
        schema,
    },
    diesel::ExpressionMethods,
    util::{
        cors::get_cors,
        headers::CorsHeaders,
        http_error,
        Context,
    },
};

#[derive(Deserialize, JsonSchema)]
pub struct GetLsParams {
    pub project: ResourceId,
}

#[endpoint {
    method = OPTIONS,
    path =  "/v0/projects/{project}/reports",
    tags = ["projects", "reports"]
}]
pub async fn get_ls_options(
    _rqctx: Arc<RequestContext<Context>>,
    _path_params: Path<GetLsParams>,
) -> Result<HttpResponseHeaders<HttpResponseOk<String>>, HttpError> {
    Ok(get_cors::<Context>())
}

#[endpoint {
    method = GET,
    path =  "/v0/projects/{project}/reports",
    tags = ["projects", "reports"]
}]
pub async fn get_ls(
    rqctx: Arc<RequestContext<Context>>,
    path_params: Path<GetLsParams>,
) -> Result<HttpResponseHeaders<HttpResponseOk<Vec<JsonReport>>, CorsHeaders>, HttpError> {
    let db_connection = rqctx.context();
    let path_params = path_params.into_inner();

    let conn = db_connection.lock().await;
    let query_project = QueryProject::from_resource_id(&*conn, &path_params.project)?;
    let json: Vec<JsonReport> = schema::report::table
        .left_join(schema::version::table.on(schema::report::version_id.eq(schema::version::id)))
        .left_join(schema::branch::table.on(schema::version::branch_id.eq(schema::branch::id)))
        .filter(schema::branch::project_id.eq(&query_project.id))
        .select((
            schema::report::id,
            schema::report::uuid,
            schema::report::user_id,
            schema::report::version_id,
            schema::report::testbed_id,
            schema::report::adapter_id,
            schema::report::start_time,
            schema::report::end_time,
        ))
        // .order(schema::report::start_time)
        // .desc()
        .load::<QueryReport>(&*conn)
        .map_err(|_| http_error!("Failed to get reports."))?
        .into_iter()
        .filter_map(|query| query.to_json(&*conn).ok())
        .collect();

    Ok(HttpResponseHeaders::new(
        HttpResponseOk(json),
        CorsHeaders::new_pub("GET".into()),
    ))
}

// #[endpoint {
//     method = OPTIONS,
//     path =  "/v0/reports",
//     tags = ["reports"]
// }]
// pub async fn post_options(
//     _rqctx: Arc<RequestContext<Context>>,
// ) -> Result<HttpResponseHeaders<HttpResponseOk<String>>, HttpError> {
//     Ok(get_cors::<Context>())
// }

// #[endpoint {
//     method = POST,
//     path = "/v0/reports",
//     tags = ["reports"]
// }]
// pub async fn post(
//     rqctx: Arc<RequestContext<Context>>,
//     body: TypedBody<JsonNewReport>,
// ) -> Result<HttpResponseHeaders<HttpResponseAccepted<JsonReport>, CorsHeaders>, HttpError> {
//     let db_connection = rqctx.context();
//     let json_report = body.into_inner();

//     let conn = db_connection.lock().await;
//     let insert_report = InsertReport::from_json(&*conn, json_report)?;
//     diesel::insert_into(schema::report::table)
//         .values(&insert_report)
//         .execute(&*conn)
//         .map_err(|_| http_error!("Failed to create report."))?;

//     let query_report = schema::report::table
//         .filter(schema::report::uuid.eq(&insert_report.uuid))
//         .first::<QueryReport>(&*conn)
//         .map_err(|_| http_error!("Failed to create report."))?;
//     let json = query_report.to_json(&*conn)?;

//     Ok(HttpResponseHeaders::new(
//         HttpResponseAccepted(json),
//         CorsHeaders::new_auth("POST".into()),
//     ))
// }

// #[derive(Deserialize, JsonSchema)]
// pub struct GetOneParams {
//     pub project:        ResourceId,
//     pub branch:         ResourceId,
//     pub version_number: ResourceId,
// }
// #[endpoint {
//     method = OPTIONS,
//     path =  "/v0/projects/{project}/branches/{branch}/{version_number}",
//     tags = ["projects", "branches", "reports"]
// }]

// pub async fn get_one_options(
//     _rqctx: Arc<RequestContext<Context>>,
//     _path_params: Path<GetOneParams>,
// ) -> Result<HttpResponseHeaders<HttpResponseOk<String>>, HttpError> {
//     Ok(get_cors::<Context>())
// }

// #[endpoint {
//     method = GET,
//     path =  "/v0/projects/{project}/branches/{branch}/{version_number}",
//     tags = ["projects", "branches",  "reports"]
// }]
// pub async fn get_one(
//     rqctx: Arc<RequestContext<Context>>,
//     path_params: Path<GetOneParams>,
// ) -> Result<HttpResponseHeaders<HttpResponseOk<JsonReport>, CorsHeaders>,
// HttpError> {     let db_connection = rqctx.context();
//     let path_params = path_params.into_inner();
//     let resource_id = path_params.report.as_str();

//     let conn = db_connection.lock().await;
//     let project = QueryProject::from_resource_id(&*conn,
// &path_params.project)?;     let query = if let Ok(query) =
// schema::report::table         .filter(
//             schema::report::project_id.eq(project.id).and(
//                 schema::report::version_id
//                     .eq(version_id)
//                     .or(schema::report::uuid.eq(resource_id)),
//             ),
//         )
//         .first::<QueryReport>(&*conn)
//     {
//         Ok(query)
//     } else {
//         Err(http_error!("Failed to get report."))
//     }?;
//     let json = query.to_json(&*conn)?;

//     Ok(HttpResponseHeaders::new(
//         HttpResponseOk(json),
//         CorsHeaders::new_pub("GET".into()),
//     ))
// }
