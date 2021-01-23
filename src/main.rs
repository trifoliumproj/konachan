mod generated;

use generated::konachan::{GalleryRequest, GalleryResponse};

use generated::konachan::api_server;

#[derive(Debug, Default)]
pub struct APIImplementation {}

#[tonic::async_trait]
impl api_server::Api for APIImplementation {
    async fn gallery(
        &self,
        request: tonic::Request<GalleryRequest>,
    ) -> Result<tonic::Response<GalleryResponse>, tonic::Status> {
        let req = request.into_inner();
        let tags = req.tags;
        let ret = GalleryResponse { posts: [].to_vec() };
        Ok(tonic::Response::new(ret))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let server = APIImplementation::default();
    tonic::transport::Server::builder()
        .add_service(api_server::ApiServer::new(server))
        .serve(addr)
        .await?;
    Ok(())
}
