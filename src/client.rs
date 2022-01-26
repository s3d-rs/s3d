use aws_endpoint::AwsEndpointStage;
use aws_http::{
    auth::CredentialsStage, recursion_detection::RecursionDetectionStage,
    user_agent::UserAgentStage,
};
use aws_sig_auth::{middleware::SigV4SigningStage, signer::SigV4Signer};
use aws_smithy_http::{endpoint::Endpoint, middleware::MapRequest, operation::Request};
use aws_smithy_http_tower::map_request::{AsyncMapRequestLayer, MapRequestLayer};
use codegen_client_s3::{Builder, Client, Config};
use hyper::Uri;
use std::fmt::Debug;
use tower::layer::util::{Identity, Stack};
use tower::ServiceBuilder;

pub type S3Client = Client<aws_smithy_client::erase::DynConnector, ClientMiddleware>;

pub fn new_s3d_client() -> S3Client {
    new_s3_client(ClientOptions {
        endpoint: "http://localhost:33333".to_string(),
    })
}

pub fn new_s3_client(
    options: ClientOptions,
) -> S3Client {
    S3Client::with_config(
        Builder::dyn_https()
            .middleware(ClientMiddleware { options })
            .build(),
        Config::builder().build(),
    )
}

#[derive(Debug, Default, Clone)]
pub struct ClientOptions {
    pub endpoint: String,
    // pub access_key: String,
    // pub secret_key: String,
}

#[derive(Debug, Default, Clone)]
pub struct ClientMiddleware {
    pub options: ClientOptions,
}

pub type ClientMiddlewareStack = Stack<MapRequestLayer<InitRequestStage>, Identity>;
// Stack<MapRequestLayer<RecursionDetectionStage>,
//     Stack<MapRequestLayer<SigV4SigningStage>,
//         Stack<AsyncMapRequestLayer<CredentialsStage>,
//             Stack<MapRequestLayer<UserAgentStage>,
//                 Stack<MapRequestLayer<AwsEndpointStage>,
//                     Identity
//                 >,
//             >,
//         >,
//     >,
// >;

impl<S> tower::Layer<S> for ClientMiddleware {
    type Service = <ClientMiddlewareStack as tower::Layer<S>>::Service;

    fn layer(&self, inner: S) -> Self::Service {
        let init_request = MapRequestLayer::for_mapper(InitRequestStage {
            options: self.options.clone(),
        });

        // let endpoint_resolver = MapRequestLayer::for_mapper(AwsEndpointStage);
        // let user_agent = MapRequestLayer::for_mapper(UserAgentStage::new());
        // let credential_provider = AsyncMapRequestLayer::for_mapper(CredentialsStage::new());
        // let signer = MapRequestLayer::for_mapper(SigV4SigningStage::new(SigV4Signer::new()));
        // let recursion_detection = MapRequestLayer::for_mapper(RecursionDetectionStage::new());

        ServiceBuilder::new()
            .layer(init_request)
            // .layer(endpoint_resolver)
            // .layer(user_agent)
            // .layer(credential_provider)
            // .layer(signer)
            // .layer(recursion_detection)
            .service(inner)
    }
}

#[derive(Clone, Debug)]
pub struct InitRequestStage {
    pub options: ClientOptions,
}

impl MapRequest for InitRequestStage {
    type Error = anyhow::Error;

    fn apply(&self, request: Request) -> Result<Request, Self::Error> {
        request.augment(|mut req, conf| {
            Endpoint::immutable(Uri::try_from(&self.options.endpoint)?)
                .set_endpoint(req.uri_mut(), None);
            Ok(req)
        })
    }
}
