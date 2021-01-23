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
        let _tags = req.tags;
        let resp =
            reqwest::blocking::get("https://konachan.com/post.json?tags=mona_%28genshin_impact%29");
        let _ = match resp {
            Ok(res) => {
                let json = res.json::<Vec<generated::konachan::Post>>();
                let _ = match json {
                    Ok(posts) => {
                        println!("{:#?}", posts);
                    }
                    Err(error) => {
                        println!("{:#?}", error);
                    }
                };
            }
            Err(error) => {
                println!("{:#?}", error);
            }
        };
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
