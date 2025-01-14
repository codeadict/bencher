use bencher_json::JsonEndpoint;
use dropshot::{endpoint, HttpError, RequestContext};

use crate::{
    context::ApiContext,
    endpoints::{
        endpoint::{pub_response_ok, response_ok, ResponseOk},
        system::server::Resource,
        Endpoint, Method,
    },
    model::user::auth::AuthUser,
    util::cors::{get_cors, CorsResponse},
    ApiError,
};

const ENDPOINT_RESOURCE: Resource = Resource::Endpoint;

#[allow(clippy::unused_async)]
#[endpoint {
        method = OPTIONS,
        path =  "/v0/server/endpoint",
        tags = ["server", "endpoint"]
    }]
pub async fn server_endpoint_options(
    _rqctx: RequestContext<ApiContext>,
) -> Result<CorsResponse, HttpError> {
    Ok(get_cors::<ApiContext>())
}

#[endpoint {
        method = GET,
        path =  "/v0/server/endpoint",
        tags = ["server", "endpoint"]
    }]
pub async fn server_endpoint_get(
    rqctx: RequestContext<ApiContext>,
) -> Result<ResponseOk<JsonEndpoint>, HttpError> {
    let auth_user = AuthUser::new(&rqctx).await.ok();
    let endpoint = Endpoint::new(ENDPOINT_RESOURCE, Method::GetOne);

    let context = rqctx.context();
    let json = get_one_inner(context).await.map_err(|e| endpoint.err(e))?;

    if auth_user.is_some() {
        response_ok!(endpoint, json)
    } else {
        pub_response_ok!(endpoint, json)
    }
}

async fn get_one_inner(context: &ApiContext) -> Result<JsonEndpoint, ApiError> {
    Ok(JsonEndpoint {
        endpoint: context.endpoint.clone().into(),
    })
}
