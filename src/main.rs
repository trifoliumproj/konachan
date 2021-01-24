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

        if tags.len() == 0 {
            return Err(tonic::Status::failed_precondition(
                "Tags should not be empty.",
            ));
        }

        let mut n: Vec<String> = Vec::new();
        for tag in &tags {
            n.push(tag.name.clone());
        }

        let mut url = reqwest::Url::parse("https://konachan.com/post.json").unwrap();
        url.query_pairs_mut().append_pair("tags", &(n.join(" ")));
        let mut ret = GalleryResponse { posts: [].to_vec() };
        let resp = reqwest::blocking::get(url);
        let _ = match resp {
            Ok(res) => {
                let json = res.json::<Vec<generated::konachan::Post>>();
                let _ = match json {
                    Ok(posts) => {
                        ret.posts = posts;
                        // println!("{:#?}", posts);
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
